use avian3d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

const CAMERA_RADIUS: f32 = 0.25;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FollowCamera {
    target: Entity,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

#[derive(Resource, Default)]
struct CollisionEnabled(bool);

#[derive(Component)]
struct StatusText;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TPS Camera Collision - Bevy Practice".into(),
                resolution: WindowResolution::new(1000, 700),
                ..default()
            }),
            ..default()
        }), PhysicsPlugins::default()))
        .insert_resource(CollisionEnabled(true))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, toggle_collision, follow_camera).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = commands
        .spawn((
            Player,
            RigidBody::Kinematic,
            Collider::capsule(0.5, 1.2),
            Mesh3d(meshes.add(Capsule3d::new(0.5, 1.2))),
            MeshMaterial3d(materials.add(Color::srgb(0.2, 0.62, 1.0))),
            Transform::from_xyz(0.0, 1.1, 0.0),
        ))
        .id();

    commands.spawn((
        FollowCamera {
            target: player,
            yaw: 0.62,
            pitch: 0.28,
            distance: 8.0,
        },
        Camera3d::default(),
        Transform::default(),
    ));

    spawn_box(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, -0.25, 0.0),
        Vec3::new(18.0, 0.5, 18.0),
        Color::srgb(0.18, 0.28, 0.2),
    );
    spawn_box(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(2.5, 2.0, 3.4),
        Vec3::new(4.0, 4.0, 0.7),
        Color::srgb(0.55, 0.38, 0.26),
    );
    spawn_box(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-3.0, 1.5, -1.5),
        Vec3::new(2.0, 3.0, 2.0),
        Color::srgb(0.45, 0.35, 0.28),
    );

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.5, 0.0)),
    ));
    commands.spawn((
        StatusText,
        Text::new("WASD: MOVE  |  C: CAMERA COLLISION ON"),
        TextFont {
            font_size: FontSize::Px(21.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            ..default()
        },
    ));
}

fn spawn_box(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
    size: Vec3,
    color: Color,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(size.x, size.y, size.z),
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(materials.add(color)),
        Transform::from_translation(position),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let x = keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) as i8
        - keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) as i8;
    let z = keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) as i8
        - keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) as i8;
    let direction = Vec3::new(x as f32, 0.0, z as f32).normalize_or_zero();
    player.translation += direction * 3.5 * time.delta_secs();
}

fn toggle_collision(
    keys: Res<ButtonInput<KeyCode>>,
    mut enabled: ResMut<CollisionEnabled>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        enabled.0 = !enabled.0;
        text.0 = format!(
            "WASD: MOVE  |  C: CAMERA COLLISION {}",
            if enabled.0 { "ON" } else { "OFF" }
        );
    }
}

fn follow_camera(
    spatial_query: SpatialQuery,
    enabled: Res<CollisionEnabled>,
    targets: Query<&GlobalTransform, With<Player>>,
    mut cameras: Query<(&FollowCamera, &mut Transform)>,
) {
    for (follow, mut camera) in &mut cameras {
        let Ok(target_transform) = targets.get(follow.target) else {
            continue;
        };
        let focus = target_transform.translation() + Vec3::Y * 1.1;
        let rotation = Quat::from_euler(EulerRot::YXZ, follow.yaw, -follow.pitch, 0.0);
        let backward = rotation * Vec3::Z;
        let mut distance = follow.distance;

        if enabled.0 {
            let filter = SpatialQueryFilter::from_excluded_entities([follow.target]);
            if let Some(hit) = spatial_query.cast_ray(
                focus,
                Dir3::new(backward).expect("camera direction must be non-zero"),
                follow.distance,
                true,
                &filter,
            ) {
                distance = (hit.distance - CAMERA_RADIUS).max(0.5);
            }
        }

        camera.translation = focus + backward * distance;
        camera.look_at(focus, Vec3::Y);
    }
}
