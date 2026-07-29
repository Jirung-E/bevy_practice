use bevy::{
    asset::AssetPlugin,
    image::ImageLoaderSettings,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    window::WindowResolution,
};

const SHADER_PATH: &str = "shaders/30a_custom_pbr.wgsl";
const BASE_COLOR_PATH: &str = "textures/sci_fi_panel/base_color_clamp.png";
const NORMAL_PATH: &str = "textures/sci_fi_panel/normal_clamp.png";
const MASK_PATH: &str = "textures/sci_fi_panel/emissive_clamp.png";

type CustomPbrMaterial = ExtendedMaterial<StandardMaterial, PulseExtension>;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PulseExtension {
    // x: time, y: vertex displacement, z: emissive pulse, w: tint mix
    #[uniform(100)]
    effect: Vec4,
    #[uniform(101)]
    tint: LinearRgba,
    #[texture(102)]
    #[sampler(103)]
    mask_texture: Handle<Image>,
}

impl MaterialExtension for PulseExtension {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

struct CustomPbrPlugin;

impl Plugin for CustomPbrPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomPbrMaterial>::default())
            .init_resource::<EffectSettings>()
            .add_systems(Update, (handle_effect_input, update_effect_uniform));
    }
}

#[derive(Resource)]
struct EffectSettings {
    vertex_enabled: bool,
    fragment_enabled: bool,
    tint_index: usize,
}

impl Default for EffectSettings {
    fn default() -> Self {
        Self {
            vertex_enabled: true,
            fragment_enabled: true,
            tint_index: 0,
        }
    }
}

#[derive(Resource)]
struct CustomMaterialHandle(Handle<CustomPbrMaterial>);

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
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Custom PBR Material - Bevy Practice".into(),
                        resolution: WindowResolution::new(1100, 720),
                        ..default()
                    }),
                    ..default()
                }),
            CustomPbrPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.032)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomPbrMaterial>>,
) {
    let base_color = asset_server.load(BASE_COLOR_PATH);
    let normal = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load(NORMAL_PATH);
    let mask = asset_server.load(MASK_PATH);

    let mut sphere = Sphere::new(1.25).mesh().uv(64, 36);
    sphere
        .generate_tangents()
        .expect("UV sphere에는 tangent 생성에 필요한 속성이 있어야 합니다");
    let sphere = meshes.add(sphere);

    let standard = StandardMaterial {
        base_color_texture: Some(base_color.clone()),
        normal_map_texture: Some(normal.clone()),
        metallic: 0.45,
        perceptual_roughness: 0.32,
        ..default()
    };

    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(standard_materials.add(standard.clone())),
        Transform::from_xyz(-1.65, 0.1, 0.0),
    ));

    let custom = custom_materials.add(ExtendedMaterial {
        base: standard,
        extension: PulseExtension {
            effect: Vec4::new(0.0, 0.13, 1.4, 0.65),
            tint: tint_color(0),
            mask_texture: mask,
        },
    });
    commands.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(custom.clone()),
        Transform::from_xyz(1.65, 0.1, 0.0),
    ));
    commands.insert_resource(CustomMaterialHandle(custom));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.4, 7.7).looking_at(Vec3::new(0.0, 0.1, 0.0), Vec3::Y),
        AmbientLight {
            color: Color::srgb(0.15, 0.2, 0.35),
            brightness: 160.0,
            ..default()
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 11_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, -0.65, 0.0)),
    ));
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.38, 0.14),
            intensity: 420_000.0,
            range: 12.0,
            ..default()
        },
        Transform::from_xyz(-3.2, 2.8, 3.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 8.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.035, 0.045, 0.07),
            perceptual_roughness: 0.82,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.35, 0.0),
    ));

    spawn_ui(&mut commands);
}

fn spawn_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("CUSTOM PBR MATERIAL\nV: VERTEX   F: FRAGMENT   C: TINT"),
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
        Text::new("STANDARD MATERIAL"),
        TextFont {
            font_size: FontSize::Px(19.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(58),
            left: percent(19.0),
            ..default()
        },
    ));
    commands.spawn((
        Text::new("EXTENDED MATERIAL"),
        TextFont {
            font_size: FontSize::Px(19.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(58),
            left: percent(66.0),
            ..default()
        },
    ));
    commands.spawn((
        StatusText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(19.0),
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

fn handle_effect_input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<EffectSettings>) {
    if keys.just_pressed(KeyCode::KeyV) {
        settings.vertex_enabled = !settings.vertex_enabled;
    }
    if keys.just_pressed(KeyCode::KeyF) {
        settings.fragment_enabled = !settings.fragment_enabled;
    }
    if keys.just_pressed(KeyCode::KeyC) {
        settings.tint_index = (settings.tint_index + 1) % 3;
    }
}

fn update_effect_uniform(
    time: Res<Time>,
    settings: Res<EffectSettings>,
    handle: Res<CustomMaterialHandle>,
    mut materials: ResMut<Assets<CustomPbrMaterial>>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    if let Some(mut material) = materials.get_mut(&handle.0) {
        material.extension.effect = Vec4::new(
            time.elapsed_secs(),
            if settings.vertex_enabled { 0.13 } else { 0.0 },
            if settings.fragment_enabled { 1.4 } else { 0.0 },
            if settings.fragment_enabled { 0.65 } else { 0.0 },
        );
        material.extension.tint = tint_color(settings.tint_index);
    }

    status.0 = format!(
        "VERTEX: {}   |   FRAGMENT: {}   |   TINT: {}",
        on_off(settings.vertex_enabled),
        on_off(settings.fragment_enabled),
        ["CYAN", "ORANGE", "MAGENTA"][settings.tint_index],
    );
}

fn tint_color(index: usize) -> LinearRgba {
    [
        LinearRgba::rgb(0.1, 1.3, 2.6),
        LinearRgba::rgb(2.8, 0.55, 0.08),
        LinearRgba::rgb(2.0, 0.12, 1.4),
    ][index % 3]
}

fn on_off(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tint_cycle_stays_in_bounds() {
        for index in 0..12 {
            let _ = tint_color(index);
        }
    }

    #[test]
    fn status_uses_explicit_on_off_words() {
        assert_eq!(on_off(true), "ON");
        assert_eq!(on_off(false), "OFF");
    }
}
