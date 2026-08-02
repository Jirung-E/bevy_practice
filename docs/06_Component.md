# 06. Component: Entity에 데이터 붙이기

## 학습 목표

- Component를 작고 독립적인 데이터 단위로 설계할 수 있다.
- `#[derive(Component)]`로 사용자 Component를 정의할 수 있다.
- Bundle 문법으로 여러 Component를 한 Entity에 추가할 수 있다.
- 객체지향 상속과 ECS 조합의 차이를 설명할 수 있다.
- Required Components로 필수 구성을 보장할 수 있다.

## 이 내용으로 만들 수 있는 것

- 체력·위치·속도·팀처럼 Entity마다 다른 데이터를 조합할 수 있습니다.
- 같은 데이터에 `Player`와 `Enemy` marker를 붙여 서로 다른 규칙을 적용할 수 있습니다.

## 이번에 만들 결과물

Player 표식, 체력, 위치를 가진 플레이어 Entity를 생성합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin component
```

## 핵심 개념

> **Bevy ECS는 클래스 기반 객체지향 모델이 아닙니다.**
>
> Entity는 `Player` 클래스의 인스턴스가 아니라 World 안의 ID입니다. `Player`, `Health`, `Position` 같은 Component 조합이 그 Entity의 데이터와 역할을 나타냅니다. Component끼리 필드나 메서드를 상속하지 않으며, 동작은 주로 System이 Component 조합을 조회해 처리합니다.

Component는 Entity에 붙는 Rust 데이터입니다. 상속 계층으로 대상을 정의하는 대신 필요한 데이터의 조합으로 대상을 구성합니다.

### 객체지향 상속과 ECS 조합

객체지향 설계에서는 `Character` 기반 클래스를 `Player`와 `Enemy`가 상속하고, 공통 필드와 메서드를 물려받는 구조를 자주 사용합니다. Bevy ECS에서 Entity는 특정 클래스의 인스턴스가 아닙니다. Entity는 ID이고, 그 Entity에 붙은 Component 조합이 현재 역할과 기능을 결정합니다.

여기서 “클래스 기반 객체지향 모델이 아니다”라는 말은 `impl`이나 trait을 금지한다는 뜻이 아닙니다. Component 값 자체의 검증·계산·캡슐화에는 일반 Rust 메서드와 trait을 사용할 수 있습니다. 다만 게임 대상 전체를 하나의 클래스에 넣고 상속시키는 대신, World에서는 Entity와 Component 조합으로 표현합니다.

| 객체지향 방식 | Bevy ECS 방식 |
|---|---|
| 클래스 인스턴스가 대상이다 | `Entity`가 대상을 식별한다 |
| 클래스의 필드에 데이터를 저장한다 | 독립된 Component에 데이터를 저장한다 |
| 기반 클래스에서 공통 필드와 메서드를 상속한다 | 필요한 Entity에 공통 Component를 각각 붙인다 |
| 파생 클래스로 역할을 구분한다 | `Player`, `Enemy` 같은 marker Component로 역할을 표시한다 |
| 객체의 메서드가 동작을 처리한다 | System이 Component 조합을 조회해 처리한다 |
| 생성자가 필수 필드를 초기화한다 | Bundle, 생성 함수, Required Components로 구성을 만든다 |
| 실행 중 객체의 클래스를 바꾸기 어렵다 | Component를 추가·제거해 역할을 바꿀 수 있다 |
| 기반 타입이나 인터페이스로 대상을 모은다 | Query의 데이터와 Filter로 대상을 선택한다 |

예를 들어 객체지향의 `Player : Character`, `Enemy : Character`를 그대로 옮기려고 하지 않습니다. 공통 역할과 데이터를 별도 Component로 나눕니다.

```text
Player Entity                 Enemy Entity
├─ Character                 ├─ Character
├─ Player                    ├─ Enemy
├─ Health                    ├─ Health
└─ Position                  └─ Position
```

`Player`가 `Character`의 필드나 메서드를 물려받은 것은 아닙니다. 같은 Entity에 두 Component가 각각 붙어 있을 뿐입니다. 덕분에 캐릭터가 아닌 파괴 가능한 상자에도 `Health`만 붙여 같은 체력 규칙을 재사용할 수 있습니다.

```text
Destructible Crate Entity
├─ Destructible
├─ Health
└─ Position
```

Component 조합은 실행 중에도 바뀔 수 있습니다. 예를 들어 독에 걸릴 때 `Poisoned`를 추가하고 치료될 때 제거하면, `With<Poisoned>`를 사용하는 System의 처리 대상도 함께 바뀝니다. 실제 추가·제거는 [10. Commands](10_Commands.md)에서 다룹니다.

- `Player`: 값이 없는 표식(marker) Component
- `Health(u32)`: 체력만 책임지는 새 타입
- `Position { x, y }`: 2차원 위치 데이터

플레이어와 적이 모두 Health를 가질 수 있으므로 체력 감소 System을 재사용할 수 있습니다. 데이터를 작게 나누면 Query도 필요한 항목만 빌릴 수 있어 System 병렬 실행에 유리합니다.

### Required Components는 상속이 아니다

특정 Component를 넣을 때 반드시 함께 있어야 하는 구성이 있다면 `#[require(...)]`를 사용할 수 있습니다.

