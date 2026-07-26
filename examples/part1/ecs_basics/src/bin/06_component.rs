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
    let health = Health(100);
    let position = Position { x: 0.0, y: 0.0 };
    println!(
        "초기 데이터: health={}, position=({}, {})",
        health.0, position.x, position.y
    );

    let player = commands.spawn((Player, health, position)).id();

    println!("Component를 가진 플레이어 생성: {player:?}");
}
