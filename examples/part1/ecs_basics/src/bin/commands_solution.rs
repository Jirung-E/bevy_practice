use bevy::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Loot;

#[derive(Component)]
struct TrainingTarget;

#[derive(Component)]
struct Health(u32);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
struct Position(i32);

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
        (remove_training_marker, defeat_one, drop_loot_and_despawn).chain(),
    );
    app
}

fn setup(mut commands: Commands) {
    commands.spawn((Enemy, Health(10), Position(1)));
    commands.spawn((Enemy, Health(10), Position(2)));
    commands.spawn((Enemy, Health(10), Position(3)));
    commands.spawn((Enemy, TrainingTarget, Health(99), Position(99)));
}

fn remove_training_marker(
    mut commands: Commands,
    targets: Query<Entity, (With<Enemy>, With<TrainingTarget>)>,
) {
    for entity in &targets {
        commands.entity(entity).remove::<Enemy>();
    }
}

fn defeat_one(mut enemies: Query<&mut Health, (With<Enemy>, Without<TrainingTarget>)>) {
    if let Some(mut health) = enemies.iter_mut().next() {
        health.0 = 0;
    }
}

fn drop_loot_and_despawn(
    mut commands: Commands,
    enemies: Query<(Entity, &Health, &Position), With<Enemy>>,
) {
    for (entity, health, position) in &enemies {
        if health.0 == 0 {
            commands.spawn((Loot, *position));
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_updates_replace_enemies_with_loot_and_marker_removal_keeps_entity() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();

        let world = app.world_mut();
        let enemy_count = world
            .query_filtered::<(), With<Enemy>>()
            .iter(world)
            .count();
        let mut loot_positions = world
            .query_filtered::<&Position, With<Loot>>()
            .iter(world)
            .map(|position| position.0)
            .collect::<Vec<_>>();
        loot_positions.sort_unstable();
        let target_position = world
            .query_filtered::<&Position, (With<TrainingTarget>, Without<Enemy>)>()
            .single(world)
            .expect("marker만 제거된 훈련 대상이 남아야 한다");

        assert_eq!(enemy_count, 0);
        assert_eq!(loot_positions, vec![1, 2, 3]);
        assert_eq!(*target_position, Position(99));
    }
}