```rust
#[derive(Component, Default)]
struct Character;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
#[require(Character, Health(100), Position { x: 0.0, y: 0.0 })]
struct Player;
```

이제 `Player`를 삽입하면 빠진 `Character`, `Health`, `Position`도 함께 초기화됩니다.

```rust
commands.spawn(Player);
commands.spawn((Player, Health(250))); // 직접 제공한 Health가 우선한다.
```

Required Components는 `Player`가 `Character`를 상속한다는 뜻이 아닙니다. 별개의 Component를 같은 Entity에 넣어 유효한 구성을 보장하는 기능입니다. Bevy의 카메라·조명처럼 항상 Transform이나 가시성 데이터가 필요한 기능도 이 방식으로 필요한 구성을 선언합니다.

Required Components는 빠진 값을 자동으로 채우는 기본 규칙입니다. Entity마다 초기값 전체가 달라져야 한다면 Bundle이나 생성 함수에 값을 명시하는 편이 더 분명합니다.

## 샘플 코드

```rust
use bevy::prelude::*;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Health(u32);

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_player);
    app.update();
}

fn spawn_player(mut commands: Commands) {
    let player = commands
        .spawn((Player, Health(100), Position { x: 0.0, y: 0.0 }))
        .id();

    println!("Component를 가진 플레이어 생성: {player:?}");
}
```

## 코드 설명

- `#[derive(Component)]`는 타입을 Bevy ECS 저장소에 넣을 수 있게 합니다.
- 필드가 없는 `Player`는 대상을 분류하는 데 쓰며 별도 값을 저장하지 않습니다.
- tuple struct인 `Health(u32)`는 원시 숫자를 체력이라는 도메인 타입으로 구분합니다.
- `commands.spawn((...))`의 튜플은 여러 Component를 한 번에 추가하는 Bundle로 동작합니다.
- 하나의 Entity에는 같은 타입의 Component를 하나만 가질 수 있습니다.
- Component는 다른 Component의 필드와 메서드를 상속하지 않습니다.

이 예제의 `Position`은 ECS 설명용입니다. 화면에 그릴 실제 2D/3D 대상의 위치에는 Bevy의 `Transform`을 사용하게 됩니다.

## 실습 과제

1. `Name(String)` Component를 추가하세요.
2. `Mana(u32)`를 추가하고 초기값을 50으로 설정하세요.
3. 플레이어와 적을 구분할 수 있도록 적절한 marker Component를 설계하세요.
4. 플레이어와 적이 `Health`, `Position`을 공유하면서도 서로 다른 대상으로 분류되도록 두 Entity의 Component 조합을 구성하세요.

## 심화 과제

플레이어 생성에 필요한 Component 조합을 반환하는 `fn player_bundle() -> impl Bundle` 함수를 작성하세요. 이후 초기값이 다른 두 플레이어를 만들려면 어떤 매개변수가 필요한지도 생각해 보세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/06_component.md)를 확인하세요.

## 다음 챕터

System을 추가해 Component 데이터를 읽고 플레이어의 위치를 변경합니다.
