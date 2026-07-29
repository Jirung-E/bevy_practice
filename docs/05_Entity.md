# 05. Entity: 월드의 대상 만들기

## 학습 목표

- Entity가 데이터가 아닌 식별자라는 사실을 설명할 수 있다.
- `Commands`로 Entity를 생성하고 ID를 얻을 수 있다.
- Entity ID를 장기간 저장할 때 주의할 점을 이해한다.

## 이번에 만들 결과물

화면 없이 한 프레임만 실행되는 ECS 실험실을 만듭니다. 플레이어와 적을 나타낼 빈 Entity 두 개를 생성하고 서로 다른 ID를 출력합니다.

아래 명령은 이 교재 저장소에 포함된 `05_entity.rs` 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 작성한 코드를 `src/main.rs`에 넣고 `cargo run`을 사용하세요.

```bash
cargo run -p ecs_basics --bin entity
```

화면에 창은 열리지 않고 터미널에 다음과 같은 두 줄이 출력됩니다.

```text
플레이어 Entity: 2v0
적 Entity: 3v0
```

ID의 숫자는 실행 환경과 Bevy 내부 Entity 생성 순서에 따라 달라질 수 있습니다. 두 값이 서로 다르다는 점만 확인하면 됩니다.

## 핵심 개념

Bevy의 모든 ECS 데이터는 `World`에 저장됩니다. Entity는 World 안의 대상을 가리키는 가벼운 ID입니다. Entity 자체에는 이름, 위치, 체력 같은 의미가 없습니다. 다음 챕터에서 Component를 붙여 의미를 구성합니다.

Entity ID는 인덱스와 세대(generation)를 함께 사용합니다. 제거된 Entity의 인덱스가 나중에 재사용되더라도 이전 ID가 새 Entity를 잘못 가리키지 않게 하기 위해서입니다. 따라서 출력된 숫자의 크기나 생성 순서에 게임 규칙을 의존하면 안 됩니다.

## 샘플 코드

전체 코드: `examples/part1/ecs_basics/src/bin/05_entity.rs`

```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_entities);
    app.update();
}

fn spawn_entities(mut commands: Commands) {
    let player = commands.spawn_empty().id();
    let enemy = commands.spawn_empty().id();

    println!("플레이어 Entity: {player:?}");
    println!("적 Entity: {enemy:?}");
}
```

## 코드 설명

- `App::new()`은 빈 App과 World를 만듭니다.
- `Startup` System은 첫 `update()`에서 한 번 실행됩니다.
- `Commands::spawn_empty()`는 Component가 없는 Entity 생성을 예약합니다.
- `.id()`는 예약된 Entity의 ID를 즉시 돌려줍니다.
- System이 끝날 때 예약된 명령이 World에 적용됩니다.

실제 게임에서는 Entity ID보다 `Player`, `Enemy` 같은 Component로 대상을 검색하는 방식을 우선합니다. ID는 특정 대상 하나를 계속 참조해야 할 때만 저장하세요.

Bevy 0.19는 System과 Observer 같은 내부 실행 단위도 Entity로 관리합니다. 따라서 `world.entities().len()`을 게임 오브젝트 수로 사용하지 말고, 필요한 Component를 가진 Entity를 Query로 세어야 합니다.

## 실습 과제

1. `npc` Entity를 하나 더 생성하고 세 ID를 출력하세요.
2. `player != enemy && player != npc && enemy != npc`의 결과를 출력해 세 ID가 모두 다른지 확인하세요.
3. 플레이어와 적을 생성하는 두 줄의 순서를 바꾸어 실행하세요. 변수 이름이 같아도 출력되는 ID 순서는 달라질 수 있다는 점을 기록하세요.

## 심화 과제

빈 Entity를 다섯 개 생성하고 각 ID를 `Vec<Entity>`에 저장한 뒤 `for` 반복문으로 출력하세요. ID를 숫자로 해석하지 않고 “현재 World 안의 대상을 가리키는 값”으로만 다루어야 하는 이유를 한 문장으로 설명하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/05_entity.md)를 확인하세요.

## 다음 챕터

빈 Entity에 Player, Health, Position Component를 붙여 실제 게임 대상을 표현합니다.
