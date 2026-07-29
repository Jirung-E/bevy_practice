# 10. Commands 과제 해설

[본문으로 돌아가기](../../10_Commands.md#실습-과제)

## P1-C10-P1 · 세 프레임 동안 적 제거

`app.update()`를 세 번 호출하면 매 프레임 한 적에게 표식이 붙고 다음 System에서 제거됩니다. `chain()`이 두 System 사이의 deferred 명령 적용을 보장하는지 확인하세요.

## P1-C10-P2 · Health가 0인 적 제거

### 힌트

`With<Defeated>` 대신 Query 데이터로 `&Health`를 받고 값이 0인지 검사합니다.

```rust
fn remove_defeated(
    mut commands: Commands,
    enemies: Query<(Entity, &Health), With<Enemy>>,
) {
    for (entity, health) in &enemies {
        if health.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}
```

`Health(0)`은 게임 상태이고 `Defeated`는 처리 단계라는 차이가 있습니다. 사망 연출이 여러 프레임 지속된다면 둘을 함께 사용하는 설계도 가능합니다.

## P1-C10-P3 · Component 제거와 despawn

`remove::<Enemy>()`는 Entity를 유지한 채 Enemy Component만 제거합니다. `despawn()`은 Entity와 모든 Component를 제거합니다.

### 확인 기준

- marker를 제거한 뒤 `With<Enemy>` Query에서는 제외된다.
- 같은 Entity의 Position 같은 다른 Component는 남아 있다.
- despawn 뒤에는 Entity 자체를 조회할 수 없다.

## P1-C10-A1 · 위치를 복사한 Loot

### 접근 방법

제거 Query가 `(Entity, &Health, &Position)`을 가져오게 하고, despawn 예약 전에 Position 값을 새 Loot Entity에 복사합니다.

```rust
commands.spawn((Loot, *position));
commands.entity(entity).despawn();
```

예제의 `Position`은 `Copy`이므로 값을 복사할 수 있습니다. 복사할 수 없는 데이터라면 Loot에 필요한 필드만 새 값으로 구성해야 합니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin commands_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/commands_solution.rs`

