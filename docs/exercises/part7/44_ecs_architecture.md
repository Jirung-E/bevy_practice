# 44. ECS 아키텍처 과제 해설

[본문으로 돌아가기](../../44_EcsArchitecture.md#실습-과제)

## P7-C44-P1 · 두 번째 Reader

점수 Reader와 사운드 Reader가 같은 적 제거 Message를 독립적으로 읽게 합니다. 하나의 System이 읽은 뒤 다른 System용 임시 Resource를 만드는 결합을 피합니다.

## P7-C44-P2 · 공격 쿨다운

전역 하나면 Resource, 캐릭터별이면 Component로 둡니다. 고정 시뮬레이션 시간으로 tick해 렌더 프레임률과 공격 빈도를 분리합니다.

## P7-C44-P3 · Feedback 순서

점수 계산이 먼저, HUD 반영이 나중이라는 순서를 SystemSet 또는 명시적 `after`로 표현합니다. 같은 프레임에 새 점수를 보여야 한다는 요구사항이 스케줄에 드러나야 합니다.

## P7-C44-A1 · Fixed Simulation과 Presentation

Update는 키 변화를 도메인 `InputCommand`로 버퍼에 넣고 FixedUpdate는 버퍼를 한 번 소비해 시뮬레이션합니다. Presentation은 최신 simulation snapshot을 보간해 그립니다.

수행 예제는 Update에서 넣은 Attack을 FixedUpdate가 정확히 한 번만 소비하는지 검사합니다.

- 짧은 버튼 입력을 현재 키 상태만으로 읽으면 FixedUpdate 사이에서 놓칠 수 있습니다.
- 축 입력은 최신 값을 유지하고 edge 입력은 queue하는 혼합 정책이 흔합니다.
- 렌더 보간 데이터와 authoritative Transform의 소유권을 분리합니다.

## 전체 코드 실행

```bash
cargo test -p production_structure --bin production_solution
```

전체 코드: `examples/part7/production_structure/src/bin/production_solution.rs`
