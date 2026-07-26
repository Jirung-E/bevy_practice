use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Position(f32);

#[derive(Component)]
struct Velocity(f32);

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, setup)
        .add_systems(Update, (move_player, print_position).chain());

    app.update();
    app.update();
}

fn setup(mut commands: Commands) {
    commands.spawn((Player, Position(0.0), Velocity(2.5)));
}

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
