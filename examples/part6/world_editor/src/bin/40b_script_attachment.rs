use bevy::{
    asset::{io::Reader, AssetEvent, AssetLoader, AssetPlugin, LoadContext},
    prelude::*,
    reflect::TypePath,
    window::WindowResolution,
};
use serde::Deserialize;
use std::io::{Error, ErrorKind};

#[derive(Asset, TypePath, Debug, Deserialize)]
struct EditorScript {
    commands: Vec<ScriptCommand>,
}

#[derive(Debug, Deserialize)]
enum ScriptCommand {
    RotateY { degrees_per_second: f32 },
    Bob { height: f32, speed: f32 },
    Tint { rgb: [f32; 3] },
}

#[derive(Default, TypePath)]
struct EditorScriptLoader;

impl AssetLoader for EditorScriptLoader {
    type Asset = EditorScript;
    type Settings = ();
    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        ron::de::from_bytes(&bytes)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error.to_string()))
    }

    fn extensions(&self) -> &[&str] {
        &["editor_script"]
    }
}

#[derive(Component)]
struct AttachedScript(Handle<EditorScript>);

#[derive(Component)]
struct ScriptOrigin(Vec3);

#[derive(Component)]
struct ScriptStatus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                watch_for_changes_override: Some(true),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Script Attachment & Hot Reload - Bevy Practice".into(),
                    resolution: WindowResolution::new(960, 680),
                    ..default()
                }),
                ..default()
            }))
        .init_asset::<EditorScript>()
        .init_asset_loader::<EditorScriptLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, (run_attached_scripts, report_script_events))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let script = asset_server.load("scripts/spin_and_bob.editor_script");
    let origin = Vec3::new(0.0, 1.2, 0.0);
    commands.spawn((
        AttachedScript(script),
        ScriptOrigin(origin),
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.62, 1.0))),
        Transform::from_translation(origin),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(14.0, 14.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.12, 0.16, 0.18))),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.55, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.0, 5.5, 9.0).looking_at(Vec3::Y, Vec3::Y),
    ));
    commands.spawn((
        ScriptStatus,
        Text::new("SCRIPT: loading..."),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: px(18),
            top: px(18),
            ..default()
        },
    ));
}

fn run_attached_scripts(
    time: Res<Time>,
    scripts: Res<Assets<EditorScript>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities: Query<(
        &AttachedScript,
        &ScriptOrigin,
        &MeshMaterial3d<StandardMaterial>,
        &mut Transform,
    )>,
) {
    for (attached, origin, material_handle, mut transform) in &mut entities {
        let Some(script) = scripts.get(&attached.0) else {
            continue;
        };
        for command in &script.commands {
            match *command {
                ScriptCommand::RotateY { degrees_per_second } => {
                    transform.rotate_y(degrees_per_second.to_radians() * time.delta_secs());
                }
                ScriptCommand::Bob { height, speed } => {
                    transform.translation.y =
                        origin.0.y + (time.elapsed_secs() * speed).sin() * height;
                }
                ScriptCommand::Tint { rgb } => {
                    if let Some(mut material) = materials.get_mut(material_handle) {
                        material.base_color = Color::srgb(rgb[0], rgb[1], rgb[2]);
                    }
                }
            }
        }
    }
}

fn report_script_events(
    mut events: MessageReader<AssetEvent<EditorScript>>,
    attached: Single<&AttachedScript>,
    scripts: Res<Assets<EditorScript>>,
    mut status: Single<&mut Text, With<ScriptStatus>>,
) {
    for event in events.read() {
        let changed = event.is_added(attached.0.id())
            || event.is_modified(attached.0.id())
            || event.is_loaded_with_dependencies(attached.0.id());
        if !changed {
            continue;
        }
        if let Some(script) = scripts.get(&attached.0) {
            info!(
                commands = script.commands.len(),
                "editor script loaded or reloaded"
            );
            status.0 = format!(
                "SCRIPT ATTACHED: scripts/spin_and_bob.editor_script\nCOMMANDS: {}\n파일을 저장하면 실행 중 다시 로드됩니다.",
                script.commands.len()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_declarative_script() {
        let source = br#"(
            commands: [
                RotateY(degrees_per_second: 90.0),
                Bob(height: 0.3, speed: 2.0),
                Tint(rgb: (0.2, 0.6, 1.0)),
            ],
        )"#;
        let script: EditorScript = ron::de::from_bytes(source).unwrap();
        assert_eq!(script.commands.len(), 3);
    }
}
