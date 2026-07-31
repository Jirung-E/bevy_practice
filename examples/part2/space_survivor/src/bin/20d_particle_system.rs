use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 640.0;
const PLAYER_SPEED: f32 = 360.0;
const PARTICLE_BASE_SIZE: f32 = 10.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Particle {
    velocity: Vec2,
    acceleration: Vec2,
    age: f32,
    lifetime: f32,
    start_color: Vec4,
    end_color: Vec4,
    start_size: f32,
    end_size: f32,
}

#[derive(Component)]
struct ParticleCount;

#[derive(Resource)]
struct ThrusterTimer(Timer);

impl Default for ThrusterTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.04, TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
struct BurstSequence(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Particle System - Bevy Practice".into(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.015, 0.025, 0.06)))
        .init_resource::<ThrusterTimer>()
        .init_resource::<BurstSequence>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                emit_particles,
                update_particles,
                update_particle_count,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.15, 0.8, 1.0), Vec2::new(42.0, 48.0)),
        Transform::from_xyz(0.0, -110.0, 1.0),
    ));

    commands.spawn((
        Text::new("2D PARTICLE SYSTEM\nWASD / ARROWS: MOVE    SPACE: BURST"),
        TextFont {
            font_size: FontSize::Px(25.0),
            ..default()
        },
        TextColor(Color::srgb(0.45, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            top: px(18),
            ..default()
        },
    ));

    commands.spawn((
        ParticleCount,
        Text::new("PARTICLES: 0"),
        TextFont {
            font_size: FontSize::Px(21.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.84, 0.25)),
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

    let movement = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation.x =
        (player.translation.x + movement.x).clamp(-WINDOW_WIDTH * 0.45, WINDOW_WIDTH * 0.45);
    player.translation.y =
        (player.translation.y + movement.y).clamp(-WINDOW_HEIGHT * 0.38, WINDOW_HEIGHT * 0.38);
}

fn emit_particles(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    mut thruster_timer: ResMut<ThrusterTimer>,
    mut sequence: ResMut<BurstSequence>,
) {
    let origin = player.translation.truncate();
    let moving = keyboard.any_pressed([
        KeyCode::KeyA,
        KeyCode::KeyD,
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ]);

    if moving && thruster_timer.0.tick(time.delta()).just_finished() {
        for offset in [-0.24_f32, 0.0, 0.24] {
            let angle = -FRAC_PI_2 + offset;
            spawn_particle(
                &mut commands,
                origin + Vec2::new(0.0, -30.0),
                Particle {
                    velocity: Vec2::from_angle(angle) * 150.0,
                    acceleration: Vec2::new(0.0, -40.0),
                    age: 0.0,
                    lifetime: 0.48,
                    start_color: Vec4::new(0.3, 0.9, 1.0, 1.0),
                    end_color: Vec4::new(0.05, 0.2, 1.0, 0.0),
                    start_size: 8.0,
                    end_size: 2.0,
                },
            );
        }
    }

    if keyboard.just_pressed(KeyCode::Space) {
        const PARTICLE_COUNT: usize = 36;
        let phase = sequence.0 as f32 * 0.19;
        sequence.0 = sequence.0.wrapping_add(1);

        for index in 0..PARTICLE_COUNT {
            let angle = TAU * index as f32 / PARTICLE_COUNT as f32 + phase;
            let speed = 150.0 + (index % 6) as f32 * 22.0;
            spawn_particle(
                &mut commands,
                origin + Vec2::new(0.0, 30.0),
                Particle {
                    velocity: Vec2::from_angle(angle) * speed,
                    acceleration: Vec2::new(0.0, -90.0),
                    age: 0.0,
                    lifetime: 0.7 + (index % 4) as f32 * 0.08,
                    start_color: Vec4::new(1.0, 0.95, 0.45, 1.0),
                    end_color: Vec4::new(1.0, 0.12, 0.03, 0.0),
                    start_size: 13.0,
                    end_size: 1.0,
                },
            );
        }
    }
}

fn spawn_particle(commands: &mut Commands, position: Vec2, particle: Particle) {
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::splat(PARTICLE_BASE_SIZE)),
        Transform::from_xyz(position.x, position.y, 0.0),
        particle,
    ));
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    let delta = time.delta_secs();

    for (entity, mut particle, mut transform, mut sprite) in &mut particles {
        particle.age += delta;
        if particle.age >= particle.lifetime {
            commands.entity(entity).despawn();
            continue;
        }

        let acceleration = particle.acceleration;
        particle.velocity += acceleration * delta;
        transform.translation += (particle.velocity * delta).extend(0.0);

        let (color, size) = particle_visual(&particle);
        sprite.color = Color::srgba(color.x, color.y, color.z, color.w);
        transform.scale = Vec3::splat(size / PARTICLE_BASE_SIZE);
    }
}

fn particle_visual(particle: &Particle) -> (Vec4, f32) {
    let t = (particle.age / particle.lifetime).clamp(0.0, 1.0);
    (
        particle.start_color.lerp(particle.end_color, t),
        particle.start_size.lerp(particle.end_size, t),
    )
}

fn update_particle_count(
    particles: Query<(), With<Particle>>,
    mut text: Single<&mut Text, With<ParticleCount>>,
) {
    text.0 = format!("PARTICLES: {}", particles.iter().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_particle(age: f32) -> Particle {
        Particle {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            age,
            lifetime: 2.0,
            start_color: Vec4::new(1.0, 0.8, 0.2, 1.0),
            end_color: Vec4::new(1.0, 0.0, 0.0, 0.0),
            start_size: 12.0,
            end_size: 2.0,
        }
    }

    #[test]
    fn particle_visual_interpolates_color_alpha_and_size() {
        let (color, size) = particle_visual(&test_particle(1.0));

        assert_eq!(color, Vec4::new(1.0, 0.4, 0.1, 0.5));
        assert_eq!(size, 7.0);
    }

    #[test]
    fn particle_visual_clamps_normalized_age() {
        let (color, size) = particle_visual(&test_particle(3.0));

        assert_eq!(color, Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(size, 2.0);
    }
}
