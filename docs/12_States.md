# 12. States: 화면과 흐름 관리하기

## 학습 목표

- States로 애플리케이션의 상호 배타적인 흐름을 모델링할 수 있다.
- `OnEnter`, `OnExit`, `run_if(in_state(...))`를 사용할 수 있다.
- `NextState`의 전환 적용 시점을 이해한다.

## 이 내용으로 만들 수 있는 것

- 로딩, 타이틀, 플레이, 일시정지와 게임오버 화면 흐름을 관리할 수 있습니다.
- 특정 State에서만 Entity와 System이 존재하게 해 화면 전환 뒤 데이터가 누적되는 문제를 막을 수 있습니다.

## 이번에 만들 결과물

Menu에서 시작해 Playing을 거쳐 GameOver로 전환되는 작은 상태 머신을 만들고 각 상태 진입을 출력합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin states
```

## 핵심 개념

게임에는 메뉴, 플레이, 일시정지, 게임오버처럼 특정 시점에 하나만 활성화되는 흐름이 있습니다. 모든 System 안에 `if`를 반복하는 대신 States와 스케줄로 실행 조건을 선언할 수 있습니다.

- `OnEnter(S)`: 상태 S에 들어갈 때 한 번
- `OnExit(S)`: 상태 S에서 나갈 때 한 번
- `Update` + `run_if(in_state(S))`: S인 매 프레임

상태 변경은 `NextState<S>`에 예약하며 StateTransition 스케줄에서 적용됩니다. 현재 System 안에서 `State<S>`가 즉시 바뀐다고 가정하면 안 됩니다.

## 샘플 코드

```rust
use bevy::state::app::StatesPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Menu), enter_menu)
        .add_systems(Update, start_game.run_if(in_state(GameState::Menu)))
        .add_systems(OnEnter(GameState::Playing), enter_game)
        .add_systems(Update, end_game.run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), enter_game_over);

    app.update();
    app.update();
    app.update();
}
```

## 코드 설명

- `States`에는 비교와 해시에 필요한 trait도 함께 derive합니다.
- `#[default]`인 Menu가 초기 상태입니다.
- `StatesPlugin`은 상태 전환 스케줄을 App에 추가합니다. `DefaultPlugins`를 쓰는 일반 게임에는 이미 포함되지만 이 콘솔 예제에서는 명시적으로 등록합니다.
- `init_state`는 `State<GameState>`와 `NextState<GameState>`를 준비합니다.
- `run_if(in_state(...))`는 System을 스케줄에 남겨 두고 조건이 참일 때만 실행합니다.
- 상태별 생성물에는 상태 범위 despawn 기능을 적용할 수 있습니다. Part 2의 메뉴와 게임오버 UI에서 사용합니다.

일시정지처럼 Playing과 동시에 표현될 수 있는 개념은 별도 State나 Resource로 분리하는 편이 좋습니다. 하나의 거대한 enum에 모든 조합을 넣으면 상태 수가 폭발합니다.

## 실습 과제

1. `Paused` 상태를 추가하고 진입 메시지를 출력하세요.
2. 각 상태에 `OnExit` System을 등록하세요.
3. `State<GameState>`를 읽어 현재 상태를 출력하는 System을 만드세요.

## 심화 과제

Menu와 Playing에서 각각 Entity를 생성하고 상태를 떠날 때 자동 또는 명시적으로 제거하세요. 상태를 여러 번 오가도 Entity 수가 계속 늘어나지 않는지 검사하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/12_states.md)를 확인하세요.

## 다음 챕터

Part 1에서 배운 Entity, Component, System, Query, Resource, Commands, Message, States를 결합해 키보드로 움직이는 2D 플레이어를 만듭니다.
