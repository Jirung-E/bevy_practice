# 06. Component 과제 해설

[본문으로 돌아가기](../../06_Component.md#실습-과제)

먼저 본문의 과제를 직접 시도하세요. 아래 내용은 한 가지 수행 예시이며, 타입 이름이나 초기값을 구성하는 다른 방식도 요구사항을 만족할 수 있습니다.

## P1-C06-P1 · Name Component

### 힌트

`Health(u32)`와 같은 tuple struct를 만들되 문자열을 소유해야 하므로 필드 타입으로 `String`을 사용합니다.

### 확인 기준

- `Name`에 `Component`가 derive되어 있다.
- 플레이어 Entity에 이름이 들어 있다.
- 이름을 출력하거나 Query로 읽을 수 있다.

### 수행 예시

```rust
#[derive(Component, Debug)]
struct Name(String);

commands.spawn((
    Player,
    Name("Player One".to_owned()),
    Health(100),
    Position { x: 0.0, y: 0.0 },
));
```

## P1-C06-P2 · Mana Component

### 힌트

체력과 마나는 둘 다 `u32`를 저장하지만 서로 다른 의미를 가지므로 별도 새 타입으로 정의합니다.

### 확인 기준

- `Mana(u32)`가 별도 Component다.
- 플레이어의 초기 마나가 50이다.
- `Health`와 `Mana`를 같은 Entity에 동시에 넣을 수 있다.

### 수행 예시

```rust
#[derive(Component, Debug)]
struct Mana(u32);

commands.spawn((Player, Health(100), Mana(50)));
```

원시 `u32`를 Component로 직접 사용하지 않고 새 타입으로 감싸면 Query에서 체력과 마나를 타입으로 구분할 수 있습니다.

## P1-C06-P3 · 적 marker 설계

### 힌트

`Player`가 값을 저장하지 않고 대상을 분류하는 것처럼 적도 필드 없는 marker Component로 표현할 수 있습니다.

### 확인 기준

- 플레이어와 적을 타입 수준에서 구분할 수 있다.
- `Health`나 `Position`에 `is_enemy` 같은 분류 필드를 추가하지 않는다.
- 이후 `With<Enemy>` 또는 `Without<Player>` 같은 Query 필터를 사용할 수 있다.

### 수행 예시

```rust
#[derive(Component, Debug)]
struct Enemy;
```

enum 하나로 모든 종류를 표현하는 방식도 가능하지만, marker는 여러 역할을 조합하거나 Query 필터로 사용할 때 단순합니다.

## P1-C06-P4 · 공유 데이터와 분류 조합

### 힌트

플레이어와 적은 `Health`, `Position`을 공유합니다. 두 대상의 차이는 공유 데이터가 아니라 marker의 조합으로 나타냅니다.

### 확인 기준

- 플레이어에는 `Player`, 적에는 `Enemy`가 있다.
- 두 Entity 모두 `Health`와 `Position`을 가진다.
- 적 Entity에 `Player`가 붙지 않는다.

### 수행 예시

```rust
commands.spawn((
    Player,
    Name("Player One".to_owned()),
    Health(100),
    Mana(50),
    Position { x: 0.0, y: 0.0 },
));

commands.spawn((
    Enemy,
    Name("Training Dummy".to_owned()),
    Health(40),
    Position { x: 8.0, y: 3.0 },
));
```

본문의 기존 플레이어 코드가 과제의 출발점이고, 위 코드는 적 marker와 공유 Component 조합을 추가한 결과입니다. 다음 Query 챕터에서는 이런 조합을 필터로 읽습니다.

## P1-C06-A1 · Bundle 함수

### 접근 방법

1. 플레이어를 구성하는 Component 타입을 먼저 정합니다.
2. 호출할 때 달라져야 하는 이름, 체력, 마나, 위치를 매개변수로 받습니다.
3. 튜플을 반환해 `commands.spawn(player_bundle(...))`에 넘깁니다.

### 확인 기준

- 생성 System에 Component 조합이 반복되지 않는다.
- 서로 다른 초기값을 가진 플레이어를 두 명 만들 수 있다.
- 함수가 `Commands`를 직접 받지 않고 Bundle 구성만 책임진다.

### 수행 예시

```rust
fn player_bundle(
    name: impl Into<String>,
    health: u32,
    mana: u32,
    position: Position,
) -> impl Bundle {
    (
        Player,
        Name(name.into()),
        Health(health),
        Mana(mana),
        position,
    )
}
```

작은 예제에서는 튜플 반환이 간결합니다. 실제 프로젝트에서 같은 조합이 여러 모듈의 공개 계약이 된다면 이름 있는 `#[derive(Bundle)]` 타입을 사용하는 편이 필드 의미와 문서화를 분명하게 만듭니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin component_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/component_solution.rs`

