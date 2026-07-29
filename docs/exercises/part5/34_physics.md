# 34. 물리 과제 해설

[본문으로 돌아가기](../../34_Physics.md#실습-과제)

## P5-C34-P1 · 중력과 점프

점프 속도만 키우면 최고점과 체공 시간이 모두 달라집니다. 중력과 초기 수직 속도 조합을 기록해 비교합니다.

## P5-C34-P2 · Mesh와 Collider

형상을 의도적으로 다르게 해 접촉 지점을 확인합니다. 보이지 않는 벽이나 모델 관통은 두 형상 차이에서 생길 수 있습니다.

## P5-C34-P3 · 디버그 렌더

Avian 디버그 렌더 Plugin은 개발 구성에서만 켭니다. Collider와 shape cast를 표시하되 출시 빌드에는 포함하지 않습니다.

## P5-C34-A1 · shape cast 컨트롤러

발 아래로 capsule shape cast를 하고 hit normal과 up의 각도를 계산합니다. 수행 예제는 `normal · up >= cos(max_slope)`로 45도보다 가파른 표면을 지면에서 제외합니다.

- 계단 오르기는 전방 장애물과 위쪽 여유 공간을 함께 검사합니다.
- 공중 제어는 목표 속도로 제한된 비율만 보간합니다.
- 단일 ray보다 캐릭터 폭을 반영하는 shape cast가 모서리에서 안정적입니다.

## 전체 코드 실행

```bash
cargo test -p tps_training --bin tps_rules_solution
```

전체 코드: `examples/part5/tps_training/src/bin/tps_rules_solution.rs`
