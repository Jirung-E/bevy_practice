use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    let mut app = build_app();
    app.update();
    app.update();
    app.update();
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_systems(Startup, setup).add_systems(
        Update,
        (move_player, clamp_position, print_position).chain(),
    );
    app
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 10.0, y: -6.0 },
    ));
}

fn move_player(mut players: Query<(&mut Position, &Velocity), With<Player>>) {
    for (mut position, velocity) in &mut players {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn clamp_position(mut players: Query<&mut Position, With<Player>>) {
    for mut position in &mut players {
        position.x = position.x.clamp(-10.0, 10.0);
        position.y = position.y.clamp(-10.0, 10.0);
    }
}

fn print_position(players: Query<&Position, With<Player>>) {
    for position in &players {
        println!("플레이어 위치: ({}, {})", position.x, position.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_is_clamped_after_multiple_updates() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();

        let world = app.world_mut();
        let mut query = world.query_filtered::<&Position, With<Player>>();
        let position = query.single(world).expect("플레이어가 하나 있어야 한다");

        assert_eq!(*position, Position { x: 10.0, y: -10.0 });
    }
}
