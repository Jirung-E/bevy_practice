use bevy::prelude::*;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Enemy;

#[derive(Component, Debug)]
struct Name(String);

#[derive(Component, Debug)]
struct Health(u32);

#[derive(Component, Debug)]
struct Mana(u32);

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

type EnemyFilter = (With<Enemy>, Without<Player>);

fn player_bundle(
    name: impl Into<String>,
    health: u32,
    mana: u32,
    position: Position,
) -> impl Bundle {
    (
        Player,
        Name(name.into()),
        Health(health),
        Mana(mana),
        position,
    )
}

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, (setup, print_entities).chain());
    app.update();
}

fn setup(mut commands: Commands) {
    commands.spawn(player_bundle(
        "Player One",
        100,
        50,
        Position { x: 0.0, y: 0.0 },
    ));
    commands.spawn(player_bundle(
        "Player Two",
        80,
        70,
        Position { x: -4.0, y: 2.0 },
    ));
    commands.spawn((
        Enemy,
        Name("Training Dummy".to_owned()),
        Health(40),
        Position { x: 8.0, y: 3.0 },
    ));
}

fn print_entities(
    players: Query<(&Name, &Health, &Mana, &Position), With<Player>>,
    enemies: Query<(&Name, &Health, &Position), EnemyFilter>,
) {
    for (name, health, mana, position) in &players {
        println!(
            "플레이어 {}: HP {}, MP {}, 위치 ({}, {})",
            name.0, health.0, mana.0, position.x, position.y
        );
    }

    for (name, health, position) in &enemies {
        println!(
            "적 {}: HP {}, 위치 ({}, {})",
            name.0, health.0, position.x, position.y
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn players_and_enemy_share_data_but_have_different_markers() {
        let mut app = App::new();
        app.add_systems(Startup, setup);
        app.update();

        let world = app.world_mut();

        let players = world
            .query_filtered::<(&Name, &Health, &Mana, &Position), With<Player>>()
            .iter(world)
            .map(|(name, health, mana, position)| {
                (name.0.as_str(), health.0, mana.0, position.x, position.y)
            })
            .collect::<Vec<_>>();

        assert_eq!(
            players,
            vec![
                ("Player One", 100, 50, 0.0, 0.0),
                ("Player Two", 80, 70, -4.0, 2.0),
            ]
        );

        let enemies = world
            .query_filtered::<(&Name, &Health, &Position), (With<Enemy>, Without<Player>)>()
            .iter(world)
            .map(|(name, health, position)| (name.0.as_str(), health.0, position.x, position.y))
            .collect::<Vec<_>>();

        assert_eq!(enemies, vec![("Training Dummy", 40, 8.0, 3.0)]);
    }
}
