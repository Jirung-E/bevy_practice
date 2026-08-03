# 10A. Entity 수명과 제거 감지

## 학습 목표

- Component 제거와 Entity despawn의 차이를 구분할 수 있다.
- `RemovedComponents<T>`로 제거 사실을 다른 System에서 관찰할 수 있다.
- 저장해 둔 Entity ID가 여전히 유효한지 검사하고 참조를 정리할 수 있다.

## 이 내용으로 만들 수 있는 것

- 대상이 사라질 때 lock-on, UI, 소유권 정보를 안전하게 해제하는 시스템
- 장비·상태 효과 Component가 제거될 때 후속 정리를 실행하는 시스템
- 제거된 Entity ID를 계속 사용해 생기는 오류를 방지하는 참조 관리

## 이번에 만들 결과물

적 Entity에서 `Health`만 먼저 제거해 `RemovedComponents<Health>`로 감지하고, 다음 업데이트에서 Entity를 despawn한 뒤 Resource에 남은 ID를 정리합니다.

```bash
cargo run -p ecs_basics --bin entity_lifecycle
```

```text
제거 전 Health: 30
Health 제거 감지: 1건
무효 Entity 참조 정리: true
```

## 핵심 개념

Component 제거는 Entity를 남겨 둔 채 역할만 바꿉니다. `despawn()`은 Entity와 모든 Component를 제거합니다.

```text
remove::<Health>()   Enemy Entity는 존재, Health만 없음
despawn()            Entity ID 자체가 더 이상 World에 없음
```

`RemovedComponents<Health>`는 제거된 값이 아니라 **어떤 Entity에서 제거됐는지** 전달합니다. 제거된 `Health` 값이 필요하다면 제거하기 전에 별도 Message나 Resource로 복사해야 합니다.

Entity ID는 세대를 포함해 오래된 ID가 새 Entity를 잘못 가리키는 일을 막지만, ID를 Resource에 저장했다고 자동으로 `None`이 되지는 않습니다. `Query::get`, `World::get_entity` 등으로 유효성을 확인하고 소유자가 참조를 정리해야 합니다.

| 필요한 동작 | 도구 |
|---|---|
| 생성·삽입·제거 예약 | `Commands` |
| 특정 Component가 사라졌는지 관찰 | `RemovedComponents<T>` |
| 현재 Component 조합 확인 | `Query::get(entity)` |
| Entity 자체가 존재하는지 확인 | `World::get_entity(entity)` |
| 제거 직전 값이 필요한 처리 | 제거 전 Message 또는 Observer/hook |

## 샘플 코드

전체 코드: `examples/part1/ecs_basics/src/bin/10a_entity_lifecycle.rs`

```rust
fn record_removed_health(
    mut removed: RemovedComponents<Health>,
    mut report: ResMut<LifecycleReport>,
) {
    report.removed_health.extend(removed.read());
}
```

```rust
if enemies.get(entity).is_err() {
    tracked.0 = None;
    report.stale_reference_cleared = true;
}
```

## 코드 설명

- `remove::<Health>()` 뒤 deferred 명령이 적용되면 `RemovedComponents<Health>` Reader가 Entity ID를 받습니다.
- `Without<Health>`는 적이 살아 있지만 체력 역할을 잃은 중간 상태를 찾습니다.
- 두 번째 업데이트에서 적을 despawn합니다.
- `Query::get` 실패를 확인한 Resource 소유자가 오래된 ID를 `None`으로 바꿉니다.
- `.chain()`은 제거·관찰·despawn·정리 순서를 학습 예제에서 명시합니다.

## 실습 과제

1. `Shield` Component를 추가하고 제거 횟수를 별도로 기록하세요.
2. Health 제거와 Entity despawn 사이에 `Defeated` marker를 붙여 사망 연출 상태를 표현하세요.
3. 추적 대상이 아닌 Entity의 Health 제거는 `TrackedTarget`을 지우지 않는지 테스트하세요.

## 심화 과제

하나의 Entity를 여러 Resource가 참조하는 상황을 만들고, 제거 알림을 Message로 변환해 각 소유자가 독립적으로 참조를 정리하게 하세요. 제거 순서에 따라 메시지가 유실되지 않는 테스트도 작성하세요.

[힌트와 수행 예시](exercises/part1/10a_entity_lifecycle.md)

## 다음 챕터

Messages와 Events를 사용해 여러 System이 직접 호출 없이 결과를 공유합니다.
