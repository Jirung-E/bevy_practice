use avian3d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    input::mouse::{MouseMotion, MouseWheel},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
    window::WindowResolution,
};
use bevy_landmass::{
    FromAgentRadius, NavMeshHandle, nav_mesh::bevy_mesh_to_landmass_nav_mesh, prelude::*,
};
use std::sync::Arc;

const PLAYER_SPEED: f32 = 5.5;
const OBSTACLE_POSITIONS: [Vec3; 3] = [
    Vec3::new(-4.0, 1.0, -2.0),
    Vec3::new(4.0, 1.0, 0.5),
    Vec3::new(0.0, 1.0, -5.0),
];
const NAV_OBSTACLE_CLEARANCE: f32 = 1.55;

#[derive(Resource, Clone, Copy)]
pub struct LessonConfig {
    pub camera: bool,
    pub animation: bool,
    pub physics: bool,
    pub navigation: bool,
}

impl LessonConfig {
    pub const CORE: Self = Self {
        camera: false,
        animation: false,
        physics: false,
        navigation: false,
    };
    pub const CAMERA: Self = Self {
        camera: true,
        ..Self::CORE
    };
    pub const ANIMATION: Self = Self {
        animation: true,
        ..Self::CAMERA
    };
    pub const PHYSICS: Self = Self {
        physics: true,
        ..Self::ANIMATION
    };
    pub const COMPLETE: Self = Self {
        navigation: true,
        ..Self::PHYSICS
    };
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerVisual;

#[derive(Component)]
struct Enemy;

#[derive(Component, Default)]
struct MotionAmount(f32);

#[derive(Component)]
struct FollowCamera;

#[derive(Resource)]
struct CameraRig {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.28,
            distance: 7.5,
        }
    }
}

pub fn run(config: LessonConfig) {
    App::new()
        .insert_resource(config)
        .init_resource::<CameraRig>()
        .insert_resource(ClearColor(Color::srgb(0.015, 0.022, 0.035)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TPS Training Ground - Bevy Practice".into(),
                    resolution: WindowResolution::new(1150, 720),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            Landmass3dPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                read_camera_input,
                move_player,
                animate_player,
                drive_nav_agents,
            )
                .chain(),
        )
        .add_systems(PostUpdate, follow_player)
        .run();
}

fn setup(
    mut commands: Commands,
    config: Res<LessonConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nav_meshes: ResMut<Assets<NavMesh3d>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 13_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.5, 0.0)),
    ));

    let ground_mesh = meshes.add(Plane3d::default().mesh().size(24.0, 24.0));
    let ground = commands
        .spawn((
            Mesh3d(ground_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.16, 0.22, 0.17),
                perceptual_roughness: 0.9,
                ..default()
            })),
        ))
        .id();
    if config.physics {
        commands
            .entity(ground)
            .insert((RigidBody::Static, Collider::cuboid(24.0, 0.2, 24.0)));
    }

    let player_visual = meshes.add(Capsule3d::new(0.45, 1.0));
    let player_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.55, 0.95),
        metallic: 0.15,
        perceptual_roughness: 0.35,
        ..default()
    });
    let player = commands
        .spawn((
            Player,
            MotionAmount::default(),
            Transform::from_xyz(0.0, 1.0, 4.0),
            GlobalTransform::default(),
            Visibility::default(),
            children![(
                PlayerVisual,
                Mesh3d(player_visual),
                MeshMaterial3d(player_material),
                Transform::default(),
            )],
        ))
        .id();
    if config.physics {
        commands.entity(player).insert((
            RigidBody::Dynamic,
            Collider::capsule(0.45, 1.0),
            LinearVelocity::ZERO,
            LockedAxes::ROTATION_LOCKED,
            Friction::new(0.0),
        ));
    }

    commands.spawn((
        FollowCamera,
        Camera3d::default(),
        AmbientLight {
            color: Color::srgb(0.35, 0.42, 0.58),
            brightness: 160.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.5, 11.0).looking_at(Vec3::Y, Vec3::Y),
    ));
    commands.spawn((
        Text::new("TPS TRAINING\nWASD: MOVE   RMB: CAMERA   WHEEL: ZOOM   SPACE: JUMP"),
        TextFont {
            font_size: FontSize::Px(19.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(14),
            left: px(16),
            ..default()
        },
    ));

    spawn_obstacles(&mut commands, &mut meshes, &mut materials, config.physics);

    if config.navigation {
        setup_navigation(
            &mut commands,
            &mut nav_meshes,
            &mut materials,
            &mut meshes,
            player,
        );
    }
}

fn spawn_obstacles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    physics: bool,
) {
    let mesh = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let material = materials.add(Color::srgb(0.35, 0.28, 0.22));
    for position in OBSTACLE_POSITIONS {
        let obstacle = commands
            .spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position),
            ))
            .id();
        if physics {
            commands
                .entity(obstacle)
                .insert((RigidBody::Static, Collider::cuboid(2.0, 2.0, 2.0)));
        }
    }
}

