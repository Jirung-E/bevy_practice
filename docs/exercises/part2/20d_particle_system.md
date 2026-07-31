# 20D. 파티클 시스템 과제 해설

[본문으로 돌아가기](../../20D_ParticleSystem.md#실습-과제)

## P2-C20D-P1 · burst 밀도

`PARTICLE_COUNT`를 바꾸기 전에 Space를 한 번 눌렀을 때 표시되는 최대 입자 수와 0으로 돌아오는 시간을 기록하세요. 개수만 바꾸면 각도 간격은 `TAU / PARTICLE_COUNT` 계산에 의해 자동으로 달라집니다.

확인할 점:

- 12개에서도 원형 방향을 알아볼 수 있는가?
- 72개에서 시각적 밀도 증가가 Entity 수 증가만큼 가치가 있는가?
- 수명은 동일하므로 제거 시점도 비슷한가?

## P2-C20D-P2 · 반대 방향 가속도

폭발용 `Particle`의 가속도를 다음처럼 바꾸어 비교합니다.

```rust
acceleration: Vec2::new(0.0, 120.0),
```

초기 velocity는 바깥쪽을 향하지만 시간이 지날수록 위쪽 속도가 더해집니다. 위치를 직접 위로 더하는 것과 달리 velocity가 매 프레임 누적해서 변한다는 점을 관찰하세요.

## P2-C20D-P3 · 시간 기준 방출

`ThrusterTimer`의 주기를 바꾸고 1초 동안 생성될 입자 수를 예상합니다.

```text
초당 방출 횟수 ≈ 1 / Timer 간격
초당 입자 수 ≈ 초당 방출 횟수 × 한 번에 생성하는 입자 수
```

0.1초 간격에 매번 세 개를 만들면 대략 초당 30개입니다. 실제 화면의 수는 기존 입자의 수명과 동시에 살아 있는 시간에도 영향을 받습니다.

## P2-C20D-P4 · 얼음 폭발

폭발 입자의 시작색을 흰색에 가까운 청록, 끝색을 투명한 파랑으로 바꾸세요. 크기가 처음에 급격히 커졌다가 줄어드는 효과가 필요하다면 단순한 시작·끝 선형 보간만으로는 부족합니다. 우선 색상만 바꿔 기존 구조 안에서 변형한 뒤 심화 단계에서 곡선을 고려하세요.

## P2-C20D-A1 · 재사용 pool

pool을 구현하기 전에 다음 상태를 분리합니다.

- 사용할 수 있는 비활성 입자
- 화면에서 갱신 중인 활성 입자
- emitter가 요구한 새 입자의 초기값

한 가지 접근은 미리 Entity를 만들고 `Visibility::Hidden` 상태로 두는 것입니다. 방출할 때 비활성 Entity에 `Particle`을 삽입하고 보이게 만들며, 수명이 끝나면 `Particle`을 제거하고 다시 숨깁니다.

주의할 점:

- pool 용량을 무한히 늘리면 생성 비용만 미룬 것에 불과합니다.
- 비활성 Entity도 World에 있으므로 Query filter를 명확히 해야 합니다.
- 작은 효과에서는 spawn/despawn이 더 단순하고 충분히 빠를 수 있습니다.

먼저 기본 예제를 profiler로 측정한 뒤 pool 적용 전후를 비교해야 최적화의 의미가 있습니다.

## 실행과 테스트

```bash
cargo run -p space_survivor --bin particle_system
cargo test -p space_survivor --bin particle_system
```

전체 코드: `examples/part2/space_survivor/src/bin/20d_particle_system.rs`
