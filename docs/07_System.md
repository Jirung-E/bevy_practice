# 07. System: 데이터에 로직 적용하기

## 학습 목표

- 일반 Rust 함수가 Bevy System이 되는 조건을 이해한다.
- `Startup`과 `Update` 스케줄의 차이를 구분한다.
- System 실행 순서를 명시적으로 연결할 수 있다.
- 객체 메서드와 ECS System의 역할 차이를 설명할 수 있다.

## 이 내용으로 만들 수 있는 것

- 입력에 따른 이동, 체력 회복, 점수 계산처럼 매 프레임 실행되는 게임 규칙을 만들 수 있습니다.
- 서로 독립적인 로직을 작은 System으로 나눠 테스트하고 실행 순서를 구성할 수 있습니다.

## 이번에 만들 결과물

플레이어의 Position에 Velocity를 더하고, 변경된 위치를 두 프레임 동안 출력합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin system
```

## 핵심 개념

System은 World의 데이터를 입력으로 받는 Rust 함수입니다. Bevy는 함수 매개변수의 타입을 보고 어떤 데이터에 읽기·쓰기 접근하는지 판단합니다. 서로 충돌하지 않는 System은 병렬 실행할 수 있습니다.

### 객체의 메서드에서 데이터 조합을 처리하는 System으로

객체지향 코드에서는 대상을 찾은 뒤 그 객체의 메서드를 호출하는 형태가 흔합니다.

```rust,ignore
player.move_forward();
enemy.take_damage(10);
```

ECS에서는 동작을 특정 클래스 안에 묶기보다, 필요한 Component 조합을 매개변수로 받는 System으로 작성합니다. 다음 이동 System은 `Player`라는 객체의 메서드가 아니라 `Player + Position + Velocity` 조합에 적용되는 규칙입니다.

```rust
fn move_player(mut players: Query<(&mut Position, &Velocity), With<Player>>) {
    for (mut position, velocity) in &mut players {
        position.0 += velocity.0;
    }
}
```

이 구조에서는 `Health`를 가진 플레이어·적·파괴 가능한 상자를 하나의 피해 System으로 처리할 수 있습니다. 새 대상 종류를 추가할 때 기존 기반 클래스를 수정하지 않고 필요한 Component를 조합하면 됩니다.

Bevy ECS가 클래스 기반 객체지향 모델이 아니라고 해서 Component에 `impl`을 작성하거나 Rust trait을 사용하면 안 된다는 뜻은 아닙니다. 값 검증, 거리 계산처럼 World 접근이 필요 없는 작은 연산은 일반 Rust 메서드로 둘 수 있습니다. 여러 Entity나 Resource를 읽고 게임의 실행 흐름에 참여하는 로직은 System이 담당하는 편이 자연스럽습니다.

스케줄은 System이 실행될 시점을 정합니다.

- `Startup`: 앱 시작 시 한 번
- `Update`: 앱이 업데이트될 때마다

System 등록 순서는 실행 순서를 보장하지 않습니다. 결과를 출력하기 전에 반드시 이동이 끝나야 하므로 `(move_player, print_position).chain()`으로 순서를 지정합니다.

## 샘플 코드

```rust
#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Position(f32);

#[derive(Component)]
struct Velocity(f32);

fn move_player(mut players: Query<(&mut Position, &Velocity), With<Player>>) {
    for (mut position, velocity) in &mut players {
        position.0 += velocity.0;
    }
}

fn print_position(players: Query<&Position, With<Player>>) {
    for position in &players {
        println!("플레이어 위치: {position:?}");
    }
}

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, setup)
        .add_systems(Update, (move_player, print_position).chain());

    app.update();
    app.update();
}
```

전체 파일에는 `setup` System도 포함되어 있습니다.

## 코드 설명

- `Query<(&mut Position, &Velocity)>`는 Position을 쓰고 Velocity를 읽습니다.
- `With<Player>`는 Player 표식이 붙은 Entity만 선택합니다.
- `for ... in &mut players`는 조건에 맞는 모든 Entity를 순회합니다.
- System은 `Player`의 메서드를 호출하는 대신 필요한 Component 조합에 동작을 적용합니다.
- `chain()`은 앞 System의 deferred 명령 적용까지 포함해 순차 실행합니다.
- 수동으로 `app.update()`를 두 번 호출했으므로 위치가 2.5, 5.0으로 바뀝니다.

실제 게임에서는 매 프레임의 이동량을 `velocity * time.delta_secs()`처럼 시간과 함께 계산합니다.

## 실습 과제

1. Velocity를 10.0으로 바꾸고 출력을 예상한 뒤 실행하세요.
2. `app.update()`를 한 번 더 호출하세요.
3. Y축을 추가해 Position과 Velocity를 2차원 구조체로 바꾸세요.

## 심화 과제

이동과 출력 사이에 `clamp_position` System을 추가해 위치를 `-10.0..=10.0`으로 제한하세요. 세 System을 `chain()`으로 연결하고 충분한 프레임을 실행해 제한이 작동하는지 확인하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/07_system.md)를 확인하세요.

## 다음 챕터

Query의 읽기·쓰기, 튜플, Filter를 더 자세히 사용해 적만 골라 체력을 감소시킵니다.
