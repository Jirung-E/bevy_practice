use bevy::{
    asset::AssetPlugin,
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

const SHADER_PATH: &str = "shaders/20b_starfield.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StarfieldMaterial {
    // x: elapsed time, y: scroll speed, z: star density, w: layer separation
    #[uniform(0)]
    options: Vec4,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

#[derive(Resource)]
struct StarfieldHandle(Handle<StarfieldMaterial>);

#[derive(Resource, Debug)]
struct StarfieldSettings {
    paused: bool,
    speed: f32,
    density: f32,
    elapsed: f32,
}

impl Default for StarfieldSettings {
    fn default() -> Self {
        Self {
            paused: false,
            speed: 0.18,
            density: 0.1,
            elapsed: 0.0,
        }
    }
}

impl StarfieldSettings {
    fn options(&self) -> Vec4 {
        Vec4::new(self.elapsed, self.speed, self.density, 0.42)
    }
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
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Procedural Starfield - Bevy Practice".into(),
                        resolution: (960, 640).into(),
                        ..default()
                    }),
                    ..default()
                }),
            Material2dPlugin::<StarfieldMaterial>::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<StarfieldSettings>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (control_starfield, update_starfield, update_status).chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
) {
    commands.spawn(Camera2d);

    let material = materials.add(StarfieldMaterial {
        options: StarfieldSettings::default().options(),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(960.0, 640.0))),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    commands.insert_resource(StarfieldHandle(material));

    // 셰이더 배경 위에서도 게임 오브젝트의 윤곽이 읽히는지 확인한다.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.15, 0.82, 1.0), Vec2::new(44.0, 36.0)),
        Transform::from_xyz(0.0, -225.0, 1.0),
    ));
    for (x, y) in [(-270.0, 155.0), (40.0, 225.0), (300.0, 95.0)] {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.24, 0.34), Vec2::splat(38.0)),
            Transform::from_xyz(x, y, 1.0),
        ));
    }

    commands.spawn((
        Text::new("PROCEDURAL STARFIELD\nWGSL CREATES EVERY STAR"),
        TextFont {
            font_size: FontSize::Px(27.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.92, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            top: px(16),
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
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            bottom: px(18),
            ..default()
        },
    ));
}

fn control_starfield(keyboard: Res<ButtonInput<KeyCode>>, mut settings: ResMut<StarfieldSettings>) {
    if keyboard.just_pressed(KeyCode::Space) {
        settings.paused = !settings.paused;
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        settings.speed = (settings.speed + 0.05).min(0.6);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        settings.speed = (settings.speed - 0.05).max(0.0);
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        settings.density = (settings.density + 0.02).min(0.24);
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        settings.density = (settings.density - 0.02).max(0.02);
    }
}

fn update_starfield(
    time: Res<Time>,
    mut settings: ResMut<StarfieldSettings>,
    starfield: Res<StarfieldHandle>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
) {
    if !settings.paused {
        settings.elapsed += time.delta_secs();
    }

    if let Some(mut material) = materials.get_mut(&starfield.0) {
        material.options = settings.options();
    }
}

fn update_status(
    settings: Res<StarfieldSettings>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    status.0 = format!(
        "SPACE: {}  |  UP/DOWN: SPEED {:.2}  |  LEFT/RIGHT: DENSITY {:.2}",
        if settings.paused { "RESUME" } else { "PAUSE" },
        settings.speed,
        settings.density
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_map_to_shader_uniform_slots() {
        let settings = StarfieldSettings {
            paused: false,
            speed: 0.31,
            density: 0.14,
            elapsed: 2.5,
        };

        assert_eq!(settings.options(), Vec4::new(2.5, 0.31, 0.14, 0.42));
    }
}
