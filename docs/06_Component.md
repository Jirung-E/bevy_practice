# 06. Component: Entity에 데이터 붙이기

## 학습 목표

- Component를 작고 독립적인 데이터 단위로 설계할 수 있다.
- `#[derive(Component)]`로 사용자 Component를 정의할 수 있다.
- Bundle 문법으로 여러 Component를 한 Entity에 추가할 수 있다.

## 이번에 만들 결과물

Player 표식, 체력, 위치를 가진 플레이어 Entity를 생성합니다.

```bash
cargo run -p ecs_basics --bin component
```

## 핵심 개념

Component는 Entity에 붙는 Rust 데이터입니다. 상속 계층으로 대상을 정의하는 대신 필요한 데이터의 조합으로 대상을 구성합니다.

- `Player`: 값이 없는 표식(marker) Component
- `Health(u32)`: 체력만 책임지는 새 타입
- `Position { x, y }`: 2차원 위치 데이터

플레이어와 적이 모두 Health를 가질 수 있으므로 체력 감소 System을 재사용할 수 있습니다. 데이터를 작게 나누면 Query도 필요한 항목만 빌릴 수 있어 System 병렬 실행에 유리합니다.

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

이 예제의 `Position`은 ECS 설명용입니다. 화면에 그릴 실제 2D/3D 대상의 위치에는 Bevy의 `Transform`을 사용하게 됩니다.

## 실습 과제

1. `Name(String)` Component를 추가하세요.
2. `Mana(u32)`를 추가하고 초기값을 50으로 설정하세요.
3. Player가 없는 적 Entity를 `Health`와 `Position` 조합으로 생성하세요.

## 심화 과제

플레이어 생성에 필요한 Component 조합을 반환하는 `fn player_bundle() -> impl Bundle` 함수를 작성하세요. 이후 초기값이 다른 두 플레이어를 만들려면 어떤 매개변수가 필요한지도 생각해 보세요.

## 다음 챕터

System을 추가해 Component 데이터를 읽고 플레이어의 위치를 변경합니다.

