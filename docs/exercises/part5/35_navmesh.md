# 35. NavMesh 과제 해설

[본문으로 돌아가기](../../35_NavMesh.md#실습-과제)

## P5-C35-P1 · 에이전트 속도

`desired_speed`는 순항 속도, `max_speed`는 회피 결과도 넘지 못하는 상한입니다. 둘을 따로 바꿔 역할을 관찰합니다.

## P5-C35-P2 · 다중 에이전트

서로 다른 시작점에서 적을 만들고 반지름·회피 값을 확인합니다. spawn이 겹치면 지역 회피가 풀기 어려운 초기 조건입니다.

## P5-C35-P3 · 이동 Target

`Target` Entity를 Query해 목표를 갱신하고, 목표가 사라졌을 때 기존 목표 유지 또는 정지 정책을 둡니다.

## P5-C35-A1 · 동적 장애물과 문

본문의 역회전 식을 `cell_is_blocked(center, transform, half_extents, agent_radius)` 같은 순수 함수로 분리합니다. 45도 회전한 상자의 모서리 근처 점을 안·밖 한 개씩 골라 테스트하세요.

문은 본문에서 설명한 link 또는 Island 연결을 상태에 따라 비활성화하고 경로를 다시 요청합니다. 수행 예제는 문이 열렸을 때 직선 경로, 닫혔을 때 복도 우회 경로를 선택하는지 검사합니다.

물리 Collider와 NavMesh 장애물 크기 차이는 “길은 찾았지만 통과 못 하는” 문제를 만듭니다. 경로점 도달 반경도 두어 점 주변 진동을 막습니다.

## 전체 코드 실행

```bash
cargo run -p tps_training --bin tps_rules_solution
cargo test -p tps_training --bin tps_rules_solution
```

전체 코드: `examples/part5/tps_training/src/bin/tps_rules_solution.rs`
