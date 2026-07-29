# 45. 최적화 과제 해설

[본문으로 돌아가기](../../45_Optimization.md#실습-과제)

## P7-C45-P1 · 적 1,000개

카메라·맵·적 AI 조건을 고정하고 warm-up 뒤 평균·p95 프레임 시간을 기록합니다. “느려 보인다”가 아니라 재현 가능한 수치를 남깁니다.

## P7-C45-P2 · Query 최소화

System이 실제 읽고 쓰는 Component만 Query에 포함합니다. `&mut Transform`이 필요하지 않으면 `&Transform`을 사용해 병렬 실행 제약과 변경 감지를 불필요하게 만들지 않습니다.

## P7-C45-P3 · dev와 release

같은 장면과 측정 시간을 사용해 비교합니다. dev 성능만 보고 알고리즘을 확정하지 않되, 개발 반복 속도 문제도 별도 지표로 관리합니다.

## P7-C45-A1 · 자동 성능 비교

시나리오 파일에 빌드 프로필, 프레임 시간, Entity 수, 적 수, 해상도, 실행 환경을 기록합니다. 후보 결과가 기준보다 허용 비율 이상 느려지면 실패시킵니다.

수행 예제는 다음 두 경우를 구분합니다.

- Entity/적 수가 다르면 성능 회귀가 아니라 `ScenarioMismatch`
- 같은 시나리오에서 10% 허용치를 넘으면 `FrameTime`

CI 공유 러너는 노이즈가 크므로 여러 번 실행한 중앙값, 전용 머신, 넉넉한 경고 임계값을 조합합니다. microbenchmark와 실제 프레임 시나리오는 서로 대체하지 않습니다.

## 전체 코드 실행

```bash
cargo run -p production_structure --bin production_solution
cargo test -p production_structure --bin production_solution
```

전체 코드: `examples/part7/production_structure/src/bin/production_solution.rs`
