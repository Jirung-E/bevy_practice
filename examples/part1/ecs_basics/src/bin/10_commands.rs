use bevy::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Defeated;

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_enemies)
        .add_systems(Update, (mark_one_enemy, remove_defeated).chain());

    app.update();
    app.update();

    let world = app.world_mut();
    let mut enemies = world.query_filtered::<(), With<Enemy>>();
    println!("남은 Enemy 수: {}", enemies.iter(world).count());
}

fn spawn_enemies(mut commands: Commands) {
    commands.spawn(Enemy);
    commands.spawn(Enemy);
    commands.spawn(Enemy);
}

fn mark_one_enemy(
    mut commands: Commands,
    enemies: Query<Entity, (With<Enemy>, Without<Defeated>)>,
) {
    if let Some(entity) = enemies.iter().next() {
        commands.entity(entity).insert(Defeated);
        println!("{entity:?}에 Defeated 추가");
    }
}

fn remove_defeated(mut commands: Commands, defeated: Query<Entity, With<Defeated>>) {
    for entity in &defeated {
        commands.entity(entity).despawn();
        println!("{entity:?} 제거 예약");
    }
}
