use std::{collections::HashSet, time::Duration};

use bevy::{
    asset::AssetPlugin,
    math::ops,
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

const STARFIELD_SHADER: &str = "shaders/20b_starfield.wgsl";
const DISSOLVE_SHADER: &str = "shaders/20c_dissolve.wgsl";
const SHIELD_SHADER: &str = "shaders/20c_shield.wgsl";
const PLAYER_SPEED: f32 = 420.0;
const BULLET_SPEED: f32 = 620.0;
const ENEMY_SPEED: f32 = 115.0;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StarfieldMaterial {
    #[uniform(0)]
    options: Vec4,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        STARFIELD_SHADER.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DissolveMaterial {
    // x: time, y: dissolve progress, z: glowing edge width
    #[uniform(0)]
    effect: Vec4,
}

impl Material2d for DissolveMaterial {
    fn fragment_shader() -> ShaderRef {
        DISSOLVE_SHADER.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ShieldMaterial {
    // x: time, y: impact strength
    #[uniform(0)]
    effect: Vec4,
}

impl Material2d for ShieldMaterial {
    fn vertex_shader() -> ShaderRef {
        SHIELD_SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHIELD_SHADER.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct ShieldVisual;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Dissolving(Timer);

#[derive(Component)]
struct StatusText;

#[derive(Resource)]
struct StarfieldHandle(Handle<StarfieldMaterial>);

#[derive(Resource)]
struct ShieldHandle(Handle<ShieldMaterial>);

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource, Default)]
struct SpawnSequence(u32);

#[derive(Resource)]
struct ShieldPulse(Timer);

#[derive(Resource, Default)]
struct DissolveCount(u32);

fn main() {
    let mut shield_pulse = Timer::from_seconds(0.8, TimerMode::Once);
    shield_pulse.finish();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Survivor Shader Edition - Bevy Practice".into(),
                        resolution: (960, 640).into(),
                        ..default()
                    }),
                    ..default()
                }),
            Material2dPlugin::<StarfieldMaterial>::default(),
            Material2dPlugin::<DissolveMaterial>::default(),
            Material2dPlugin::<ShieldMaterial>::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.85,
            TimerMode::Repeating,
        )))
        .insert_resource(ShieldPulse(shield_pulse))
        .init_resource::<SpawnSequence>()
        .init_resource::<DissolveCount>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                follow_player_with_shield,
                shoot,
                spawn_enemies,
                move_entities,
                detect_hits,
                trigger_shield_manually,
                update_dissolves,
                update_shader_uniforms,
                update_status,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut starfields: ResMut<Assets<StarfieldMaterial>>,
    mut shields: ResMut<Assets<ShieldMaterial>>,
) {
    commands.spawn(Camera2d);

    let starfield = starfields.add(StarfieldMaterial {
        options: Vec4::new(0.0, 0.18, 0.1, 0.42),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(960.0, 640.0))),
        MeshMaterial2d(starfield.clone()),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    commands.insert_resource(StarfieldHandle(starfield));

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.12, 0.72, 1.0), Vec2::new(44.0, 36.0)),
        Transform::from_xyz(0.0, -250.0, 1.0),
    ));

    let shield = shields.add(ShieldMaterial { effect: Vec4::ZERO });
    commands.spawn((
        ShieldVisual,
        Mesh2d(meshes.add(Circle::new(42.0))),
        MeshMaterial2d(shield.clone()),
        Transform::from_xyz(0.0, -250.0, 2.0),
    ));
    commands.insert_resource(ShieldHandle(shield));

    commands.spawn((
        Text::new("SPACE SURVIVOR · SHADER EDITION"),
        TextFont {
            font_size: FontSize::Px(27.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.92, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            top: px(16),
            ..default()
        },
    ));

    commands.spawn((
        StatusText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            bottom: px(18),
            ..default()
        },
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    player.translation +=
        (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
    player.translation.x = player.translation.x.clamp(-445.0, 445.0);
    player.translation.y = player.translation.y.clamp(-280.0, 280.0);
}

fn follow_player_with_shield(
    player: Single<&Transform, (With<Player>, Without<ShieldVisual>)>,
    mut shield: Single<&mut Transform, (With<ShieldVisual>, Without<Player>)>,
) {
    shield.translation.x = player.translation.x;
    shield.translation.y = player.translation.y;
}

fn shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    commands.spawn((
        Bullet,
        Velocity(Vec2::Y * BULLET_SPEED),
        Sprite::from_color(Color::srgb(1.0, 0.88, 0.22), Vec2::new(8.0, 22.0)),
        Transform::from_translation(player.translation + Vec3::Y * 32.0),
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut sequence: ResMut<SpawnSequence>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DissolveMaterial>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    sequence.0 += 1;
    let x = ops::sin(sequence.0 as f32 * 2.17) * 405.0;
    let material = materials.add(DissolveMaterial {
        effect: Vec4::new(time.elapsed_secs(), 0.0, 0.075, 0.0),
    });
    commands.spawn((
        Enemy,
        Velocity(Vec2::NEG_Y * ENEMY_SPEED),
        Mesh2d(meshes.add(Rectangle::new(42.0, 42.0))),
        MeshMaterial2d(material),
        Transform::from_xyz(x, 345.0, 1.0),
    ));
}

fn move_entities(
    mut commands: Commands,
    time: Res<Time>,
    mut moving: Query<(Entity, &Velocity, &mut Transform)>,
) {
    for (entity, velocity, mut transform) in &mut moving {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
        if transform.translation.y > 370.0 || transform.translation.y < -370.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[expect(
    clippy::type_complexity,
    reason = "the query type documents the active-enemy filter used by collision handling"
)]
fn detect_hits(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<
        (Entity, &Transform, &MeshMaterial2d<DissolveMaterial>),
        (With<Enemy>, Without<Dissolving>),
    >,
    player: Single<&Transform, With<Player>>,
    mut shield_pulse: ResMut<ShieldPulse>,
) {
    let mut used_bullets = HashSet::new();

    for (bullet_entity, bullet_transform) in &bullets {
        for (enemy_entity, enemy_transform, _) in &enemies {
            if used_bullets.contains(&bullet_entity) {
                break;
            }
            if overlaps(
                bullet_transform.translation.truncate(),
                Vec2::new(8.0, 22.0),
                enemy_transform.translation.truncate(),
                Vec2::splat(42.0),
            ) {
                used_bullets.insert(bullet_entity);
                commands.entity(bullet_entity).despawn();
                commands
                    .entity(enemy_entity)
                    .remove::<(Enemy, Velocity)>()
                    .insert(Dissolving(Timer::new(
                        Duration::from_secs_f32(0.9),
                        TimerMode::Once,
                    )));
            }
        }
    }

    for (enemy_entity, enemy_transform, _) in &enemies {
        if overlaps(
            player.translation.truncate(),
            Vec2::new(52.0, 44.0),
            enemy_transform.translation.truncate(),
            Vec2::splat(42.0),
        ) {
            commands.entity(enemy_entity).despawn();
            shield_pulse.0.reset();
        }
    }
}

fn trigger_shield_manually(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut shield_pulse: ResMut<ShieldPulse>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        shield_pulse.0.reset();
    }
}

fn update_dissolves(
    mut commands: Commands,
    time: Res<Time>,
    mut dissolving: Query<(Entity, &mut Dissolving, &MeshMaterial2d<DissolveMaterial>)>,
    mut materials: ResMut<Assets<DissolveMaterial>>,
    mut count: ResMut<DissolveCount>,
) {
    for (entity, mut dissolve, material_handle) in &mut dissolving {
        dissolve.0.tick(time.delta());
        let progress = dissolve.0.fraction();

        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.effect.x = time.elapsed_secs();
            material.effect.y = progress;
        }

        if dissolve.0.just_finished() {
            count.0 += 1;
            commands.entity(entity).despawn();
        }
    }
}

fn update_shader_uniforms(
    time: Res<Time>,
    mut shield_pulse: ResMut<ShieldPulse>,
    starfield: Res<StarfieldHandle>,
    shield: Res<ShieldHandle>,
    mut starfields: ResMut<Assets<StarfieldMaterial>>,
    mut shields: ResMut<Assets<ShieldMaterial>>,
) {
    shield_pulse.0.tick(time.delta());
    let impact = if shield_pulse.0.is_finished() {
        0.0
    } else {
        1.0 - shield_pulse.0.fraction()
    };

    if let Some(mut material) = starfields.get_mut(&starfield.0) {
        material.options.x = time.elapsed_secs();
    }
    if let Some(mut material) = shields.get_mut(&shield.0) {
        material.effect = Vec4::new(time.elapsed_secs(), impact, 0.0, 0.0);
    }
}

fn update_status(
    shield_pulse: Res<ShieldPulse>,
    count: Res<DissolveCount>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    status.0 = format!(
        "WASD: MOVE  |  SPACE: FIRE  |  H: SHIELD IMPACT  |  DISSOLVED: {}  |  SHIELD: {}",
        count.0,
        if shield_pulse.0.is_finished() {
            "IDLE"
        } else {
            "IMPACT"
        }
    );
}

fn overlaps(a_position: Vec2, a_size: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let distance = (a_position - b_position).abs();
    distance.x < (a_size.x + b_size.x) * 0.5 && distance.y < (a_size.y + b_size.y) * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separated_shader_entities_do_not_collide() {
        assert!(!overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(20.0, 0.0),
            Vec2::splat(10.0),
        ));
    }

    #[test]
    fn overlapping_shader_entities_collide() {
        assert!(overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(4.0, 3.0),
            Vec2::splat(10.0),
        ));
    }
}
