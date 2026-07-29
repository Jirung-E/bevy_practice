# 30. Light 과제 해설

[본문으로 돌아가기](../../30_Light.md#실습-과제)

## P4-C30-P1 · 광원 색 교환

Transform과 intensity는 그대로 둔 채 색만 교환합니다. 키 라이트와 보조 라이트가 제품의 어느 면을 분리해 보이게 하는지 관찰하세요.

## P4-C30-P2 · PointLight range

range 밖에서는 영향이 사라집니다. intensity를 동시에 바꾸지 않아야 거리 범위가 만든 차이를 분리해 볼 수 있습니다.

## P4-C30-P3 · 그림자 비용

광원을 하나씩 끄는 것과 `shadows_enabled`만 끄는 경우를 각각 측정합니다. 눈으로 부드러워 보인다는 판단과 프레임 시간 수치를 함께 기록하세요.

## P4-C30-A1 · 회전 광원과 진단

세 번째 PointLight 위치를 `cos(angle) * radius`, `sin(angle) * radius`로 계산합니다. 수행 예제는 시간이 달라도 XZ 반지름이 일정한지 검사합니다.

`FrameTimeDiagnosticsPlugin`을 추가하고 최소 수백 프레임의 평균 프레임 시간을 비교하세요. 한 프레임의 FPS는 노이즈가 크므로 다음 조건을 고정합니다.

- 창 크기와 카메라 위치
- 제품·광원 수
- 그림자 on/off 조합
- warm-up 뒤 같은 측정 시간

GPU 병목 분석은 FPS만으로 원인을 확정할 수 없으므로 전문 GPU profiler가 후속 단계입니다.

## 전체 코드 실행

```bash
cargo run -p product_showcase --bin showcase_solution
cargo test -p product_showcase --bin showcase_solution
```

전체 코드: `examples/part4/product_showcase/src/bin/showcase_solution.rs`
