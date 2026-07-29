use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Name(&'static str);

#[derive(Component, Debug)]
struct Health(u32);

type EnemyFilter = (With<Enemy>, Without<Player>);
type ChangedEnemyFilter = (With<Enemy>, Changed<Health>);

fn main() {
    let mut app = build_app();
    app.update();
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_systems(Startup, setup)
        .add_systems(Update, (damage_enemies, report_changed).chain());
    app
}

fn setup(mut commands: Commands) {
    commands.spawn((Player, Name("Player"), Health(100)));
    commands.spawn((Enemy, Name("Scout"), Health(50)));
    commands.spawn((Enemy, Name("Tank"), Health(80)));
    commands.spawn((Enemy, Name("Weak"), Health(5)));
}

fn damage_enemies(mut enemies: Query<(&Name, &mut Health), EnemyFilter>) {
    for (name, mut health) in &mut enemies {
        health.0 = health.0.saturating_sub(10);
        println!("{}의 남은 체력: {}", name.0, health.0);
    }
}

fn report_changed(enemies: Query<(&Name, &Health), ChangedEnemyFilter>) {
    for (name, health) in &enemies {
        println!("이번 프레임에 변경됨: {} = {}", name.0, health.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_enemies_are_damaged_and_low_health_saturates_at_zero() {
        let mut app = build_app();
        app.update();

        let world = app.world_mut();
        let mut query = world.query::<(&Name, &Health)>();
        let mut values = query
            .iter(world)
            .map(|(name, health)| (name.0, health.0))
            .collect::<Vec<_>>();
        values.sort_unstable();

        assert_eq!(
            values,
            vec![("Player", 100), ("Scout", 40), ("Tank", 70), ("Weak", 0)]
        );
    }
}
