# 13D. Shader Hot Reload 과제 해설

[본문으로 돌아가기](../../13D_ShaderHotReload.md#실습-과제)

## P2-C13D-P1 · WOBBLE_SCALE 변경

앱을 실행한 상태에서 `0`, `10`, `80`을 한 번에 하나씩 저장합니다.

### 확인 기준

- `cargo`가 Rust를 다시 컴파일하지 않는다.
- 파일 저장 뒤 실행 중인 창의 흔들림만 바뀐다.
- `0`에서는 정점 효과가 사라지고 `80`에서 가장 크게 보인다.
- 터미널에 watcher 미설정 경고가 없다.

변경이 반영되지 않으면 먼저 `file_watcher` feature와 `AssetPlugin.watch_for_changes_override`를 확인합니다.

## P2-C13D-P2 · HIT_COLOR 변경

녹색과 흰색을 저장한 뒤 각각 `H`를 눌러 확인합니다.

```wgsl
const HIT_COLOR: vec3<f32> = vec3<f32>(0.2, 1.0, 0.35);
const HIT_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
```

두 선언을 동시에 두는 것이 아니라 한 선언의 값을 차례로 교체합니다.

## P2-C13D-P3 · 의도적 오류와 복구

1. 변경 전 WGSL이 정상 동작하는지 확인합니다.
2. 세미콜론 하나만 제거하고 저장합니다.
3. 로그에서 asset 경로, 줄, 열, parser 메시지를 기록합니다.
4. 즉시 세미콜론을 복원하고 저장합니다.
5. 프로세스 재시작 없이 화면이 정상으로 돌아오는지 확인합니다.

오류를 여러 개 동시에 만들면 첫 원인을 찾기 어렵습니다. 실험이 끝난 뒤 `git diff`로 WGSL이 원래 상태인지 확인하세요.

## P2-C13D-P4 · texture reload

원본과 같은 크기·atlas 구조의 PNG 복사본을 사용하세요. 레이아웃이 다른 이미지로 바꾸면 reload 여부와 UV 오류를 구분하기 어렵습니다.

### 확인 기준

- Handle과 Material을 새로 생성하지 않아도 픽셀이 바뀐다.
- Rust 코드는 다시 컴파일되지 않는다.
- 원본 파일을 복구한 뒤 다시 반영된다.

## P2-C13D-A1 · 개발 전용 상태 Plugin

`AssetEvent<Shader>`는 Shader asset이 추가·수정·제거되거나 의존성과 함께 로드된 사실을 알려 줍니다. 파일 자체의 로드 실패는 `AssetLoadFailedEvent<Shader>`로 별도 수신합니다.

중요한 제한이 있습니다. `AssetEvent::Modified`는 **파일 변경 감지**이지 GPU shader pipeline 컴파일 성공 보장이 아닙니다. WGSL pipeline 컴파일 오류는 렌더 로그에 나타날 수 있으므로 UI에 “컴파일 성공”이라고 단정하지 말고 다음처럼 표현합니다.

- 변경 감지
- asset 로드 완료
- asset 파일 로드 실패
- GPU 컴파일 결과는 터미널 확인 필요

```rust
fn watch_shader_events(
    mut events: MessageReader<AssetEvent<Shader>>,
    mut status: ResMut<ShaderReloadStatus>,
) {
    for event in events.read() {
        status.last = ReloadState::from_asset_event(event);
    }
}
```

Plugin은 개발 상태 Resource와 UI만 관리합니다. 게임 규칙 Component, SaveGame, 점수 Resource에는 reload 상태를 넣지 않습니다. 배포 구성에서는 Plugin을 등록하지 않거나 feature로 제외할 수 있습니다.

## 전체 코드 실행

```bash
cargo test -p space_survivor --bin shader_reload_status_solution
cargo check -p space_survivor --bin shader_reload_status_solution
```

실제 hot reload 화면:

```bash
cargo run -p space_survivor --bin material_2d
```

전체 코드: `examples/part2/space_survivor/src/bin/shader_reload_status_solution.rs`

