use bevy::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Component, Debug)]
struct Health(u32);

#[derive(Resource, Default, Debug)]
struct Defeated(u32);

fn main() {
    let mut app = test_app();
    app.world_mut().spawn((Enemy, Health(5)));
    app.update();

    println!("처치 수: {}", app.world().resource::<Defeated>().0);
}

fn apply_damage(health: u32, damage: u32) -> u32 {
    health.saturating_sub(damage)
}

fn test_app() -> App {
    let mut app = App::new();
    app.init_resource::<Defeated>()
        .add_systems(Update, damage_enemies);
    app
}

fn damage_enemies(mut enemies: Query<&mut Health, With<Enemy>>, mut defeated: ResMut<Defeated>) {
    for mut health in &mut enemies {
        health.0 = apply_damage(health.0, 10);
        if health.0 == 0 {
            defeated.0 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_rule_is_tested_without_bevy_app() {
        assert_eq!(apply_damage(5, 10), 0);
        assert_eq!(apply_damage(20, 10), 10);
    }

    #[test]
    fn world_query_checks_component_composition() {
        let mut world = World::new();
        world.spawn((Enemy, Health(30)));
        world.spawn(Health(100));

        let mut query = world.query_filtered::<&Health, With<Enemy>>();
        let values = query
            .iter(&world)
            .map(|health| health.0)
            .collect::<Vec<_>>();
        assert_eq!(values, [30]);
    }

    #[test]
    fn headless_app_runs_real_systems_and_resources() {
        let mut app = test_app();
        app.world_mut().spawn((Enemy, Health(5)));
        app.world_mut().spawn((Enemy, Health(25)));

        app.update();

        assert_eq!(app.world().resource::<Defeated>().0, 1);
        let world = app.world_mut();
        let health = world
            .query_filtered::<&Health, With<Enemy>>()
            .iter(world)
            .map(|health| health.0)
            .collect::<Vec<_>>();
        assert_eq!(health, [0, 15]);
    }
}
