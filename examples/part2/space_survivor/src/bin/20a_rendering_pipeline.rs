use bevy::{
    asset::AssetPlugin,
    prelude::*,
    reflect::TypePath,
    render::{
        RenderPlugin,
        render_resource::{AsBindGroup, WgpuFeatures},
        settings::WgpuSettings,
    },
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin, Wireframe2dConfig, Wireframe2dPlugin},
};

const SHADER_PATH: &str = "shaders/20a_pipeline.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PipelineMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    // x: vertex 변형 사용 여부, y: fragment 그라데이션 사용 여부
    #[uniform(1)]
    options: Vec4,
}

impl Material2d for PipelineMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

#[derive(Resource)]
struct DemoMaterial(Handle<PipelineMaterial>);

#[derive(Resource, Default)]
struct DemoOptions {
    vertex_enabled: bool,
    fragment_enabled: bool,
    wireframe_enabled: bool,
}

#[derive(Component)]
struct StatusText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "2D Rendering Pipeline - Bevy Practice".into(),
                        resolution: (960, 640).into(),
                        ..default()
                    }),
                    ..default()
                }),
            Material2dPlugin::<PipelineMaterial>::default(),
            Wireframe2dPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.07)))
        .insert_resource(Wireframe2dConfig {
            global: false,
            default_color: Color::srgb(1.0, 0.82, 0.15),
        })
        .init_resource::<DemoOptions>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_status))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PipelineMaterial>>,
) {
    commands.spawn(Camera2d);

    let material = materials.add(PipelineMaterial {
        base_color: LinearRgba::new(0.12, 0.72, 1.0, 1.0),
        options: Vec4::ZERO,
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(380.0, 260.0))),
        MeshMaterial2d(material.clone()),
    ));
    commands.insert_resource(DemoMaterial(material));

    commands.spawn((
        Text::new("2D RENDERING PIPELINE"),
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
        Text::new("V: VERTEX OFF  |  F: FRAGMENT OFF  |  W: WIREFRAME OFF"),
        TextFont {
            font_size: FontSize::Px(22.0),
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

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<DemoOptions>,
    demo_material: Res<DemoMaterial>,
    mut materials: ResMut<Assets<PipelineMaterial>>,
    mut wireframe: ResMut<Wireframe2dConfig>,
) {
    let mut changed = false;
    if keyboard.just_pressed(KeyCode::KeyV) {
        options.vertex_enabled = !options.vertex_enabled;
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::KeyF) {
        options.fragment_enabled = !options.fragment_enabled;
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        options.wireframe_enabled = !options.wireframe_enabled;
        wireframe.global = options.wireframe_enabled;
    }

    if changed && let Some(mut material) = materials.get_mut(&demo_material.0) {
        material.options.x = f32::from(options.vertex_enabled);
        material.options.y = f32::from(options.fragment_enabled);
    }
}

fn update_status(options: Res<DemoOptions>, mut status: Single<&mut Text, With<StatusText>>) {
    if !options.is_changed() {
        return;
    }
    status.0 = format!(
        "V: VERTEX {}  |  F: FRAGMENT {}  |  W: WIREFRAME {}",
        if options.vertex_enabled { "ON" } else { "OFF" },
        if options.fragment_enabled {
            "ON"
        } else {
            "OFF"
        },
        if options.wireframe_enabled {
            "ON"
        } else {
            "OFF"
        },
    );
}
