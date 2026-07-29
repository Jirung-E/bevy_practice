# 13. 플레이어 이동

## 학습 목표

- 2D 카메라와 단색 Sprite를 배치할 수 있다.
- 키보드 입력을 방향 벡터로 변환할 수 있다.
- 프레임 독립 이동과 화면 경계 제한을 구현할 수 있다.

## 이번에 만들 결과물

청록색 우주선을 WASD 또는 방향키로 움직이는 첫 2D 게임 화면을 만듭니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 13_player_movement
```

## 핵심 개념

화면 중앙은 `(0, 0)`이고 기본 Camera2d에서 X는 오른쪽, Y는 위쪽입니다. 화면에 그릴 Entity에는 `Sprite`와 `Transform`을 붙입니다.

입력 방향을 그대로 더하면 대각선 속도가 더 빨라집니다. `normalize_or_zero()`로 길이를 1로 맞춘 뒤 `속도 × delta time`을 곱해야 컴퓨터 성능과 방향에 무관한 이동이 됩니다.

## 샘플 코드

```rust
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    player.translation +=
        (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
}
```

실행 파일은 `LessonConfig::MOVEMENT`로 통합 게임의 이동 기능만 활성화합니다. 전체 구현은 `examples/part2/space_survivor/src/lib.rs`에 있습니다.

## 코드 설명

- `ButtonInput<KeyCode>`는 현재 눌린 키 상태를 보관하는 Resource입니다.
- `pressed`는 누르고 있는 모든 프레임에 참입니다.
- `Single`은 조건에 맞는 Entity가 정확히 하나여야 한다는 의도를 표현합니다.
- `time.delta_secs()`는 직전 프레임에 걸린 초 단위 시간입니다.
- `Vec2::extend(0.0)`은 이동 벡터를 Transform의 Vec3에 맞춥니다.
- 완성 코드에서는 Sprite 반 크기를 고려해 `clamp`로 창 밖 이동을 막습니다.

## 실습 과제

1. `PLAYER_SPEED`를 200과 800으로 바꾸어 비교하세요.
2. IJKL 키도 이동 입력으로 추가하세요.
3. 플레이어 색과 크기를 바꾸고 경계 계산도 맞추세요.

## 심화 과제

이동 입력이 없을 때 서서히 멈추는 가속·감속 이동을 구현하세요. Velocity Component를 추가하고 최대 속도를 제한하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/13_player_movement.md)를 확인하세요.

## 다음 챕터

Space 키를 누르면 플레이어 위치에서 위쪽으로 날아가는 총알을 생성합니다.
