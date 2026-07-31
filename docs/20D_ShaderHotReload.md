# 20D. Rust-WGSL 연결과 Shader Hot Reload

## 학습 목표

- Rust Material과 WGSL의 경로·binding 연결을 추적할 수 있다.
- WGSL 변경과 Rust 구조 변경에 필요한 reload 범위를 구분할 수 있다.
- 실행 중 별 배경·dissolve·실드 효과를 수정할 수 있다.
- shader 문법·binding 오류를 로그에서 찾고 마지막 정상 상태로 복구할 수 있다.
- hot reload를 개발 도구로만 사용하고 배포 동작과 구분할 수 있다.

## 이 내용으로 만들 수 있는 것

- 게임을 재시작하지 않고 효과의 속도·경계·파동을 조정하는 작업 흐름
- shader 파일 오류를 감지하고 복구 절차를 안내하는 개발 도구
- 아티스트가 Rust를 다시 컴파일하지 않고 WGSL 표현을 반복 조정하는 환경

## 이번에 만들 결과물

20B 또는 20C 예제를 실행한 상태에서 WGSL 파일을 수정합니다.

```bash
cargo run -p space_survivor --bin procedural_background
cargo run -p space_survivor --bin shader_effects
```

수정 대상:

```text
examples/part2/space_survivor/assets/shaders/
├── 20b_starfield.wgsl
├── 20c_dissolve.wgsl
└── 20c_shield.wgsl
```

파일을 저장하면 Bevy AssetServer가 변경을 감지하고 관련 render pipeline을 다시 준비합니다. 성공하면 다음 프레임부터 결과가 바뀌며, 실패하면 터미널에 shader 파일과 오류 위치가 표시됩니다.

## 핵심 개념

### 첫 번째 연결: shader 경로

Rust Material이 사용할 WGSL을 반환합니다.

```rust
impl Material2d for DissolveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/20c_dissolve.wgsl".into()
    }
}
```

경로는 `AssetPlugin.file_path`로 설정한 assets 루트를 기준으로 합니다. 파일명만 바꾸고 Rust 상수를 수정하지 않으면 로드 실패가 발생합니다.

### 두 번째 연결: Material plugin

```rust
app.add_plugins(Material2dPlugin::<DissolveMaterial>::default());
```

이 Plugin이 Material 타입의 extraction, GPU 준비, specialization을 등록합니다. 단순히 WGSL 파일이 assets 폴더에 있다고 자동으로 사용되는 것은 아닙니다.

### 세 번째 연결: binding 번호와 타입

Rust:

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DissolveMaterial {
    #[uniform(0)]
    effect: Vec4,
}
```

WGSL:

```wgsl
@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> effect: vec4<f32>;
```

binding 번호와 GPU 표현 타입이 일치해야 합니다. WGSL 계산식이나 상수만 바꾸면 shader hot reload로 충분하지만, binding을 추가하거나 Rust 필드 타입을 바꾸면 Rust 코드도 다시 컴파일해야 합니다.

### 파일 감시 활성화

Space Survivor 패키지는 Bevy의 `file_watcher` feature를 사용합니다.

```toml
bevy = { workspace = true, features = ["file_watcher"] }
```

AssetServer는 파일 변경 알림을 받은 뒤 Shader asset과 그 의존성을 다시 읽습니다. 이는 “파일 변경 감지”이며 GPU pipeline 생성 성공을 뜻하지는 않습니다.

### 안전하게 수정하는 순서

한 번에 한 값만 바꾸고 저장합니다.

1. `20b_starfield.wgsl`의 가까운 별 색상 계수 변경
2. 화면 반영 확인
3. `20c_dissolve.wgsl`의 `glowing_edge` 변경
4. 적을 다시 맞혀 새 효과 확인
5. `20c_shield.wgsl`의 impact 진폭 변경
6. `H`로 반복 확인

WGSL 문법 오류를 만들었다면 터미널의 첫 번째 shader 오류부터 읽습니다. 오류를 고치고 다시 저장하면 앱을 재시작하지 않고 정상 pipeline으로 복구되어야 합니다.

### 무엇이 자동 반영되는가

| 변경 | hot reload | 이유 |
|---|---:|---|
| WGSL 숫자 상수 | 가능 | Shader asset만 변경 |
| WGSL 함수 계산식 | 가능 | 같은 binding 계약 유지 |
| Rust의 Timer 지속 시간 | 불가 | 실행 코드 재컴파일 필요 |
| Rust Material 필드 추가 | 불가 | bind group layout 변경 |
| texture 파일 교체 | 가능 | Image asset reload |
| Cargo feature 변경 | 불가 | 실행 파일 재빌드 필요 |

### Shader asset event의 한계

`AssetEvent<Shader>`로 파일이 다시 로드된 사실은 알 수 있지만 GPU pipeline 컴파일 성공을 확정할 수는 없습니다. 개발 UI에는 “파일 변경 감지” 또는 “Shader asset 재로드”라고 표시하고, 성공을 단정하지 마세요. 실제 pipeline 오류는 렌더 로그도 함께 확인해야 합니다.

## 샘플 코드

개발 중에만 파일 감시를 명시적으로 켜고 싶다면 다음처럼 설정할 수 있습니다.

```rust
DefaultPlugins.set(AssetPlugin {
    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
    watch_for_changes_override: Some(true),
    ..default()
})
```

다만 현재 패키지는 Cargo feature로 watcher를 활성화했으므로 운영체제 watcher를 사용할 수 있습니다. watcher가 없는 플랫폼이나 배포 환경에서는 이 개발 흐름에 의존하면 안 됩니다.

20B·20C의 전체 Rust 및 WGSL 코드는 [Part 2 전체 코드](source/part2.md)에서 함께 확인할 수 있습니다.

## 코드 설명

- Rust `ShaderRef`는 shader asset의 위치를 선택합니다.
- `AsBindGroup`은 Rust 값을 GPU bind group으로 변환합니다.
- WGSL `@binding`은 같은 슬롯에서 값을 읽습니다.
- WGSL만 수정하면 기존 Rust World와 게임 진행 상태를 유지한 채 표현을 바꿀 수 있습니다.
- binding 계약이 바뀌면 Rust 구조와 pipeline layout이 함께 바뀌므로 재컴파일이 필요합니다.
- hot reload는 빠른 반복을 위한 개발 기능이지 저장 시스템이나 사용자 mod 기능이 아닙니다.

## 실습 과제

1. 별의 가까운 레이어 색을 보라색으로 바꾸고 저장 후 즉시 반영되는지 확인하세요.
2. dissolve 발광 경계의 색과 배율을 각각 변경하세요.
3. 실드 vertex 파동 진폭을 두 배로 바꾸고 `H`로 비교하세요.
4. 세 WGSL 중 하나에 세미콜론을 제거해 오류 위치를 찾은 뒤 복구하세요.

## 심화 과제

Shader asset 변경·로드 실패 Message를 읽어 화면에 마지막 변경 시각과 파일 경로를 표시하는 개발 전용 Plugin을 만드세요. GPU pipeline 성공 여부는 알 수 없으므로 상태 문구가 사실보다 강하게 단정하지 않도록 설계하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/20d_shader_hot_reload.md)를 확인하세요.

## 다음 챕터

Part 2의 게임 제작과 2D 셰이더 보강 과정이 끝났습니다. 다음 Part 3에서는 같은 ECS와 UI를 게임이 아닌 GUI 애플리케이션에 적용합니다. 대량 GPU 파티클은 3D 렌더링 개념을 익힌 뒤 Part 4에서 다룹니다.
