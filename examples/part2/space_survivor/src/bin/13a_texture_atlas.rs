use std::time::Duration;

use bevy::{asset::AssetPlugin, prelude::*};

const FRAME_SIZE: UVec2 = UVec2::new(128, 128);
const PLAYER_SPEED: f32 = 260.0;
const WORLD_HALF_SIZE: Vec2 = Vec2::new(400.0, 250.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Animation {
    timer: Timer,
    moving: bool,
}

#[derive(Component)]
struct AnimationStatus;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Texture Atlas - Bevy Practice".into(),
                        resolution: (960, 640).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.07)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, animate_player).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let image = asset_server.load("textures/robot_sheet.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        FRAME_SIZE,
        4,
        2,
        None,
        None,
    ));

    commands.spawn((
        Player,
        Animation {
            timer: Timer::new(Duration::from_secs_f32(0.14), TimerMode::Repeating),
            moving: false,
        },
        Sprite {
            image,
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("TEXTURE ATLAS\nWASD / ARROWS: MOVE"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(0.45, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(18),
            top: px(14),
            ..default()
        },
    ));

    commands.spawn((
        AnimationStatus,
        Text::new("STATE: IDLE  |  FRAME: 0"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            left: px(18),
            bottom: px(14),
            ..default()
        },
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<(&mut Transform, &mut Sprite, &mut Animation), With<Player>>,
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

    let (transform, sprite, animation) = &mut *player;
    let velocity = direction.normalize_or_zero() * PLAYER_SPEED;
    transform.translation += (velocity * time.delta_secs()).extend(0.0);
    transform.translation.x = transform
        .translation
        .x
        .clamp(-WORLD_HALF_SIZE.x, WORLD_HALF_SIZE.x);
    transform.translation.y = transform
        .translation
        .y
        .clamp(-WORLD_HALF_SIZE.y, WORLD_HALF_SIZE.y);

    animation.moving = direction != Vec2::ZERO;
    if direction.x != 0.0 {
        sprite.flip_x = direction.x < 0.0;
    }
}

fn animate_player(
    time: Res<Time>,
    mut player: Single<(&mut Sprite, &mut Animation), With<Player>>,
    mut status: Single<&mut Text, With<AnimationStatus>>,
) {
    let (sprite, animation) = &mut *player;
    let Some(atlas) = &mut sprite.texture_atlas else {
        return;
    };

    if animation.moving {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            atlas.index = 4 + (atlas.index.saturating_sub(4) + 1) % 4;
        }
    } else {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            atlas.index = (atlas.index + 1) % 4;
        }
    }

    status.0 = format!(
        "STATE: {}  |  FRAME: {}",
        if animation.moving { "WALK" } else { "IDLE" },
        atlas.index
    );
}