fn setup_navigation(
    commands: &mut Commands,
    nav_meshes: &mut Assets<NavMesh3d>,
    materials: &mut Assets<StandardMaterial>,
    render_meshes: &mut Assets<Mesh>,
    player: Entity,
) {
    let navigation_mesh = {
        let source_mesh = build_training_nav_mesh();
        bevy_mesh_to_landmass_nav_mesh(&source_mesh)
            .expect("the ground mesh can be converted")
            .validate()
            .expect("the ground navigation mesh is valid")
    };
    let nav_handle = nav_meshes.add(NavMesh3d {
        nav_mesh: Arc::new(navigation_mesh),
    });

    let archipelago = commands
        .spawn(Archipelago3d::new(ArchipelagoOptions::from_agent_radius(
            0.45,
        )))
        .id();
    commands.spawn((
        Island3dBundle {
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
            island: Island,
            nav_mesh: NavMeshHandle(nav_handle),
        },
        // Agent와 Player의 Transform은 캡슐 중심(y = 1)을 나타냅니다.
        // Landmass가 같은 높이에서 시작점과 목표를 샘플링하도록 맞춥니다.
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    commands.spawn((
        Enemy,
        RigidBody::Kinematic,
        Collider::capsule(0.45, 1.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Agent3dBundle {
            agent: default(),
            settings: AgentSettings {
                radius: 0.45,
                desired_speed: 2.2,
                max_speed: 3.2,
            },
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
        },
        AgentTarget3d::Entity(player),
        Mesh3d(render_meshes.add(Capsule3d::new(0.45, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.18, 0.12))),
        Transform::from_xyz(-7.0, 1.0, -7.0),
    ));
}

fn build_training_nav_mesh() -> Mesh {
    let x_coordinates = [-12.0, -5.55, -2.45, -1.55, 1.55, 2.45, 5.55, 12.0];
    let z_coordinates = [-12.0, -6.55, -3.55, -3.45, -1.05, -0.45, 2.05, 12.0];
    let mut positions = Vec::with_capacity(x_coordinates.len() * z_coordinates.len());
    for &z in &z_coordinates {
        for &x in &x_coordinates {
            positions.push([x, 0.0, z]);
        }
    }

    let row_width = x_coordinates.len() as u32;
    let mut indices = Vec::new();
    for z_index in 0..z_coordinates.len() - 1 {
        for x_index in 0..x_coordinates.len() - 1 {
            let center = Vec2::new(
                (x_coordinates[x_index] + x_coordinates[x_index + 1]) * 0.5,
                (z_coordinates[z_index] + z_coordinates[z_index + 1]) * 0.5,
            );
            let blocked = OBSTACLE_POSITIONS.iter().any(|obstacle| {
                (center.x - obstacle.x).abs() < NAV_OBSTACLE_CLEARANCE
                    && (center.y - obstacle.z).abs() < NAV_OBSTACLE_CLEARANCE
            });
            if blocked {
                continue;
            }

            let bottom_left = z_index as u32 * row_width + x_index as u32;
            let bottom_right = bottom_left + 1;
            let top_left = bottom_left + row_width;
            let top_right = top_left + 1;
            indices.extend_from_slice(&[
                bottom_left,
                top_left,
                top_right,
                bottom_left,
                top_right,
                bottom_right,
            ]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(Indices::U32(indices))
}

fn read_camera_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    config: Res<LessonConfig>,
    mut rig: ResMut<CameraRig>,
) {
    if !config.camera {
        motion.clear();
        wheel.clear();
        return;
    }
    let delta = motion.read().map(|event| event.delta).sum::<Vec2>();
    if buttons.pressed(MouseButton::Right) {
        rig.yaw -= delta.x * 0.005;
        rig.pitch = (rig.pitch - delta.y * 0.004).clamp(-0.8, 0.05);
    }
    for event in wheel.read() {
        rig.distance = (rig.distance - event.y * 0.5).clamp(3.5, 11.0);
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<LessonConfig>,
    rig: Res<CameraRig>,
    mut player: Single<
        (
            &mut Transform,
            &mut MotionAmount,
            Option<&mut LinearVelocity>,
        ),
        With<Player>,
    >,
) {
    let mut input = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    let camera_rotation = Quat::from_rotation_y(rig.yaw);
    let direction = camera_rotation * Vec3::new(input.x, 0.0, -input.y).normalize_or_zero();
    player.1.0 = direction.length();
    if direction.length_squared() > 0.0 {
        player.0.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
    }

    if config.physics {
        let is_near_ground = player.0.translation.y < 1.15;
        if let Some(velocity) = player.2.as_mut() {
            velocity.x = direction.x * PLAYER_SPEED;
            velocity.z = direction.z * PLAYER_SPEED;
            if keyboard.just_pressed(KeyCode::Space) && is_near_ground {
                velocity.y = 6.5;
            }
        }
    } else {
        player.0.translation += direction * PLAYER_SPEED * time.delta_secs();
        player.0.translation.x = player.0.translation.x.clamp(-11.0, 11.0);
        player.0.translation.z = player.0.translation.z.clamp(-11.0, 11.0);
    }
}

fn animate_player(
    time: Res<Time>,
    config: Res<LessonConfig>,
    player: Single<&MotionAmount, With<Player>>,
    mut visuals: Query<&mut Transform, With<PlayerVisual>>,
) {
    if !config.animation {
        return;
    }
    let phase = time.elapsed_secs() * 10.0;
    for mut transform in &mut visuals {
        transform.translation.y = phase.sin().abs() * 0.08 * player.0;
        transform.rotation = Quat::from_rotation_z(phase.sin() * 0.08 * player.0);
    }
}

fn follow_player(
    config: Res<LessonConfig>,
    rig: Res<CameraRig>,
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    let target = player.translation + Vec3::Y * 1.4;
    if config.camera {
        let rotation = Quat::from_euler(EulerRot::YXZ, rig.yaw, rig.pitch, 0.0);
        let position = target + rotation * Vec3::new(0.0, 0.0, rig.distance);
        **camera = Transform::from_translation(position).looking_at(target, Vec3::Y);
    } else {
        **camera = Transform::from_translation(target + Vec3::new(0.0, 5.0, 8.0))
            .looking_at(target, Vec3::Y);
    }
}

fn drive_nav_agents(
    config: Res<LessonConfig>,
    mut agents: Query<
        (
            &AgentDesiredVelocity3d,
            &mut Velocity3d,
            &mut LinearVelocity,
            &mut Transform,
        ),
        With<Enemy>,
    >,
) {
    if !config.navigation {
        return;
    }
    for (desired, mut navigation_velocity, mut physics_velocity, mut transform) in &mut agents {
        let movement = desired.velocity();
        navigation_velocity.velocity = movement;
        physics_velocity.x = movement.x;
        physics_velocity.z = movement.z;
        if movement.length_squared() > 0.01 {
            transform.rotation = Quat::from_rotation_y(movement.x.atan2(movement.z));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_distance_is_clamped_to_playable_range() {
        let mut rig = CameraRig::default();
        rig.distance = (rig.distance - 100.0).clamp(3.5, 11.0);
        assert_eq!(rig.distance, 3.5);
    }

    #[test]
    fn obstacle_aware_ground_produces_valid_nav_mesh() {
        let mesh = build_training_nav_mesh();
        let nav_mesh: NavigationMesh3d =
            bevy_mesh_to_landmass_nav_mesh(&mesh).expect("plane conversion succeeds");
        nav_mesh.validate().expect("plane nav mesh is valid");
    }

    #[test]
    fn centered_agent_finds_a_path_on_the_shifted_nav_mesh() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            TransformPlugin,
            Landmass3dPlugin::default(),
        ))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            Time::<Fixed>::default().timestep(),
        ));
        app.finish();
        app.update();

        let archipelago = app
            .world_mut()
            .spawn(Archipelago3d::new(ArchipelagoOptions::from_agent_radius(
                0.45,
            )))
            .id();
        let navigation_mesh = bevy_mesh_to_landmass_nav_mesh(&build_training_nav_mesh())
            .expect("mesh conversion succeeds")
            .validate()
            .expect("navigation mesh is valid");
        let nav_handle = app
            .world_mut()
            .resource_mut::<Assets<NavMesh3d>>()
            .add(NavMesh3d {
                nav_mesh: Arc::new(navigation_mesh),
            });

        app.world_mut().spawn((
            Island3dBundle {
                archipelago_ref: ArchipelagoRef3d::new(archipelago),
                island: Island,
                nav_mesh: NavMeshHandle(nav_handle),
            },
            Transform::from_xyz(0.0, 1.0, 0.0),
            GlobalTransform::default(),
        ));
        let target = app
            .world_mut()
            .spawn(Transform::from_xyz(0.0, 1.0, 4.0))
            .insert(GlobalTransform::default())
            .id();
        let agent = app
            .world_mut()
            .spawn((
                Transform::from_xyz(-7.0, 1.0, -7.0),
                GlobalTransform::default(),
                Agent3dBundle {
                    agent: default(),
                    settings: AgentSettings {
                        radius: 0.45,
                        desired_speed: 2.2,
                        max_speed: 3.2,
                    },
                    archipelago_ref: ArchipelagoRef3d::new(archipelago),
                },
                AgentTarget3d::Entity(target),
            ))
            .id();

        app.update();
        app.update();

        assert_eq!(
            *app.world()
                .get::<AgentState>(agent)
                .expect("agent state exists"),
            AgentState::Moving
        );
        let desired = app
            .world()
            .get::<AgentDesiredVelocity3d>(agent)
            .expect("desired velocity exists")
            .velocity();
        assert!(desired.length_squared() > 0.0);

        let direct = (Vec3::new(0.0, 1.0, 4.0) - Vec3::new(-7.0, 1.0, -7.0)).normalize();
        assert!(
            desired.normalize().dot(direct) < 0.99,
            "장애물이 직선 경로를 막으므로 첫 이동 방향은 우회해야 합니다."
        );
    }
}
