# 13D. Rust-WGSL 연결과 Shader Hot Reload

## 학습 목표

- Rust의 `Material2d`가 WGSL 파일을 선택하고 GPU binding을 구성하는 과정을 추적할 수 있다.
- 앱을 종료하지 않고 WGSL 변경을 반영할 수 있다.
- shader hot reload와 Rust 코드 재컴파일을 구분할 수 있다.
- shader 문법 오류가 발생했을 때 로그를 읽고 마지막 정상 상태로 복구할 수 있다.

## 이 내용으로 만들 수 있는 것

- 게임을 재시작하지 않고 WGSL을 수정하며 색·왜곡 효과를 반복 조정할 수 있습니다.
- shader 컴파일 실패와 복구 상태를 화면에 표시하는 개발 도구를 만들 수 있습니다.

## 이번에 만들 결과물

13C 예제를 실행한 채 WGSL 파일의 흔들림 배율과 피격색을 수정합니다. 저장 후 새 shader가 자동으로 컴파일되어 화면에 반영되는 과정을 확인합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin material_2d
```

실행 중 다음 파일을 편집합니다.

```text
examples/part2/space_survivor/assets/shaders/13c_sprite_effect.wgsl
```

## 핵심 개념

### Rust에서 WGSL까지 연결되는 순서

첫 번째 연결은 shader asset 경로입니다.

```rust
const SHADER_PATH: &str = "shaders/13c_sprite_effect.wgsl";

impl Material2d for SpriteEffectMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}
```

이 경로는 `AssetPlugin.file_path`가 정한 asset 루트를 기준으로 합니다. 이번 프로젝트는 패키지의 `assets` 폴더를 루트로 사용하므로 WGSL 전체 경로를 Rust 코드에 적지 않습니다.

두 번째 연결은 Material 등록입니다.

```rust
app.add_plugins(Material2dPlugin::<SpriteEffectMaterial>::default());
```

이 Plugin이 없다면 Bevy는 `MeshMaterial2d<SpriteEffectMaterial>`을 위한 2D 렌더 파이프라인을 만들지 않습니다.

세 번째 연결은 bind group입니다.

| Rust | WGSL | 역할 |
|---|---|---|
| `#[uniform(0)] effect` | `@binding(0) effect` | 시간·흔들림·점멸 |
| `#[texture(1)] color_texture` | `@binding(1) color_texture` | PNG 픽셀 |
| `#[sampler(2)]` | `@binding(2) color_sampler` | 필터링 방식 |
| `#[uniform(3)] uv_rect` | `@binding(3) uv_rect` | atlas 프레임 영역 |

번호뿐 아니라 Rust와 WGSL의 데이터 배치도 호환되어야 합니다. binding을 바꾸면 양쪽을 함께 수정해야 합니다.

### 파일 감시 활성화

이번 예제는 개발 실습에서 동작을 분명히 하기 위해 파일 감시를 명시적으로 켭니다.

```rust
AssetPlugin {
    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
    watch_for_changes_override: Some(true),
    ..default()
}
```

Bevy 0.19의 기본 feature에는 `file_watcher`가 포함되지 않습니다. `watch_for_changes_override`만 켜면 watcher를 만들 수 없다는 경고가 나오므로 Cargo 의존성에도 feature를 명시해야 합니다.

```toml
[dependencies]
bevy = { workspace = true, features = ["file_watcher"] }
```

이 교재에서는 hot reload를 사용하는 `space_survivor` 패키지에만 feature를 추가합니다. workspace의 모든 예제에 불필요한 파일 감시 의존성을 전파하지 않기 위해서입니다.

### 무엇이 hot reload되는가

| 변경 파일 | 실행 중 반영 | 이유 |
|---|---:|---|
| `.wgsl` | 가능 | Shader는 Asset이다 |
| `.png`, `.ron` | 가능 | AssetServer가 관리하는 Asset이다 |
| `.rs` | 불가능 | 실행 파일에 컴파일된 코드다 |
| `Cargo.toml` | 불가능 | 빌드 구성을 다시 적용해야 한다 |

