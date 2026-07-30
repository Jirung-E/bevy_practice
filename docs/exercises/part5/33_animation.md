# 33. 애니메이션 과제 해설

[본문으로 돌아가기](../../33_Animation.md#실습-과제)

## P5-C33-P1 · 흔들림 값

진폭과 주파수를 하나씩 바꾸고 루트가 아니라 시각 모델 자식에 적용합니다. Collider까지 흔들면 접지 판정이 불안정해집니다.

## P5-C33-P2 · 달리기 주파수

실제 수평 속도에서 Idle/Walk/Run 상태를 계산하면 외력이나 감속 중에도 표현이 자연스럽습니다.

## P5-C33-P3 · 반대 위상 팔다리

왼팔·오른다리와 오른팔·왼다리를 같은 위상 그룹으로 묶고 한 그룹에 `sin(t)`, 다른 그룹에 `-sin(t)`을 적용합니다.

## P5-C33-A1 · 상태별 전환 시간 비교

33A에서 이미 구성한 AnimationGraph를 유지하고 상태별 Duration만 바꿉니다. Walk의 0.12초는 입력 반응이 빠르지만 자세가 급하게 바뀔 수 있고, Run의 0.25초는 부드럽지만 짧은 Shift 입력에는 Run 자세가 완전히 나타나지 않을 수 있습니다.

상태 변화 시각과 전환 완료 예상 시각을 로그로 남겨 차이를 비교하세요. root motion을 사용하는 모델이라면 물리 루트와 애니메이션 중 누가 이동을 소유하는지도 별도로 결정해야 합니다.

## 전체 코드 실행

```bash
cargo test -p tps_training --bin tps_rules_solution
cargo run -p tps_training --bin 33a_gltf_character
```

전체 코드: `examples/part5/tps_training/src/bin/tps_rules_solution.rs`
