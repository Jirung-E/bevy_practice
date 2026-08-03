use bevy::{
    camera::{visibility::RenderLayers, Viewport},
    prelude::*,
    window::{WindowResized, WindowResolution},
};

const WORLD_LAYER: usize = 0;
const MINIMAP_LAYER: usize = 1;
const MINIMAP_SIZE: u32 = 240;
const MINIMAP_MARGIN: u32 = 18;

#[derive(Component)]
struct MinimapCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Multi Camera & RenderLayers - Bevy Practice".into(),
                resolution: WindowResolution::new(1000, 700),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, resize_minimap)
        .run();
}

fn setup(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let main_camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(9.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    commands.spawn((
        MinimapCamera,
        Camera3d::default(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.025, 0.035, 0.055)),
            viewport: minimap_viewport(&window),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scale: 0.045,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 18.0, 0.01).looking_at(Vec3::ZERO, Vec3::NEG_Z),
        RenderLayers::from_layers(&[WORLD_LAYER, MINIMAP_LAYER]),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 16.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.12, 0.18, 0.22))),
    ));

    let cube_mesh = meshes.add(Cuboid::new(1.4, 1.4, 1.4));
    let marker_mesh = meshes.add(Cylinder::new(0.55, 0.08));
    for (position, color) in [
        (Vec3::new(-3.5, 0.7, -2.0), Color::srgb(0.2, 0.65, 1.0)),
        (Vec3::new(2.5, 0.7, 2.8), Color::srgb(1.0, 0.35, 0.2)),
        (Vec3::new(3.8, 0.7, -3.0), Color::srgb(0.4, 1.0, 0.45)),
    ] {
        commands.spawn((
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(materials.add(color)),
            Transform::from_translation(position),
        ));
        commands.spawn((
            Mesh3d(marker_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                ..default()
            })),
            Transform::from_translation(position + Vec3::Y * 1.2),
            RenderLayers::layer(MINIMAP_LAYER),
        ));
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.7, 0.0)),
        RenderLayers::from_layers(&[WORLD_LAYER, MINIMAP_LAYER]),
    ));

    commands.spawn((
        UiTargetCamera(main_camera),
        Text::new("MAIN CAMERA\nTOP RIGHT: WORLD + MINIMAP MARKERS"),
        TextFont {
            font_size: FontSize::Px(22.0),
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

fn minimap_viewport(window: &Window) -> Option<Viewport> {
    let width = window.physical_width();
    let height = window.physical_height();
    let size = MINIMAP_SIZE.min(width).min(height);
    Some(Viewport {
        physical_position: UVec2::new(width.saturating_sub(size + MINIMAP_MARGIN), MINIMAP_MARGIN),
        physical_size: UVec2::splat(size),
        ..default()
    })
}

fn resize_minimap(
    mut resized: MessageReader<WindowResized>,
    window: Single<&Window>,
    mut camera: Single<&mut Camera, With<MinimapCamera>>,
) {
    if resized.read().next().is_some() {
        camera.viewport = minimap_viewport(&window);
    }
}
