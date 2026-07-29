# 10. Commands: 월드 구조 변경하기

## 학습 목표

- Commands가 구조 변경을 지연하는 이유를 설명할 수 있다.
- Entity에 Component를 삽입하고 제거할 수 있다.
- Entity를 안전하게 despawn할 수 있다.

## 이번에 만들 결과물

적 세 명을 생성하고, 매 프레임 한 명에게 Defeated 표식을 붙인 뒤 해당 Entity를 제거합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin commands
```

## 핵심 개념

Query를 순회하는 중에 같은 World의 저장 구조가 바뀌면 참조가 무효화될 수 있습니다. Commands는 생성, Component 변경, despawn 같은 구조 변경을 명령 큐에 기록하고 안전한 시점에 일괄 적용합니다.

명령은 즉시 적용되지 않는다는 점이 중요합니다. 뒤 System이 변경 결과를 봐야 한다면 System 순서를 정하고 deferred 명령이 사이에서 적용되게 해야 합니다. `chain()`은 이 흐름을 간단히 표현합니다.

## 샘플 코드

```rust
#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Defeated;

fn mark_one_enemy(
    mut commands: Commands,
    enemies: Query<Entity, (With<Enemy>, Without<Defeated>)>,
) {
    if let Some(entity) = enemies.iter().next() {
        commands.entity(entity).insert(Defeated);
        println!("{entity:?}에 Defeated 추가");
    }
}

fn remove_defeated(mut commands: Commands, defeated: Query<Entity, With<Defeated>>) {
    for entity in &defeated {
        commands.entity(entity).despawn();
        println!("{entity:?} 제거 예약");
    }
}
```

두 System은 `(mark_one_enemy, remove_defeated).chain()`으로 등록합니다.

남은 적의 수는 전체 World Entity 수가 아니라 `With<Enemy>` Query의 결과를 세어 확인합니다.

## 코드 설명

- `commands.entity(entity)`는 기존 Entity를 수정할 EntityCommands를 만듭니다.
- `insert(Defeated)`는 표식을 추가하거나 같은 타입의 기존 값을 교체합니다.
- `despawn()`은 Entity와 모든 Component 제거를 예약합니다.
- `if let Some(...)`은 적이 하나 이상 있을 때만 명령을 만듭니다.
- Query의 반복 순서는 게임 규칙으로 사용하면 안 됩니다. 실제 대상 선택에는 거리, 우선순위 같은 명시적 기준을 적용하세요.
- `world.entities().len()`에는 Bevy 내부 Entity도 포함될 수 있으므로 게임 대상 수에는 Component Query를 사용합니다.

## 실습 과제

1. `app.update()`를 세 번 실행해 모든 적을 제거하세요.
2. Defeated 대신 `Health(0)`인 적을 제거하도록 바꾸세요.
3. `commands.entity(entity).remove::<Enemy>()`를 실험하고 despawn과 차이를 확인하세요.

## 심화 과제

적 제거 직전에 새 `Loot` Entity를 생성하세요. Loot에 원래 적의 위치를 복사하려면 Query가 어떤 데이터를 추가로 가져와야 하는지 설계하고 구현해 보세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/10_commands.md)를 확인하세요.

## 다음 챕터

적 제거 사실을 점수 System에 직접 연결하지 않고 Message로 전달해 System 사이 결합도를 낮춥니다.
