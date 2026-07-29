# 12B. Reflect와 DynamicWorld 과제 해설

[본문으로 돌아가기](../../12B_ReflectDynamicWorld.md#실습-과제)

## P1-C12B-P1 · Mana Component

`Mana`에도 `Component`, `Reflect`, `Default`를 derive하고 `#[reflect(Component)]`를 붙입니다. 타입 정의만으로는 부족하며 `app.register_type::<Mana>()`도 필요합니다.

확인할 것은 세 가지입니다.

- Mana를 가진 Entity만 RON에 해당 Component가 존재한다.
- 값이 왕복 뒤 유지된다.
- Mana가 없는 Entity에 기본값이 임의로 추가되지 않는다.

## P1-C12B-P2 · RON 값 편집

실행 바이너리가 매번 파일을 먼저 덮어쓰므로, 수동 편집 실습에서는 저장 단계와 불러오기 단계를 입력 또는 별도 함수로 나누는 편이 좋습니다. Health 타입 아래 숫자만 바꾸고 타입 경로·괄호 구조는 유지하세요.

사람이 편집할 수 있다는 점은 RON의 장점이지만, 임의 입력이므로 항상 역직렬화 오류를 처리해야 합니다.

## P1-C12B-P3 · 손상 파일 fallback

```rust
let restored = match restore_world(&source) {
    Ok(world) => world,
    Err(error) => {
        eprintln!("scene load failed: {error}");
        registered_app()
    }
};
```

오류를 숨기고 기본값만 쓰면 사용자는 저장 손상을 알 수 없습니다. 오류를 표시하되 앱은 기본 World로 계속 실행하는 정책이 학습 예제에 적합합니다. 에디터에서는 손상 파일을 덮어쓰지 말고 복구 기회를 남기는 편이 안전합니다.

## P1-C12B-A1 · 명시적 allowlist

```rust
let dynamic_world = DynamicWorldBuilder::from_world(world, &registry)
    .deny_all_components()
    .allow_component::<Position>()
    .allow_component::<Health>()
    .allow_component::<Mana>()
    .extract_entities(entity_ids.into_iter())
    .remove_empty_entities()
    .build();
```

자동 추출은 새로 등록된 Component가 저장에 들어갈 수 있어 작은 실습에는 편하지만 장기간 유지하는 형식에는 위험할 수 있습니다. allowlist는 저장 계약이 코드에 명시되며, 새 Component를 저장하려면 의도적으로 목록을 변경해야 합니다.

반대로 플러그인이 많은 확장형 에디터에서는 중앙 allowlist가 확장성을 막을 수 있습니다. 이 경우 각 플러그인이 저장 가능한 타입을 등록하는 정책과 형식 버전 검증을 조합할 수 있습니다.

## 확인 명령

```bash
cargo run -p ecs_basics --bin dynamic_world
cargo test -p ecs_basics --bin dynamic_world
cargo run -p ecs_basics --bin dynamic_world_solution
cargo test -p ecs_basics --bin dynamic_world_solution
```

본문 실행 코드: `examples/part1/ecs_basics/src/bin/12b_dynamic_world.rs`

과제 수행 예시 전체 코드: `examples/part1/ecs_basics/src/bin/dynamic_world_solution.rs`
