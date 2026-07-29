use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Component)]
struct ScreenEntity;

#[derive(Resource, Default)]
struct FlowStep(u8);

#[derive(Resource, Default)]
struct ExitLog(Vec<GameState>);

fn main() {
    let mut app = build_app();
    for _ in 0..7 {
        app.update();
    }
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<GameState>()
        .init_resource::<FlowStep>()
        .init_resource::<ExitLog>()
        .add_systems(OnEnter(GameState::Menu), enter_menu)
        .add_systems(OnExit(GameState::Menu), exit_menu)
        .add_systems(OnEnter(GameState::Playing), enter_playing)
        .add_systems(OnExit(GameState::Playing), exit_playing)
        .add_systems(OnEnter(GameState::Paused), enter_paused)
        .add_systems(OnExit(GameState::Paused), exit_paused)
        .add_systems(OnEnter(GameState::GameOver), enter_game_over)
        .add_systems(Update, (advance_flow, print_state).chain());
    app
}

fn enter_menu(mut commands: Commands) {
    commands.spawn((ScreenEntity, DespawnOnExit(GameState::Menu)));
}

fn enter_playing(mut commands: Commands) {
    commands.spawn((ScreenEntity, DespawnOnExit(GameState::Playing)));
}

fn enter_paused(mut commands: Commands) {
    commands.spawn((ScreenEntity, DespawnOnExit(GameState::Paused)));
}

fn enter_game_over(mut commands: Commands) {
    commands.spawn((ScreenEntity, DespawnOnExit(GameState::GameOver)));
}

fn exit_menu(mut log: ResMut<ExitLog>) {
    log.0.push(GameState::Menu);
}

fn exit_playing(mut log: ResMut<ExitLog>) {
    log.0.push(GameState::Playing);
}

fn exit_paused(mut log: ResMut<ExitLog>) {
    log.0.push(GameState::Paused);
}

fn advance_flow(
    state: Res<State<GameState>>,
    mut step: ResMut<FlowStep>,
    mut next: ResMut<NextState<GameState>>,
) {
    let target = match (state.get(), step.0) {
        (GameState::Menu, 0) => Some(GameState::Playing),
        (GameState::Playing, 1) => Some(GameState::Paused),
        (GameState::Paused, 2) => Some(GameState::Playing),
        (GameState::Playing, 3) => Some(GameState::GameOver),
        _ => None,
    };
    if let Some(target) = target {
        step.0 += 1;
        next.set(target);
    }
}

fn print_state(state: Res<State<GameState>>) {
    println!("현재 상태: {:?}", state.get());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_scoped_entities_do_not_accumulate_during_transitions() {
        let mut app = build_app();

        for _ in 0..7 {
            app.update();
            let world = app.world_mut();
            let mut query = world.query_filtered::<(), With<ScreenEntity>>();
            let count = query.iter(world).count();
            assert!(count <= 1, "상태 화면 Entity가 누적되면 안 된다");
        }

        assert_eq!(
            app.world().resource::<ExitLog>().0,
            vec![
                GameState::Menu,
                GameState::Playing,
                GameState::Paused,
                GameState::Playing,
            ]
        );
        assert_eq!(
            *app.world().resource::<State<GameState>>().get(),
            GameState::GameOver
        );
    }
}
