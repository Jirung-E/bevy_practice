use super::core::EnemyDefeated;
use crate::{
    components::{ArenaEntity, Enemy, Player, Velocity},
    resources::{ArenaAssets, Score},
    schedule::GameSet,
};
use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_arena)
            .add_systems(Update, read_player_input.in_set(GameSet::Input))
            .add_systems(
                Update,
                (move_actors, wrap_arena)
                    .chain()
                    .in_set(GameSet::Simulation),
            )
            .add_systems(
                Update,
                (defeat_nearest_enemy, apply_score)
                    .chain()
                    .in_set(GameSet::Feedback),
            );
    }
}

fn spawn_arena(mut commands: Commands, assets: Res<ArenaAssets>) {
    commands.spawn((
        ArenaEntity,
        Player,
        Velocity(Vec3::ZERO),
        Mesh3d(assets.player_mesh.clone()),
        MeshMaterial3d(assets.player_material.clone()),
        Transform::from_xyz(0.0, 0.9, 4.0),
    ));

    for (index, position) in [
        Vec3::new(-4.0, 0.55, -3.0),
        Vec3::new(0.0, 0.55, -5.0),
        Vec3::new(4.0, 0.55, -2.0),
    ]
    .into_iter()
    .enumerate()
    {
        commands.spawn((
            ArenaEntity,
            Enemy,
            Velocity(Vec3::new(if index % 2 == 0 { 1.2 } else { -1.2 }, 0.0, 0.0)),
            Mesh3d(assets.enemy_mesh.clone()),
            MeshMaterial3d(assets.enemy_material.clone()),
            Transform::from_translation(position),
        ));
    }
}

fn read_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    player.0 = direction.normalize_or_zero() * 5.0;
}

fn move_actors(time: Res<Time>, mut actors: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut actors {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

fn wrap_arena(mut actors: Query<&mut Transform, With<ArenaEntity>>) {
    for mut transform in &mut actors {
        transform.translation.x = wrap_axis(transform.translation.x, 8.0);
        transform.translation.z = wrap_axis(transform.translation.z, 8.0);
    }
}

pub(crate) fn wrap_axis(value: f32, limit: f32) -> f32 {
    if value > limit {
        -limit
    } else if value < -limit {
        limit
    } else {
        value
    }
}

fn defeat_nearest_enemy(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut defeated: MessageWriter<EnemyDefeated>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    let nearest = enemies.iter().min_by(|(_, left), (_, right)| {
        left.translation
            .distance_squared(player.translation)
            .total_cmp(&right.translation.distance_squared(player.translation))
    });
    if let Some((entity, transform)) = nearest
        && transform.translation.distance(player.translation) <= 4.0
    {
        commands.entity(entity).despawn();
        defeated.write(EnemyDefeated { points: 100 });
    }
}

fn apply_score(mut messages: MessageReader<EnemyDefeated>, mut score: ResMut<Score>) {
    score.0 += messages.read().map(|message| message.points).sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_crosses_to_opposite_edge() {
        assert_eq!(wrap_axis(8.1, 8.0), -8.0);
        assert_eq!(wrap_axis(-8.1, 8.0), 8.0);
        assert_eq!(wrap_axis(2.0, 8.0), 2.0);
    }
}
