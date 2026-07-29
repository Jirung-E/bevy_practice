use std::{f32::consts::PI, time::Duration};

use bevy::{
    asset::AssetPlugin, prelude::*, window::WindowResolution,
    world_serialization::WorldInstanceReady,
};

const FOX_PATH: &str = "models/fox/Fox.glb";
const MOVE_SPEED: f32 = 4.0;
const RUN_MULTIPLIER: f32 = 1.8;

#[derive(Resource)]
struct FoxAsset(Handle<Gltf>);

#[derive(Resource)]
struct FoxAnimations {
    graph: Handle<AnimationGraph>,
    survey: AnimationNodeIndex,
    walk: AnimationNodeIndex,
    run: AnimationNodeIndex,
}

#[derive(Resource)]
struct CharacterRoot(Entity);

#[derive(Component)]
struct Character;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Motion {
    Survey,
    Walk,
    Run,
}

#[derive(Component)]
struct StatusText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "glTF Character Animation - Bevy Practice".into(),
                        resolution: WindowResolution::new(1100, 720),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.032)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_character_when_loaded.run_if(not(resource_exists::<FoxAnimations>)),
                move_character.run_if(resource_exists::<CharacterRoot>),
                change_animation.run_if(resource_exists::<FoxAnimations>),
                update_status,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(FoxAsset(asset_server.load(FOX_PATH)));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        AmbientLight {
            color: Color::srgb(0.2, 0.28, 0.42),
            brightness: 230.0,
            ..default()
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 14_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.55, 0.0)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 18.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.09, 0.18, 0.12),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    commands.spawn((
        Text::new("glTF CHARACTER\nWASD: WALK   SHIFT: RUN"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.88, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(18),
            ..default()
        },
    ));
    commands.spawn((
        StatusText,
        Text::new("LOADING Fox.glb..."),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.82, 0.24)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(16),
            left: px(18),
            ..default()
        },
    ));
}

fn spawn_character_when_loaded(
    mut commands: Commands,
    fox_asset: Res<FoxAsset>,
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if !asset_server.is_loaded_with_dependencies(&fox_asset.0) {
        return;
    }

    let fox = gltfs
        .get(&fox_asset.0)
        .expect("로드 완료된 glTF Handle은 Assets<Gltf>에 존재해야 합니다");
    let (graph, nodes) = AnimationGraph::from_clips([
        fox.named_animations["Survey"].clone(),
        fox.named_animations["Walk"].clone(),
        fox.named_animations["Run"].clone(),
    ]);
    let animations = FoxAnimations {
        graph: graphs.add(graph),
        survey: nodes[0],
        walk: nodes[1],
        run: nodes[2],
    };
    commands.insert_resource(animations);

    let root = commands
        .spawn((
            Character,
            WorldAssetRoot(
                fox.default_scene
                    .clone()
                    .expect("Fox.glb에는 기본 Scene이 있어야 합니다"),
            ),
            Transform::from_rotation(Quat::from_rotation_y(PI)).with_scale(Vec3::splat(0.025)),
        ))
        .observe(prepare_animation_player)
        .id();
    commands.insert_resource(CharacterRoot(root));
}

fn prepare_animation_player(
    _ready: On<WorldInstanceReady>,
    mut commands: Commands,
    animations: Res<FoxAnimations>,
    player: Single<(Entity, &mut AnimationPlayer)>,
) {
    let (entity, mut player) = player.into_inner();
    let mut transitions = AnimationTransitions::new();
    transitions
        .play(&mut player, animations.survey, Duration::ZERO)
        .repeat();
    commands.entity(entity).insert((
        AnimationGraphHandle(animations.graph.clone()),
        transitions,
        Motion::Survey,
    ));
}

fn input_direction(keys: &ButtonInput<KeyCode>) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y += 1.0;
    }
    direction.normalize_or_zero()
}

fn desired_motion(keys: &ButtonInput<KeyCode>) -> Motion {
    if input_direction(keys) == Vec2::ZERO {
        Motion::Survey
    } else if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        Motion::Run
    } else {
        Motion::Walk
    }
}

fn move_character(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    root: Res<CharacterRoot>,
    mut transforms: Query<&mut Transform, With<Character>>,
) {
    let direction = input_direction(&keys);
    if direction == Vec2::ZERO {
        return;
    }
    let running = desired_motion(&keys) == Motion::Run;
    let velocity = Vec3::new(direction.x, 0.0, direction.y)
        * MOVE_SPEED
        * if running { RUN_MULTIPLIER } else { 1.0 };
    let Ok(mut transform) = transforms.get_mut(root.0) else {
        return;
    };
    transform.translation += velocity * time.delta_secs();
    transform.translation.x = transform.translation.x.clamp(-8.0, 8.0);
    transform.translation.z = transform.translation.z.clamp(-5.0, 5.0);
    transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.y));
}

fn change_animation(
    keys: Res<ButtonInput<KeyCode>>,
    animations: Res<FoxAnimations>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions, &mut Motion)>,
) {
    let desired = desired_motion(&keys);
    for (mut player, mut transitions, mut current) in &mut players {
        if *current == desired {
            continue;
        }
        let clip = match desired {
            Motion::Survey => animations.survey,
            Motion::Walk => animations.walk,
            Motion::Run => animations.run,
        };
        transitions
            .play(&mut player, clip, Duration::from_millis(220))
            .repeat();
        *current = desired;
    }
}

fn update_status(
    asset: Res<FoxAsset>,
    asset_server: Res<AssetServer>,
    motions: Query<&Motion>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    if let Ok(motion) = motions.single() {
        status.0 = format!(
            "READY   |   ANIMATION: {}   |   BLEND: 0.22s",
            match motion {
                Motion::Survey => "SURVEY (IDLE)",
                Motion::Walk => "WALK",
                Motion::Run => "RUN",
            }
        );
    } else if asset_server.is_loaded_with_dependencies(&asset.0) {
        status.0 = "SCENE LOADED   |   PREPARING ANIMATION PLAYER...".into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_selects_survey_and_shift_selects_run() {
        let mut keys = ButtonInput::default();
        assert_eq!(desired_motion(&keys), Motion::Survey);

        keys.press(KeyCode::KeyW);
        assert_eq!(desired_motion(&keys), Motion::Walk);

        keys.press(KeyCode::ShiftLeft);
        assert_eq!(desired_motion(&keys), Motion::Run);
    }

    #[test]
    fn diagonal_input_is_normalized() {
        let mut keys = ButtonInput::default();
        keys.press(KeyCode::KeyW);
        keys.press(KeyCode::KeyD);
        assert!((input_direction(&keys).length() - 1.0).abs() < 0.0001);
    }
}
