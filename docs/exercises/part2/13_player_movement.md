# 13. 플레이어 이동 과제 해설

[본문으로 돌아가기](../../13_PlayerMovement.md#실습-과제)

## P2-C13-P1 · 이동 속도 비교

`PLAYER_SPEED`를 200과 800으로 바꾸면 같은 입력 시간 동안 이동 거리가 각각 달라집니다.

### 확인 기준

- 같은 방향을 같은 시간 동안 입력한다.
- 낮은 속도와 높은 속도에서 프레임 수가 아니라 실제 이동 거리를 비교한다.
- 높은 속도에서도 플레이어가 화면 경계를 벗어나지 않는다.

속도 상수는 초당 이동 거리이고 실제 프레임 이동량은 `speed * time.delta_secs()`입니다.

## P2-C13-P2 · IJKL 입력

기존 키와 새 키를 `||`로 묶습니다.

```rust
if keyboard.pressed(KeyCode::ArrowLeft)
    || keyboard.pressed(KeyCode::KeyA)
    || keyboard.pressed(KeyCode::KeyJ)
{
    direction.x -= 1.0;
}
```

`I`, `J`, `K`, `L`은 각각 위, 왼쪽, 아래, 오른쪽에 대응시킵니다. 두 입력 체계가 동시에 눌려도 방향을 마지막에 `normalize_or_zero()`하므로 대각선 속도가 더 빨라지지 않습니다.

## P2-C13-P3 · 크기와 경계 계산

Sprite 크기를 바꾸면 경계 계산에 사용하는 반쪽 크기도 같은 값에서 계산해야 합니다.

```rust
const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 48.0);
let half_size = PLAYER_SIZE / 2.0;
```

화면에 그리는 값과 충돌·경계 값을 별도 숫자로 반복하면 한쪽만 수정하기 쉽습니다. 같은 상수나 Component를 공유하세요.

## P2-C13-A1 · 가속과 감속

### 접근 방법

1. 입력 방향으로 목표 속도를 계산합니다.
2. 현재 Velocity를 목표 속도 쪽으로 가속합니다.
3. 입력이 없으면 목표 속도 0을 향해 감속합니다.
4. 최대 속도를 제한한 뒤 위치에 적용합니다.

```rust
velocity.0 = velocity
    .0
    .move_towards(target_velocity, acceleration * delta_seconds);
velocity.0 = velocity.0.clamp_length_max(max_speed);
```

### 확인 기준

- 키를 놓은 직후 속도가 한 프레임에 0이 되지 않는다.
- 충분한 시간이 지나면 정지한다.
- 반대 방향 입력 시 감속을 거쳐 방향이 바뀐다.
- 속도 길이가 최대 속도를 넘지 않는다.

`Transform`에 가속 상태를 저장하지 않고 `Velocity` Component가 책임지게 하면 물리, 넉백, AI 이동과 조합하기 쉽습니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin movement_solution
cargo test -p space_survivor --bin movement_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/movement_solution.rs`