Rust를 “스크립트”라고 부르는 경우가 있지만 기본 Bevy 프로젝트에서 Rust는 런타임 스크립트가 아닙니다. Rust 변경에는 재컴파일과 프로세스 재시작이 필요합니다.

`cargo-watch` 같은 외부 도구는 파일 변경 시 `cargo run`을 다시 실행해 이 과정을 자동화할 수 있습니다. 이것은 실행 중인 Rust 코드를 교체하는 hot reload가 아니라 기존 프로세스를 종료하고 새 실행 파일을 시작하는 방식입니다.

### 배포 환경

파일 감시는 개발 PC의 파일 시스템을 전제로 합니다. 배포 빌드와 브라우저의 WASM 환경에서는 로컬 WGSL을 편집해 즉시 반영하는 개발 흐름을 기대하면 안 됩니다. 배포할 때는 검증된 shader와 asset을 빌드 결과에 함께 포함합니다.

## 샘플 코드

WGSL에는 실습 중 안전하게 바꿀 두 상수가 있습니다.

```wgsl
const WOBBLE_SCALE: f32 = 50.0;
const HIT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.25, 0.18);
```

앱을 실행한 채 다음처럼 바꾸고 저장합니다.

```wgsl
const WOBBLE_SCALE: f32 = 10.0;
const HIT_COLOR: vec3<f32> = vec3<f32>(0.2, 1.0, 0.35);
```

Rust 코드를 다시 컴파일하지 않아도 흔들림이 작아지고 `H` 피격색이 녹색으로 변해야 합니다.

## 코드 설명

1. 파일 watcher가 WGSL 변경을 감지합니다.
2. `AssetServer`가 Shader asset을 다시 읽습니다.
3. Bevy 렌더러가 영향을 받는 pipeline을 다시 준비합니다.
4. 성공하면 다음 프레임부터 새 shader가 사용됩니다.
5. 실패하면 터미널에 WGSL 위치와 원인이 출력됩니다.

변경 직후 한두 프레임 동안 이전 결과가 보일 수 있습니다. 파일 감지, asset reload, pipeline 재생성이 순서대로 처리되기 때문입니다.

### 오류와 복구 실습

`WOBBLE_SCALE` 선언의 세미콜론을 잠시 지우고 저장합니다.

```wgsl
const WOBBLE_SCALE: f32 = 10.0
```

터미널에서 shader parse 오류와 파일 위치를 확인합니다. 그다음 세미콜론을 복원하고 저장하세요. 앱을 재시작하지 않고 정상 렌더링으로 돌아와야 합니다.

Material binding 구조를 바꾸는 실습은 더 조심해야 합니다. Rust 쪽 구조체와 WGSL을 동시에 바꿔야 하므로 Rust 재컴파일이 필요하며 단순 shader hot reload만으로 완료되지 않습니다.

## 실습 과제

1. `WOBBLE_SCALE`을 `0.0`, `10.0`, `80.0`으로 바꾸고 저장하세요.
2. `HIT_COLOR`를 녹색과 흰색으로 바꾸고 `H`를 눌러 확인하세요.
3. 의도적인 WGSL 문법 오류를 만들고 로그의 파일명과 위치를 찾으세요.
4. PNG를 별도 복사본으로 교체했을 때 texture가 reload되는지 확인하세요.

## 심화 과제

shader reload 성공과 실패를 화면 UI에 표시하는 개발 전용 Plugin을 설계하세요. `AssetEvent<Shader>`를 읽되 게임 규칙이나 저장 데이터에는 개발 도구 상태가 섞이지 않게 구성하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/13d_shader_hot_reload.md)를 확인하세요.

## 다음 챕터

다음은 기존 총알과 적 생성 실습으로 돌아갑니다. 이후 3D 셰이더 챕터에서는 같은 연결 구조를 PBR Material에 적용하므로 이 챕터만 다시 참고해도 됩니다.
