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

const SHADER_PATH: &str = "shaders/13b_pipeline_solution.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PipelineSolutionMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    options: Vec4,
}

impl Material2d for PipelineSolutionMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

#[derive(Resource)]
struct DemoMaterial(Handle<PipelineSolutionMaterial>);

#[derive(Resource, Debug)]
struct DemoOptions {
    vertex_enabled: bool,
    fragment_enabled: bool,
    wireframe_enabled: bool,
    shear: f32,
}

impl Default for DemoOptions {
    fn default() -> Self {
        Self {
            vertex_enabled: false,
            fragment_enabled: false,
            wireframe_enabled: false,
            shear: 90.0,
        }
    }
}

impl DemoOptions {
    fn uniform(&self, elapsed: f32) -> Vec4 {
        Vec4::new(
            f32::from(self.vertex_enabled),
            f32::from(self.fragment_enabled),
            elapsed,
            self.shear,
        )
    }
}

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
                }),
            Material2dPlugin::<PipelineSolutionMaterial>::default(),
            Wireframe2dPlugin::default(),
        ))
        .init_resource::<DemoOptions>()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.07)))
        .insert_resource(Wireframe2dConfig {
            global: false,
            default_color: Color::srgb(1.0, 0.82, 0.15),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_uniform).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PipelineSolutionMaterial>>,
) {
    commands.spawn(Camera2d);
    let material = materials.add(PipelineSolutionMaterial {
        base_color: LinearRgba::new(0.12, 0.72, 1.0, 1.0),
        options: Vec4::new(0.0, 0.0, 0.0, 90.0),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(380.0, 260.0))),
        MeshMaterial2d(material.clone()),
    ));
    commands.insert_resource(DemoMaterial(material));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<DemoOptions>,
    mut wireframe: ResMut<Wireframe2dConfig>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        options.vertex_enabled = !options.vertex_enabled;
    }
    if keyboard.just_pressed(KeyCode::KeyF) {
        options.fragment_enabled = !options.fragment_enabled;
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        options.wireframe_enabled = !options.wireframe_enabled;
        wireframe.global = options.wireframe_enabled;
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        options.shear = 90.0;
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        options.shear = -120.0;
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        options.shear = 30.0;
    }
}

fn update_uniform(
    time: Res<Time>,
    options: Res<DemoOptions>,
    demo: Res<DemoMaterial>,
    mut materials: ResMut<Assets<PipelineSolutionMaterial>>,
) {
    if let Some(mut material) = materials.get_mut(&demo.0) {
        material.options = options.uniform(time.elapsed_secs());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_pipeline_combinations_map_to_independent_uniform_flags() {
        for (vertex, fragment, expected) in [
            (false, false, Vec2::new(0.0, 0.0)),
            (true, false, Vec2::new(1.0, 0.0)),
            (false, true, Vec2::new(0.0, 1.0)),
            (true, true, Vec2::new(1.0, 1.0)),
        ] {
            let options = DemoOptions {
                vertex_enabled: vertex,
                fragment_enabled: fragment,
                wireframe_enabled: false,
                shear: -120.0,
            };
            let uniform = options.uniform(2.5);
            assert_eq!(uniform.xy(), expected);
            assert_eq!(uniform.z, 2.5);
            assert_eq!(uniform.w, -120.0);
        }
    }
}
