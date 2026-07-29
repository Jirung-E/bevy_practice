# 42. 모듈화 과제 해설

[본문으로 돌아가기](../../42_Modularization.md#실습-과제)

## P7-C42-P1 · input.rs

키를 읽어 도메인 의도로 바꾸는 코드와 캐릭터 이동 규칙을 분리합니다. input 모듈은 `Move(Vec2)`, `Attack` 같은 계약만 내보냅니다.

## P7-C42-P2 · private plugins

crate 외부에서는 완성된 PluginGroup 또는 builder 함수만 사용하게 하고 내부 `plugins` 모듈은 private로 둡니다. 컴파일 실패를 확인하는 UI 테스트가 가장 확실한 공개 범위 검증입니다.

## P7-C42-P3 · pub 축소

다른 모듈에서 실제 사용하는 항목만 `pub(crate)` 또는 `pub`로 남깁니다. 테스트 편의를 위해 제품 API를 public으로 넓히지 말고 같은 모듈의 단위 테스트를 활용합니다.

## P7-C42-A1 · 별도 Gameplay crate 계약

외부 공개 계약은 다음으로 제한할 수 있습니다.

- `GameplayPlugin` 또는 `GameplayPluginGroup`
- 입력 Command/Message
- 결과 Event와 읽기 전용 snapshot
- 구성 설정 타입

Presentation 타입을 Gameplay가 참조하지 않게 하며, 상위 App crate가 두 crate를 조립합니다. 수행 예제의 `GameplayPort`는 내부 Vec 저장 방식을 감추고 `queue_attack` 계약만 노출합니다.

순환 의존이 생기면 공통 메시지 타입을 더 낮은 contract crate로 내리거나 상위 조립 계층으로 이동합니다. 서로의 구체 타입을 양방향으로 참조하는 방식은 피합니다.

## 전체 코드 실행

```bash
cargo test -p production_structure --bin production_solution
```

전체 코드: `examples/part7/production_structure/src/bin/production_solution.rs`
