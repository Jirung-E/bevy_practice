use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, Debug)]
struct Health(u32);

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, setup)
        .add_systems(Update, damage_enemies);
    app.update();
}

fn setup(mut commands: Commands) {
    commands.spawn((Player, Health(100)));
    commands.spawn((Enemy, Health(50)));
    commands.spawn((Enemy, Health(80)));
}

#[expect(
    clippy::type_complexity,
    reason = "학습 예제에서 Query의 데이터와 필터 조합을 한눈에 보여 준다"
)]
fn damage_enemies(mut enemies: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>) {
    for (entity, mut health) in &mut enemies {
        health.0 = health.0.saturating_sub(10);
        println!("{entity:?}의 남은 체력: {}", health.0);
    }
}
