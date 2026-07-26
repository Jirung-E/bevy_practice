use std::time::Duration;

use bevy::{
    asset::AssetPlugin,
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

const SHADER_PATH: &str = "shaders/13c_sprite_effect.wgsl";
const PLAYER_SPEED: f32 = 260.0;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SpriteEffectMaterial {
    // x: time, y: wobble strength, z: hit flash, w: unused
    #[uniform(0)]
    effect: Vec4,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
    // xy: atlas UV start, zw: atlas UV size
    #[uniform(3)]
    uv_rect: Vec4,
}

impl Material2d for SpriteEffectMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

struct SpriteEffectPlugin;

impl Plugin for SpriteEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<SpriteEffectMaterial>::default())
            .add_systems(Update, update_shader_effect);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct HitFlash(Timer);

#[derive(Resource)]
struct PlayerMaterial(Handle<SpriteEffectMaterial>);

#[derive(Component)]
struct StatusText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Material2d Effects - Bevy Practice".into(),
                        resolution: (960, 640).into(),
                        ..default()
                    }),
                    ..default()
                }),
            SpriteEffectPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.07)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, trigger_hit, update_status))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SpriteEffectMaterial>>,
) {
    commands.spawn(Camera2d);

    let material = materials.add(SpriteEffectMaterial {
        effect: Vec4::new(0.0, 7.0, 0.0, 0.0),
        color_texture: asset_server.load("textures/robot_sheet.png"),
        // 스프라이트 시트의 첫 프레임: 4열 × 2행 중 왼쪽 위
        uv_rect: Vec4::new(0.0, 0.0, 0.25, 0.5),
    });
    let mut hit_flash = Timer::new(Duration::from_secs_f32(1.28), TimerMode::Once);
    hit_flash.finish();

    commands.spawn((
        Player,
        HitFlash(hit_flash),
        Mesh2d(meshes.add(Rectangle::new(192.0, 192.0))),
        MeshMaterial2d(material.clone()),
    ));
    commands.insert_resource(PlayerMaterial(material));

    commands.spawn((
        Text::new("MATERIAL2D SPRITE EFFECT"),
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::srgb(0.45, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            top: px(16),
            ..default()
        },
    ));

    commands.spawn((
        StatusText,
        Text::new("WASD / ARROWS: MOVE  |  H: HIT FLASH"),
        TextFont {
            font_size: FontSize::Px(21.0),
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
    player.translation.x = player.translation.x.clamp(-390.0, 390.0);
    player.translation.y = player.translation.y.clamp(-240.0, 240.0);
}

fn trigger_hit(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hit_flash: Single<&mut HitFlash, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        hit_flash.0.reset();
    }
}

fn update_shader_effect(
    time: Res<Time>,
    player_material: Res<PlayerMaterial>,
    mut hit_flash: Single<&mut HitFlash, With<Player>>,
    mut materials: ResMut<Assets<SpriteEffectMaterial>>,
) {
    hit_flash.0.tick(time.delta());
    let flash = if hit_flash.0.is_finished() {
        0.0
    } else {
        1.0 - hit_flash.0.fraction()
    };

    if let Some(mut material) = materials.get_mut(&player_material.0) {
        material.effect.x = time.elapsed_secs();
        material.effect.z = flash;
    }
}

fn update_status(
    hit_flash: Single<&HitFlash, With<Player>>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    status.0 = format!(
        "WASD / ARROWS: MOVE  |  H: HIT FLASH  |  FLASH: {}",
        if hit_flash.0.is_finished() {
            "OFF"
        } else {
            "ON"
        }
    );
}
