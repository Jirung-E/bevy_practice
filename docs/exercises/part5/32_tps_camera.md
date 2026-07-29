# 32. TPS 카메라 과제 해설

[본문으로 돌아가기](../../32_TpsCamera.md#실습-과제)

## P5-C32-P1 · 초기 거리와 pitch

거리와 pitch를 따로 바꿔 캐릭터의 화면 비율과 지면 가시성을 비교합니다. 극단값에서는 near plane과 지면 관통도 확인합니다.

## P5-C32-P2 · 감도 Resource

yaw/pitch 감도와 반전 옵션을 카메라 설정 Resource로 옮깁니다. 마우스 motion delta에 감도를 곱하되 프레임 시간을 다시 곱하지 않습니다.

## P5-C32-P3 · 어깨 전환

Q/E로 focus 기준 right 방향 offset 부호를 바꿉니다. world X를 직접 바꾸면 yaw 회전 뒤 어깨 방향이 어긋납니다.

## P5-C32-A1 · 벽 관통 방지

focus에서 원하는 카메라 위치로 raycast하고, hit 거리에서 안전 여백을 뺀 값으로 실제 거리를 줄입니다. 장애물이 사라지면 원하는 거리로 보간해 복귀합니다.

수행 예제는 원하는 거리 6에서 2만큼 떨어진 벽을 만나면 여백 0.2를 두고 1.8로 줄이는지 검사합니다. 플레이어 Collider는 raycast 필터에서 제외하세요.

## 전체 코드 실행

```bash
cargo test -p tps_training --bin tps_rules_solution
```

전체 코드: `examples/part5/tps_training/src/bin/tps_rules_solution.rs`
