# 08. Query: 원하는 데이터 찾기

## 학습 목표

- Query의 데이터 항목과 Filter를 구분할 수 있다.
- `With`, `Without`, `Entity`를 조합할 수 있다.
- 가변 Query가 만드는 데이터 접근 규칙을 이해한다.
- `iter`, `iter_mut`, `single`, `get`, `get_mut`을 상황에 맞게 사용할 수 있다.

## 이 내용으로 만들 수 있는 것

- 모든 적만 찾아 피해를 주거나 플레이어를 제외한 Collider만 검사할 수 있습니다.
- `With`, `Without`, `Changed` filter로 필요한 Entity만 처리해 로직과 비용을 줄일 수 있습니다.

## 이번에 만들 결과물

Player 한 명과 Enemy 두 명을 만든 뒤, 모든 적을 반복해 피해를 주고 저장해 둔 Entity ID와 단일 대상 조건으로 다시 조회합니다. 이 과정에서 `iter_mut`, `get_mut`, `single`, `get`, `iter`를 모두 사용합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin query
```

## 핵심 개념

`Query<D, F>`는 조건에 맞는 Entity의 Component를 빌립니다.

- `D`: System이 실제로 받을 데이터
- `F`: 어떤 Entity를 포함하거나 제외할지 정하는 Filter

`&Health`는 읽기, `&mut Health`는 쓰기 접근입니다. 같은 System에서 동일 Component를 겹치게 빌리면 Rust의 별칭 규칙을 위반할 수 있으므로 `Without` 또는 `ParamSet`으로 집합이 분리된다는 사실을 알려야 합니다.

### 기반 타입 대신 Component 조건으로 찾기

객체지향에서는 `Character` 기반 타입이나 `Damageable` 인터페이스로 여러 종류의 객체를 한 컬렉션에 모으곤 합니다. ECS에서는 상위 타입으로 변환하지 않고 공통 Component 조합을 Query합니다.

```rust
// Health가 있는 모든 대상
Query<&Health>

// Health가 있는 적만
Query<&Health, With<Enemy>>

// Enemy이면서 Player는 아닌 대상만
Query<&Health, (With<Enemy>, Without<Player>)>
```

Rust trait을 구현했다고 그 구현체를 Bevy가 자동으로 한 Query에 모아 주는 것은 아닙니다. World에서 어떤 역할을 검색해야 한다면 marker나 데이터 Component로 그 조건을 명시합니다.

### 대상 수와 식별 방법에 따라 메서드 고르기

| 상황 | 메서드 | 결과 |
|---|---|---|
| 조건에 맞는 모든 대상을 읽는다 | `iter()` | 읽기 반복자 |
| 조건에 맞는 모든 대상을 변경한다 | `iter_mut()` | 변경 반복자 |
| 정확히 하나여야 하는 대상을 가져온다 | `single()` | 0개 또는 2개 이상이면 오류 |
| 알고 있는 Entity ID로 읽는다 | `get(entity)` | 대상이 없거나 Query 조건이 다르면 오류 |
| 알고 있는 Entity ID로 변경한다 | `get_mut(entity)` | 변경 가능한 값 또는 오류 |

`single()`과 `get()` 계열은 실패할 수 있으므로 샘플에서는 `let Ok(...) = ... else`로 오류를 처리합니다. 정확한 개수를 보장할 수 없다면 반복자를 사용하는 편이 안전합니다.

## 샘플 코드

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, Debug)]
struct Health(u32);

#[derive(Resource)]
struct Targets {
    player: Entity,
    first_enemy: Entity,
}

fn damage_enemies(
    mut enemies: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>,
) {
    for (entity, mut health) in enemies.iter_mut() {
        health.0 = health.0.saturating_sub(10);
        println!("{entity:?}의 남은 체력: {}", health.0);
    }
}

fn restore_player(targets: Res<Targets>, mut healths: Query<&mut Health>) {
    if let Ok(mut health) = healths.get_mut(targets.player) {
        health.0 = health.0.saturating_add(5);
    }
}

fn inspect_health(
    targets: Res<Targets>,
    player: Query<&Health, With<Player>>,
    all_health: Query<&Health>,
) {
    let Ok(player_health) = player.single() else {
        println!("Player는 정확히 한 명이어야 합니다.");
        return;
    };

    if let Ok(health) = all_health.get(targets.first_enemy) {
        println!("첫 번째 적 체력: {}", health.0);
    }

    let entity_count = all_health.iter().count();
    println!("Health를 가진 Entity: {entity_count}");
    println!("플레이어 체력: {}", player_health.0);
}
```

`setup`에서는 `(Player, Health(95))` 하나와 `(Enemy, Health(...))` 둘을 생성하고, 나중에 ID로 찾을 플레이어와 첫 번째 적을 `Targets` Resource에 저장합니다. 실행 샘플은 `damage_enemies`, `restore_player`, `inspect_health`를 `chain()`으로 연결합니다.

Entity ID의 숫자는 실행마다 달라질 수 있지만 출력의 값은 다음 흐름을 따릅니다.

```text
...의 남은 체력: 40
...의 남은 체력: 70
첫 번째 적 체력: 40
Health를 가진 Entity: 3
플레이어 체력: 100
```

## 코드 설명

- `(Entity, &mut Health)`는 ID와 수정 가능한 체력을 함께 가져옵니다.
- `With<Enemy>`는 Enemy가 있는 Entity만 포함합니다.
- `Without<Player>`는 Player가 있는 Entity를 제외합니다.
- 필터 튜플은 모든 조건을 동시에 만족해야 합니다.
- `saturating_sub(10)`은 0 아래로 내려갈 때 정수 오버플로 대신 0을 반환합니다.
- `iter_mut()`은 모든 적의 Health를 변경합니다.
- `get_mut(targets.player)`는 저장해 둔 Entity ID로 플레이어 체력 하나를 변경합니다.
- `single()`은 `With<Player>` 조건을 만족하는 Entity가 정확히 하나인지 함께 검사합니다.
- `get(targets.first_enemy)`는 저장해 둔 첫 번째 적 ID가 현재 Query에도 맞는지 확인하며 읽습니다.
- `iter().count()`는 Health를 가진 모든 Entity를 순회해 수를 셉니다.

## 실습 과제

1. 세 번째 Enemy를 체력 5로 생성하고 결과가 0인지 확인하세요.
2. `Without<Player>`를 제거하고 결과가 같은지 확인한 뒤, 왜 여전히 Player가 선택되지 않는지 설명하세요.
3. `Name` Component를 추가해 ID 대신 이름을 출력하세요.

## 심화 과제

`Changed<Health>` 필터를 사용하는 두 번째 System을 작성해 이번 프레임에 체력이 바뀐 대상만 출력하세요. 변경 감지가 값의 비교가 아니라 변경 접근을 추적한다는 점도 실험해 보세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/08_query.md)를 확인하세요.

## 다음 챕터

Entity에 속하지 않는 점수와 게임 규칙을 Resource로 관리합니다.
