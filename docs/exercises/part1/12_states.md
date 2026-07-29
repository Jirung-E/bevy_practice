# 12. States 과제 해설

[본문으로 돌아가기](../../12_States.md#실습-과제)

## P1-C12-P1 · Paused 상태

`GameState`에 `Paused` variant를 추가하고 `OnEnter(GameState::Paused)` System을 등록합니다. Playing과 Paused 사이를 왕복할 수 있어야 하므로 단방향 진행 단계로만 취급하지 않습니다.

## P1-C12-P2 · OnExit 등록

### 힌트

각 상태의 종료 처리를 명시하면 어떤 화면이 언제 정리되는지 로그로 확인하기 쉽습니다.

```rust
app.add_systems(OnExit(GameState::Menu), exit_menu)
    .add_systems(OnExit(GameState::Playing), exit_playing)
    .add_systems(OnExit(GameState::Paused), exit_paused);
```

## P1-C12-P3 · 현재 State 읽기

```rust
fn print_state(state: Res<State<GameState>>) {
    println!("현재 상태: {:?}", state.get());
}
```

`NextState`에 값을 설정한 System 안에서는 아직 `State`가 바뀌지 않았습니다. 전환은 StateTransition 스케줄에서 적용되므로 다음 프레임과 진입·이탈 로그를 함께 확인하세요.

## P1-C12-A1 · 상태별 Entity 정리

### 접근 방법

상태 진입 시 생성하는 Entity에 `DespawnOnExit(해당 상태)`를 붙입니다.

```rust
commands.spawn((
    ScreenEntity,
    DespawnOnExit(GameState::Menu),
));
```

### 확인 기준

- Menu와 Playing 진입마다 화면 Entity가 하나 생성된다.
- 상태를 떠나면 이전 화면 Entity가 제거된다.
- 여러 번 왕복해도 `ScreenEntity` 수가 누적되지 않는다.

명시적인 OnExit despawn도 가능하지만, 상태 소유권이 명확한 화면·UI에는 `DespawnOnExit`가 누락을 줄여 줍니다. 게임 진행 데이터처럼 상태 전환 뒤에도 유지되어야 하는 Entity에는 붙이면 안 됩니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin states_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/states_solution.rs`

