use bevy::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(u32);

#[derive(Resource)]
struct TrackedTarget(Option<Entity>);

#[derive(Resource, Default, Debug)]
struct LifecycleReport {
    removed_health: Vec<Entity>,
    stale_reference_cleared: bool,
}

#[derive(Resource, Default)]
struct LifecycleStep(u8);

fn main() {
    let mut app = build_app();
    app.update();
    app.update();

    let report = app.world().resource::<LifecycleReport>();
    println!("Health 제거 감지: {}건", report.removed_health.len());
    println!("무효 Entity 참조 정리: {}", report.stale_reference_cleared);
}

fn build_app() -> App {
    let mut app = App::new();
    app.init_resource::<LifecycleReport>()
        .init_resource::<LifecycleStep>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                remove_health,
                record_removed_health,
                despawn_without_health,
                clear_stale_reference,
                advance_step,
            )
                .chain(),
        );
    app
}

fn setup(mut commands: Commands) {
    let target = commands.spawn((Enemy, Health(30))).id();
    commands.insert_resource(TrackedTarget(Some(target)));
}

fn remove_health(
    step: Res<LifecycleStep>,
    tracked: Res<TrackedTarget>,
    healths: Query<&Health>,
    mut commands: Commands,
) {
    if step.0 == 0
        && let Some(entity) = tracked.0
    {
        if let Ok(health) = healths.get(entity) {
            println!("제거 전 Health: {}", health.0);
        }
        commands.entity(entity).remove::<Health>();
    }
}

fn record_removed_health(
    mut removed: RemovedComponents<Health>,
    mut report: ResMut<LifecycleReport>,
) {
    report.removed_health.extend(removed.read());
}

fn despawn_without_health(
    step: Res<LifecycleStep>,
    targets: Query<Entity, (With<Enemy>, Without<Health>)>,
    mut commands: Commands,
) {
    if step.0 == 1 {
        for entity in &targets {
            commands.entity(entity).despawn();
        }
    }
}

fn clear_stale_reference(
    mut tracked: ResMut<TrackedTarget>,
    enemies: Query<(), With<Enemy>>,
    mut report: ResMut<LifecycleReport>,
) {
    let Some(entity) = tracked.0 else {
        return;
    };
    if enemies.get(entity).is_err() {
        tracked.0 = None;
        report.stale_reference_cleared = true;
    }
}

fn advance_step(mut step: ResMut<LifecycleStep>) {
    step.0 += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removal_is_observed_before_despawn_and_stale_reference_is_cleared() {
        let mut app = build_app();
        app.update();

        let entity = app.world().resource::<TrackedTarget>().0.unwrap();
        assert!(app.world().get_entity(entity).is_ok());
        assert!(app.world().get::<Health>(entity).is_none());
        assert_eq!(
            app.world().resource::<LifecycleReport>().removed_health,
            [entity]
        );

        app.update();
        assert!(app.world().get_entity(entity).is_err());
        assert_eq!(app.world().resource::<TrackedTarget>().0, None);
        assert!(
            app.world()
                .resource::<LifecycleReport>()
                .stale_reference_cleared
        );
    }
}
