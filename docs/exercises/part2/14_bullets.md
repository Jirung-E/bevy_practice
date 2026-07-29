# 14. 총알 과제 해설

[본문으로 돌아가기](../../14_Bullets.md#실습-과제)

## P2-C14-P1 · 속도와 크기

총알 속도는 `Velocity`, 보이는 크기는 Sprite, 충돌 크기는 `HitBox`가 담당합니다. Sprite만 키우고 HitBox를 그대로 둘 수도 있지만 의도적인 판정 설계인지 확인해야 합니다.

## P2-C14-P2 · 수명 0.5초

```rust
Lifetime(Timer::from_seconds(0.5, TimerMode::Once))
```

수명이 짧아지면 화면 위쪽까지 도달하기 전에 사라질 수 있습니다. 속도와 발사 위치를 고정한 채 수명만 비교하세요.

## P2-C14-P3 · Q/E 대각선 발사

```rust
let direction = match key {
    KeyCode::KeyQ => Vec2::new(-1.0, 1.0),
    KeyCode::KeyE => Vec2::new(1.0, 1.0),
    _ => Vec2::Y,
};
let velocity = direction.normalize() * BULLET_SPEED;
```

정규화하지 않으면 대각선 총알이 위쪽 총알보다 `√2`배 빠릅니다.

## P2-C14-A1 · 초당 5발 cooldown

`pressed`는 매 프레임 참일 수 있으므로 반복 Timer 또는 누적 시간으로 발사 간격을 제한합니다. 초당 5발의 간격은 `1 / 5 = 0.2`초입니다.

### 확인 기준

- 60Hz와 144Hz에서 초당 발사 수가 같다.
- Space를 떼면 새 총알이 생성되지 않는다.
- 긴 프레임이 발생해도 누적 시간 오차가 계속 커지지 않는다.
- 발사 System이 총알 이동이나 수명까지 책임지지 않는다.

## 전체 코드

`combat_solution`은 cooldown과 Q/E 방향 계산을 독립적으로 실행·테스트합니다.

```bash
cargo test -p space_survivor --bin combat_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/combat_solution.rs`

