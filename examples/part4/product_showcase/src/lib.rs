use bevy::{
    asset::AssetPlugin,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::WindowResolution,
};

#[derive(Resource, Clone, Copy)]
pub struct LessonConfig {
    pub meshes: bool,
    pub materials: bool,
    pub lights: bool,
}

impl LessonConfig {
    pub const CAMERA: Self = Self {
        meshes: false,
        materials: false,
        lights: false,
    };
    pub const MESH: Self = Self {
        meshes: true,
        ..Self::CAMERA
    };
    pub const MATERIAL: Self = Self {
        materials: true,
        ..Self::MESH
    };
    pub const COMPLETE: Self = Self {
        lights: true,
        ..Self::MATERIAL
    };
}

#[derive(Component)]
struct OrbitCamera;

#[derive(Component)]
struct Product;

#[derive(Resource)]
struct Orbit {
    yaw: f32,
    pitch: f32,
    radius: f32,
    focus: Vec3,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            yaw: -0.45,
            pitch: -0.3,
            radius: 9.0,
            focus: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

pub fn run(config: LessonConfig) {
    App::new()
        .insert_resource(config)
        .init_resource::<Orbit>()
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.032)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Product Showcase - Bevy 3D Practice".into(),
                        resolution: WindowResolution::new(1100, 720),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_input, update_orbit_camera, rotate_product))
        .run();
}

fn setup(
    mut commands: Commands,
    config: Res<LessonConfig>,
    orbit: Res<Orbit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        OrbitCamera,
        Camera3d { ..default() },
        AmbientLight {
            color: Color::srgb(0.18, 0.22, 0.35),
            brightness: if config.lights { 90.0 } else { 650.0 },
            ..default()
        },
        orbit_transform(&orbit),
    ));

    commands.spawn((
        Text::new("PRODUCT SHOWCASE\nRMB DRAG: ORBIT   WHEEL: ZOOM"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(18),
            ..default()
        },
    ));

    if !config.meshes {
        return;
    }

    let unlit = !config.materials;
    let body_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.35, 0.82),
        metallic: if config.materials { 0.65 } else { 0.0 },
        perceptual_roughness: if config.materials { 0.22 } else { 1.0 },
        unlit,
        ..default()
    });
    let accent_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.32, 0.08),
        metallic: if config.materials { 0.25 } else { 0.0 },
        perceptual_roughness: 0.32,
        unlit,
        ..default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.055, 0.065, 0.09),
        metallic: 0.0,
        perceptual_roughness: 0.8,
        unlit,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 16.0))),
        MeshMaterial3d(floor_material),
    ));
    commands
        .spawn((
            Product,
            Transform::from_xyz(0.0, 1.35, 0.0),
            Visibility::default(),
        ))
        .with_children(|product| {
            product.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.6, 1.8, 1.8))),
                MeshMaterial3d(body_material.clone()),
            ));
            product.spawn((
                Mesh3d(meshes.add(Torus::new(0.54, 0.72))),
                MeshMaterial3d(accent_material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.95)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ));
            product.spawn((
                Mesh3d(meshes.add(Sphere::new(0.42))),
                MeshMaterial3d(body_material),
                Transform::from_xyz(0.0, 0.0, 1.12),
            ));
        });

    if config.lights {
        commands.spawn((
            DirectionalLight {
                illuminance: 12_000.0,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.6, 0.0)),
        ));
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.35, 0.16),
                intensity: 850_000.0,
                range: 12.0,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_xyz(-3.5, 3.0, 3.5),
        ));
    }
}

fn orbit_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut orbit: ResMut<Orbit>,
) {
    let delta = motion.read().map(|event| event.delta).sum::<Vec2>();
    if buttons.pressed(MouseButton::Right) {
        orbit.yaw -= delta.x * 0.006;
        orbit.pitch = (orbit.pitch - delta.y * 0.006).clamp(-1.3, 0.2);
    }
    for event in wheel.read() {
        orbit.radius = (orbit.radius - event.y * 0.6).clamp(4.0, 16.0);
    }
}

fn update_orbit_camera(orbit: Res<Orbit>, mut camera: Single<&mut Transform, With<OrbitCamera>>) {
    if orbit.is_changed() {
        **camera = orbit_transform(&orbit);
    }
}

fn orbit_transform(orbit: &Orbit) -> Transform {
    let rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
    let position = orbit.focus + rotation * Vec3::new(0.0, 0.0, orbit.radius);
    Transform::from_translation(position).looking_at(orbit.focus, Vec3::Y)
}

fn rotate_product(
    time: Res<Time>,
    config: Res<LessonConfig>,
    mut product: Query<&mut Transform, With<Product>>,
) {
    if !config.meshes {
        return;
    }
    for mut transform in &mut product {
        transform.rotate_y(time.delta_secs() * 0.18);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit_camera_stays_at_requested_radius() {
        let orbit = Orbit::default();
        let transform = orbit_transform(&orbit);
        let distance = transform.translation.distance(orbit.focus);
        assert!((distance - orbit.radius).abs() < 0.0001);
    }
}
