use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, Debug)]
struct Health(u32);

#[derive(Resource)]
struct Targets {
    player: Entity,
    first_enemy: Entity,
}

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, setup).add_systems(
        Update,
        (damage_enemies, restore_player, inspect_health).chain(),
    );
    app.update();
}

fn setup(mut commands: Commands) {
    let player = commands.spawn((Player, Health(95))).id();
    let first_enemy = commands.spawn((Enemy, Health(50))).id();
    commands.spawn((Enemy, Health(80)));
    commands.insert_resource(Targets {
        player,
        first_enemy,
    });
}

#[expect(
    clippy::type_complexity,
    reason = "학습 예제에서 Query의 데이터와 필터 조합을 한눈에 보여 준다"
)]
fn damage_enemies(mut enemies: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>) {
    for (entity, mut health) in enemies.iter_mut() {
        health.0 = health.0.saturating_sub(10);
        println!("{entity:?}의 남은 체력: {}", health.0);
    }
}

fn restore_player(targets: Res<Targets>, mut healths: Query<&mut Health>) {
    if let Ok(mut health) = healths.get_mut(targets.player) {
        health.0 = health.0.saturating_add(5);
    }
}

fn inspect_health(
    targets: Res<Targets>,
    player: Query<&Health, With<Player>>,
    all_health: Query<&Health>,
) {
    let Ok(player_health) = player.single() else {
        println!("Player는 정확히 한 명이어야 합니다.");
        return;
    };

    if let Ok(health) = all_health.get(targets.first_enemy) {
        println!("첫 번째 적 체력: {}", health.0);
    }

    let entity_count = all_health.iter().count();
    println!("Health를 가진 Entity: {entity_count}");
    println!("플레이어 체력: {}", player_health.0);
}
