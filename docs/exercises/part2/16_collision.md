# 16. 충돌 과제 해설

[본문으로 돌아가기](../../16_Collision.md#실습-과제)

## P2-C16-P1 · 250점

Message의 `points`를 250으로 바꿉니다. 충돌 System이 직접 Score Resource를 수정하지 않아야 점수 규칙과 충돌 판정을 독립적으로 바꿀 수 있습니다.

## P2-C16-P2 · 작은 플레이어 HitBox

Sprite가 `44 × 36`이라면 예를 들어 HitBox를 `34 × 28`로 줄일 수 있습니다.

### 확인 기준

- 눈에 보이는 Sprite 크기는 그대로다.
- 가장자리의 관대한 판정을 실제 플레이로 확인한다.
- 디버그 표시나 로그로 판정 크기를 확인할 수 있다.

## P2-C16-P3 · 한 총알과 적 두 개

현재 이중 반복은 despawn을 예약한 뒤에도 같은 프레임 Query에 Entity가 남아 있어 한 총알이 두 적과 충돌할 수 있습니다. 따라서 점수 Message가 두 번 쓰일 가능성이 있습니다.

## P2-C16-A1 · 중복 처리 방지

`HashSet<Entity>` 두 개로 이번 충돌 단계에서 이미 제거 예약한 총알과 적을 추적합니다.

```rust
let mut removed_bullets = HashSet::new();
let mut removed_enemies = HashSet::new();
```

총알이 첫 적에 적중하면 두 ID를 기록하고 해당 총알의 내부 반복을 끝냅니다.

### 확인 기준

- 같은 총알은 한 프레임에 최대 한 번 점수를 만든다.
- 이미 제거 예약한 적은 다른 총알이 다시 처리하지 않는다.
- 실제 despawn은 반복 도중 즉시 수행하지 않고 Commands로 예약한다.
- 충돌 결과와 점수 합계를 자동 테스트한다.

관통탄을 만들고 싶다면 무조건 `break`하는 대신 `Piercing { remaining }` 같은 Component로 정책을 명시해야 합니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin combat_solution
cargo test -p space_survivor --bin combat_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/combat_solution.rs`

