# 15. 적 생성 과제 해설

[본문으로 돌아가기](../../15_Enemies.md#실습-과제)

## P2-C15-P1 · 0.4초 생성 간격

Timer를 `0.4`초로 바꾸면 10초 동안 이상적으로 약 25회 생성됩니다. 프레임 수로 생성하지 말고 `Time::delta()`로 Timer를 tick하세요.

## P2-C15-P2 · 순번별 속도

```rust
let speed = base_speed + (sequence % 3) as f32 * 15.0;
```

동일한 sequence는 항상 같은 속도를 만들어 재현할 수 있습니다. 속도 차이가 Lifetime과 화면 이탈 시점에도 영향을 주는지 확인하세요.

## P2-C15-P3 · 다섯 번째 큰 적

```rust
let size = if sequence % 5 == 0 { 64.0 } else { 38.0 };
```

Sprite와 HitBox를 같은 값으로 만들지, 큰 Sprite보다 작은 판정을 줄지는 게임 설계 선택입니다. 한쪽만 우연히 바뀌지 않도록 명시하세요.

## P2-C15-A1 · Difficulty Resource

난이도 단계가 오를 때 다음 값을 함께 계산합니다.

- spawn interval: 단계마다 감소하되 최소값 유지
- enemy speed: 단계마다 증가
- 화면에 표시할 level

```rust
let stage = (elapsed_seconds / 15.0).floor() as u32;
let interval = (0.9 - stage as f32 * 0.1).max(0.25);
let speed = 135.0 + stage as f32 * 20.0;
```

### 확인 기준

- 시간이 같은 경우 결과가 결정적이다.
- interval이 0이나 음수가 되지 않는다.
- 현재 Timer의 duration을 변경할 때 남은 시간 처리 정책을 정한다.
- 난이도 계산과 실제 Entity 생성 책임을 분리한다.

전체 검증 코드: `examples/part2/space_survivor/src/bin/combat_solution.rs`

