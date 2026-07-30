# 34. 물리 과제 해설

[본문으로 돌아가기](../../34_Physics.md#실습-과제)

## P5-C34-P1 · 중력과 점프

점프 속도만 키우면 최고점과 체공 시간이 모두 달라집니다. 중력과 초기 수직 속도 조합을 기록해 비교합니다.

## P5-C34-P2 · Mesh와 Collider

형상을 의도적으로 다르게 해 접촉 지점을 확인합니다. 보이지 않는 벽이나 모델 관통은 두 형상 차이에서 생길 수 있습니다.

## P5-C34-P3 · Collider 크기 비교

본문에서 이미 켠 `PhysicsDebugPlugin`으로 시각 Mesh와 Collider를 동시에 봅니다. 반지름을 너무 작게 하면 모델이 벽에 파고들어 보이고, 너무 크게 하면 보이지 않는 공간에서 충돌합니다.

## P5-C34-A1 · 경사와 공중 가속도 비교

본문의 shape cast와 `normal · up >= cos(max_slope)`를 그대로 사용하고 상수만 바꿉니다. 순수 함수 테스트에서 35도 제한과 65도 제한이 같은 normal을 다르게 판정하는지 확인합니다.

공중 가속도 0은 점프 뒤 방향을 바꿀 수 없고, 지상 가속도의 30%는 제한적으로 바꿀 수 있습니다. 두 값을 같은 점프 시나리오에서 비교해 수평 이동 거리를 기록합니다.

## 전체 코드 실행

```bash
cargo test -p tps_training --bin tps_rules_solution
cargo run -p tps_training --bin 34_physics
```

전체 코드: `examples/part5/tps_training/src/bin/tps_rules_solution.rs`
