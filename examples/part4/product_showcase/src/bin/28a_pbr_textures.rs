use bevy::{
    asset::{AssetPlugin, RenderAssetUsages},
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
    window::WindowResolution,
};

const BASE_REPEAT: &str = "textures/sci_fi_panel/base_color.png";
const NORMAL_REPEAT: &str = "textures/sci_fi_panel/normal.png";
const EMISSIVE_REPEAT: &str = "textures/sci_fi_panel/emissive.png";
const BASE_CLAMP: &str = "textures/sci_fi_panel/base_color_clamp.png";
const NORMAL_CLAMP: &str = "textures/sci_fi_panel/normal_clamp.png";
const EMISSIVE_CLAMP: &str = "textures/sci_fi_panel/emissive_clamp.png";

#[derive(Component)]
struct Panel {
    repeated_uv: bool,
}

#[derive(Component)]
struct StatusText;

#[derive(Resource)]
struct MaterialSets {
    clamp: [Handle<StandardMaterial>; 4],
    repeat: [Handle<StandardMaterial>; 4],
}

#[derive(Resource)]
struct DisplaySettings {
    map_mode: usize,
    repeat_address: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            map_mode: 3,
            repeat_address: true,
        }
    }
}

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
                        title: "UV & PBR Textures - Bevy Practice".into(),
                        resolution: WindowResolution::new(1100, 720),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.032)))
        .init_resource::<DisplaySettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, switch_display)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let clamp_textures = [
        asset_server.load(BASE_CLAMP),
        load_clamp_linear(&asset_server, NORMAL_CLAMP),
        asset_server.load(EMISSIVE_CLAMP),
    ];
    let repeat_textures = [
        load_repeat(&asset_server, BASE_REPEAT, true),
        load_repeat(&asset_server, NORMAL_REPEAT, false),
        load_repeat(&asset_server, EMISSIVE_REPEAT, true),
    ];

    let sets = MaterialSets {
        clamp: make_material_set(&mut materials, &clamp_textures),
        repeat: make_material_set(&mut materials, &repeat_textures),
    };

    let uv_one = meshes.add(panel_mesh(1.0));
    let uv_three = meshes.add(panel_mesh(3.0));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.15, 8.2).looking_at(Vec3::ZERO, Vec3::Y),
        AmbientLight {
            color: Color::srgb(0.18, 0.22, 0.35),
            brightness: 180.0,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.55, -0.75, 0.0)),
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.45, 0.2),
            intensity: 380_000.0,
            range: 10.0,
            ..default()
        },
        Transform::from_xyz(-2.8, 3.0, 3.2),
    ));

    commands.spawn((
        Panel { repeated_uv: false },
        Mesh3d(uv_one),
        MeshMaterial3d(sets.clamp[3].clone()),
        Transform::from_xyz(-1.65, -0.05, 0.0),
    ));
    commands.spawn((
        Panel { repeated_uv: true },
        Mesh3d(uv_three),
        MeshMaterial3d(sets.repeat[3].clone()),
        Transform::from_xyz(1.65, -0.05, 0.0),
    ));

    spawn_ui(&mut commands);
    commands.insert_resource(sets);
}

fn load_clamp_linear(asset_server: &AssetServer, path: &'static str) -> Handle<Image> {
    asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load(path)
}

fn load_repeat(asset_server: &AssetServer, path: &'static str, is_srgb: bool) -> Handle<Image> {
    asset_server
        .load_builder()
        .with_settings(move |settings: &mut ImageLoaderSettings| {
            settings.is_srgb = is_srgb;
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            });
        })
        .load(path)
}

fn make_material_set(
    materials: &mut Assets<StandardMaterial>,
    textures: &[Handle<Image>; 3],
) -> [Handle<StandardMaterial>; 4] {
    let common = || StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.35,
        perceptual_roughness: 0.38,
        ..default()
    };

    [
        materials.add(StandardMaterial {
            base_color_texture: Some(textures[0].clone()),
            ..common()
        }),
        materials.add(StandardMaterial {
            base_color_texture: Some(textures[0].clone()),
            normal_map_texture: Some(textures[1].clone()),
            ..common()
        }),
        materials.add(StandardMaterial {
            base_color_texture: Some(textures[0].clone()),
            emissive: LinearRgba::WHITE,
            emissive_texture: Some(textures[2].clone()),
            ..common()
        }),
        materials.add(StandardMaterial {
            base_color_texture: Some(textures[0].clone()),
            normal_map_texture: Some(textures[1].clone()),
            emissive: LinearRgba::WHITE,
            emissive_texture: Some(textures[2].clone()),
            ..common()
        }),
    ]
}

fn panel_mesh(uv_scale: f32) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-1.4, -1.4, 0.0],
            [1.4, -1.4, 0.0],
            [1.4, 1.4, 0.0],
            [-1.4, 1.4, 0.0],
        ],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, uv_scale],
            [uv_scale, uv_scale],
            [uv_scale, 0.0],
            [0.0, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));

    mesh.generate_tangents()
        .expect("UV와 법선이 있으므로 tangent 생성에 성공해야 합니다");
    mesh
}

fn spawn_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new(
            "UV & PBR TEXTURE MAPPING\n1: BASE   2: +NORMAL   3: +EMISSIVE   4: ALL   A: ADDRESS",
        ),
        TextFont {
            font_size: FontSize::Px(22.0),
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
        Text::new("UV 0..1"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(55),
            left: percent(23.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("UV 0..3"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(55),
            left: percent(70.0),
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
        TextColor(Color::srgb(1.0, 0.82, 0.24)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(16),
            left: px(18),
            ..default()
        },
    ));
}

fn switch_display(
    keys: Res<ButtonInput<KeyCode>>,
    sets: Res<MaterialSets>,
    mut settings: ResMut<DisplaySettings>,
    mut panels: Query<(&Panel, &mut MeshMaterial3d<StandardMaterial>)>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        settings.map_mode = 0;
    } else if keys.just_pressed(KeyCode::Digit2) {
        settings.map_mode = 1;
    } else if keys.just_pressed(KeyCode::Digit3) {
        settings.map_mode = 2;
    } else if keys.just_pressed(KeyCode::Digit4) {
        settings.map_mode = 3;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        settings.repeat_address = !settings.repeat_address;
    }

    if settings.is_changed() {
        for (panel, mut material) in &mut panels {
            let use_repeat = panel.repeated_uv && settings.repeat_address;
            let source = if use_repeat {
                &sets.repeat
            } else {
                &sets.clamp
            };
            material.0 = source[settings.map_mode].clone();
        }
    }

    let map_name = ["BASE", "BASE + NORMAL", "BASE + EMISSIVE", "ALL"][settings.map_mode];
    let address = if settings.repeat_address {
        "RIGHT: REPEAT"
    } else {
        "RIGHT: CLAMP"
    };
    status.0 = format!("MAP: {map_name}   |   {address}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_mesh_contains_required_pbr_attributes() {
        let mesh = panel_mesh(3.0);
        assert!(mesh.contains_attribute(Mesh::ATTRIBUTE_POSITION));
        assert!(mesh.contains_attribute(Mesh::ATTRIBUTE_NORMAL));
        assert!(mesh.contains_attribute(Mesh::ATTRIBUTE_UV_0));
        assert!(mesh.contains_attribute(Mesh::ATTRIBUTE_TANGENT));
        assert_eq!(mesh.count_vertices(), 4);
    }
}
