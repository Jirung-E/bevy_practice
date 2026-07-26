use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
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

fn enter_menu() {
    println!("상태 진입: Menu");
}

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    println!("Playing 상태 전환 예약");
    next_state.set(GameState::Playing);
}

fn enter_game() {
    println!("상태 진입: Playing");
}

fn end_game(mut next_state: ResMut<NextState<GameState>>) {
    println!("GameOver 상태 전환 예약");
    next_state.set(GameState::GameOver);
}

fn enter_game_over() {
    println!("상태 진입: GameOver");
}
