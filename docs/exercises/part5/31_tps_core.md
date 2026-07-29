# 31. TPS 코어 과제 해설

[본문으로 돌아가기](../../31_TpsCore.md#실습-과제)

## P5-C31-P1 · 속도와 시작 위치

이동 속도와 spawn Transform을 각각 바꿉니다. 시작 위치가 지면 Collider 안쪽이면 물리 해결 과정에서 튈 수 있으므로 캐릭터 반높이를 고려합니다.

## P5-C31-P2 · Shift 달리기

방향 입력과 Shift 상태를 분리해 최종 속도만 선택합니다. 대각선 입력은 먼저 길이를 1 이하로 제한해야 대각선이 더 빠르지 않습니다.

## P5-C31-P3 · 조합 모델

루트 Entity는 이동·충돌을 담당하고 Cuboid/Sphere는 자식 시각 모델로 둡니다. 시각 모델을 바꿔도 컨트롤러 Query와 Collider가 유지됩니다.

## P5-C31-A1 · MovementSettings

걷기·달리기·공중 제어 값을 Resource로 모으고 입력 System의 숫자 상수를 제거합니다. 수행 예제는 대각선 입력을 정규화한 뒤 walk/run 속도를 적용합니다. 캐릭터마다 값이 다르면 Resource보다 Component가 적합합니다.

## 전체 코드 실행

```bash
cargo test -p tps_training --bin tps_rules_solution
```

전체 코드: `examples/part5/tps_training/src/bin/tps_rules_solution.rs`
