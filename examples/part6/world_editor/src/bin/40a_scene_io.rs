use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    asset::{AssetPath, LoadFromPath, UntypedHandle},
    ecs::entity::EntityHashMap,
    prelude::*,
    window::WindowResolution,
    world_serialization::serde::WorldDeserializer,
};
use serde::de::DeserializeSeed;

const SCENE_VERSION: u32 = 1;
const SCENE_HEADER: &str = "// world_editor_scene_version: 1";

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reflect(Component)]
struct SceneId(u64);

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
struct SceneName(String);

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
enum SceneAssetKind {
    Cube,
    Sphere,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
struct SceneParent(Option<u64>);

#[derive(Component)]
struct Editable;

#[derive(Component)]
struct HierarchyText;

#[derive(Component)]
struct InspectorText;

#[derive(Component)]
struct DocumentText;

#[derive(Resource, Default)]
struct Selection(Option<Entity>);

#[derive(Resource)]
struct SceneDocument {
    path: PathBuf,
    dirty: bool,
    next_id: u64,
    message: String,
}

#[derive(Resource)]
struct EditorAssets {
    cube: Handle<Mesh>,
    sphere: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
    sphere_material: Handle<StandardMaterial>,
}

#[derive(Debug, Clone, PartialEq)]
struct SceneRecord {
    id: u64,
    name: String,
    kind: SceneAssetKind,
    parent: Option<u64>,
    transform: Transform,
}

type EditableData = (
    Entity,
    &'static SceneId,
    &'static SceneName,
    &'static SceneAssetKind,
    &'static Transform,
    Option<&'static ChildOf>,
);

struct NoAssetLoader;

impl LoadFromPath for NoAssetLoader {
    fn load_from_path_erased(
        &mut self,
        _type_id: TypeId,
        _path: AssetPath<'static>,
    ) -> UntypedHandle {
        panic!("Scene 문서에는 런타임 Asset Handle을 직접 저장하지 않습니다")
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "World Editor Scene I/O - Bevy Practice".into(),
                resolution: WindowResolution::new(1280, 800),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.018, 0.022, 0.032)))
        .init_resource::<Selection>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_shortcuts,
                update_hierarchy,
                update_inspector,
                update_document_status,
                draw_selection,
            )
                .chain(),
        )
        .run();
}

fn scene_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/world_editor_scene.scn.ron")
}

fn registered_app() -> App {
    let mut app = App::new();
    app.register_type::<SceneId>()
        .register_type::<SceneName>()
        .register_type::<SceneAssetKind>()
        .register_type::<SceneParent>()
        .register_type::<Transform>();
    app
}

fn setup(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.0, 7.0, 11.0).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
        AmbientLight {
            color: Color::srgb(0.28, 0.34, 0.48),
            brightness: 190.0,
            ..default()
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.5, 0.0)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.12, 0.14))),
    ));

    let assets = EditorAssets {
        cube: meshes.add(Cuboid::from_length(1.5)),
        sphere: meshes.add(Sphere::new(0.9)),
        cube_material: materials.add(Color::srgb(0.12, 0.52, 0.9)),
        sphere_material: materials.add(Color::srgb(0.9, 0.3, 0.12)),
    };
    let path = scene_path();
    let mut document = SceneDocument {
        path: path.clone(),
        dirty: false,
        next_id: 1,
        message: "New scene".into(),
    };

    let records = if path.exists() {
        match load_records(&path) {
            Ok(records) => {
                document.message = format!("Opened {}", path.display());
                records
            }
            Err(error) => {
                document.message = format!("OPEN ERROR: {error}");
                default_records()
            }
        }
    } else {
        default_records()
    };
    spawn_records(
        &mut commands,
        &assets,
        &mut selection,
        &mut document,
        records,
    );
    commands.insert_resource(assets);
    commands.insert_resource(document);
    spawn_ui(&mut commands);
}

fn default_records() -> Vec<SceneRecord> {
    vec![
        SceneRecord {
            id: 1,
            name: "Blue Cube".into(),
            kind: SceneAssetKind::Cube,
            parent: None,
            transform: Transform::from_xyz(-2.0, 0.75, 0.0),
        },
        SceneRecord {
            id: 2,
            name: "Orange Sphere".into(),
            kind: SceneAssetKind::Sphere,
            parent: Some(1),
            transform: Transform::from_xyz(4.0, 0.15, -1.0),
        },
    ]
}

fn spawn_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("SCENE DOCUMENT\nCtrl+N: NEW   Ctrl+O: OPEN   Ctrl+S: SAVE"),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.88, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(14),
            left: px(270),
            ..default()
        },
    ));
    commands.spawn((
        Text::new(
            "Tab: SELECT   Arrows/PageUp/PageDown: MOVE\n1: CUBE   2: SPHERE   Delete: REMOVE",
        ),
        TextFont {
            font_size: FontSize::Px(17.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(16),
            left: px(270),
            ..default()
        },
    ));
    commands.spawn((
        HierarchyText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(17.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(18),
            width: px(235),
            ..default()
        },
    ));
    commands.spawn((
        InspectorText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(17.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            right: px(16),
            top: px(18),
            width: px(260),
            ..default()
        },
    ));
    commands.spawn((
        DocumentText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.82, 0.24)),
        Node {
            position_type: PositionType::Absolute,
            left: px(270),
            top: px(82),
            ..default()
        },
    ));
}

#[expect(
    clippy::type_complexity,
    reason = "읽기용 편집 Entity Query와 Transform 쓰기 Query를 ParamSet으로 분리한다"
)]
fn handle_shortcuts(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<EditorAssets>,
    mut document: ResMut<SceneDocument>,
    mut selection: ResMut<Selection>,
    mut editable_queries: ParamSet<(
        Query<EditableData, With<Editable>>,
        Query<&mut Transform, With<Editable>>,
    )>,
) {
    let control = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    if control && keys.just_pressed(KeyCode::KeyS) {
        match save_scene(&document.path, &editable_queries.p0()) {
            Ok(()) => {
                document.dirty = false;
                document.message = format!("Saved {}", document.path.display());
            }
            Err(error) => document.message = format!("SAVE ERROR: {error}"),
        }
        return;
    }
    if control && keys.just_pressed(KeyCode::KeyO) {
        match load_records(&document.path) {
            Ok(records) => {
                despawn_editables(&mut commands, &editable_queries.p0());
                spawn_records(
                    &mut commands,
                    &assets,
                    &mut selection,
                    &mut document,
                    records,
                );
                document.dirty = false;
                document.message = format!("Opened {}", document.path.display());
            }
            Err(error) => document.message = format!("OPEN ERROR: {error}"),
        }
        return;
    }
    if control && keys.just_pressed(KeyCode::KeyN) {
        despawn_editables(&mut commands, &editable_queries.p0());
        selection.0 = None;
        document.next_id = 1;
        document.dirty = true;
        document.message = "New empty scene".into();
        return;
    }

    if keys.just_pressed(KeyCode::Tab) {
        let mut entities = editable_queries
            .p0()
            .iter()
            .map(|row| (row.0, row.1.0))
            .collect::<Vec<_>>();
        entities.sort_by_key(|(_, id)| *id);
        if !entities.is_empty() {
            let current = selection
                .0
                .and_then(|entity| {
                    entities
                        .iter()
                        .position(|(candidate, _)| *candidate == entity)
                })
                .unwrap_or(entities.len() - 1);
            selection.0 = Some(entities[(current + 1) % entities.len()].0);
        }
    }

    let mut delta = Vec3::ZERO;
    if keys.just_pressed(KeyCode::ArrowLeft) {
        delta.x -= 0.25;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        delta.x += 0.25;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        delta.z -= 0.25;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        delta.z += 0.25;
    }
    if keys.just_pressed(KeyCode::PageUp) {
        delta.y += 0.25;
    }
    if keys.just_pressed(KeyCode::PageDown) {
        delta.y -= 0.25;
    }
    if delta != Vec3::ZERO
        && let Some(entity) = selection.0
        && let Ok(mut transform) = editable_queries.p1().get_mut(entity)
    {
        transform.translation += delta;
        document.dirty = true;
        document.message = format!("Moved object by {delta:.2}");
    }

    let kind = if keys.just_pressed(KeyCode::Digit1) {
        Some(SceneAssetKind::Cube)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(SceneAssetKind::Sphere)
    } else {
        None
    };
    if let Some(kind) = kind {
        let id = document.next_id;
        document.next_id += 1;
        let entity = spawn_record(
            &mut commands,
            &assets,
            SceneRecord {
                id,
                name: format!("New {kind:?}"),
                kind,
                parent: None,
                transform: Transform::from_xyz(0.0, kind.height(), 0.0),
            },
        );
        selection.0 = Some(entity);
        document.dirty = true;
        document.message = format!("Created object #{id}");
    }

    if keys.just_pressed(KeyCode::Delete)
        && let Some(entity) = selection.0.take()
    {
        commands.entity(entity).despawn();
        document.dirty = true;
        document.message = "Deleted selected object".into();
    }
}

impl SceneAssetKind {
    fn height(self) -> f32 {
        match self {
            Self::Cube => 0.75,
            Self::Sphere => 0.9,
        }
    }
}

fn despawn_editables(commands: &mut Commands, editables: &Query<EditableData, With<Editable>>) {
    for (entity, ..) in editables.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_records(
    commands: &mut Commands,
    assets: &EditorAssets,
    selection: &mut Selection,
    document: &mut SceneDocument,
    records: Vec<SceneRecord>,
) {
    let mut entities = HashMap::new();
    let mut parents = Vec::new();
    for record in records {
        document.next_id = document.next_id.max(record.id + 1);
        let parent = record.parent;
        let id = record.id;
        let entity = spawn_record(commands, assets, record);
        selection.0.get_or_insert(entity);
        entities.insert(id, entity);
        parents.push((entity, parent));
    }
    for (child, parent_id) in parents {
        if let Some(parent) = parent_id.and_then(|id| entities.get(&id).copied()) {
            commands.entity(child).insert(ChildOf(parent));
        }
    }
}

fn spawn_record(commands: &mut Commands, assets: &EditorAssets, record: SceneRecord) -> Entity {
    let (mesh, material) = match record.kind {
        SceneAssetKind::Cube => (assets.cube.clone(), assets.cube_material.clone()),
        SceneAssetKind::Sphere => (assets.sphere.clone(), assets.sphere_material.clone()),
    };
    commands
        .spawn((
            Editable,
            SceneId(record.id),
            SceneName(record.name),
            record.kind,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            record.transform,
        ))
        .id()
}

fn save_scene(path: &Path, editables: &Query<EditableData, With<Editable>>) -> Result<(), String> {
    let mut staging = registered_app();
    let ids = editables
        .iter()
        .map(|(entity, id, ..)| (entity, id.0))
        .collect::<HashMap<_, _>>();
    for (_entity, id, name, kind, transform, child_of) in editables.iter() {
        let parent = child_of
            .and_then(|relationship| ids.get(&relationship.parent()))
            .copied();
        staging
            .world_mut()
            .spawn((*id, name.clone(), *kind, SceneParent(parent), *transform));
    }
    let registry = staging.world().resource::<AppTypeRegistry>().read();
    let ron = DynamicWorld::from_world_with(staging.world(), &registry)
        .serialize(&registry)
        .map_err(|error| error.to_string())?;
    let serialized = format!("{SCENE_HEADER}\n{ron}");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(path, serialized).map_err(|error| error.to_string())
}

fn load_records(path: &Path) -> Result<Vec<SceneRecord>, String> {
    let serialized = fs::read_to_string(path).map_err(|error| error.to_string())?;
    deserialize_records(&serialized)
}

fn deserialize_records(serialized: &str) -> Result<Vec<SceneRecord>, String> {
    let Some(header) = serialized.lines().next() else {
        return Err("빈 Scene 파일입니다".into());
    };
    if header != SCENE_HEADER {
        return Err(format!(
            "지원하지 않는 Scene 버전입니다. 현재 버전: {SCENE_VERSION}"
        ));
    }

    let mut staging = registered_app();
    let registry = staging.world().resource::<AppTypeRegistry>().clone();
    let registry_guard = registry.read();
    let mut deserializer =
        ron::de::Deserializer::from_str(serialized).map_err(|error| error.to_string())?;
    let dynamic_world = WorldDeserializer {
        type_registry: &registry_guard,
        load_from_path: &mut NoAssetLoader,
    }
    .deserialize(&mut deserializer)
    .map_err(|error| error.to_string())?;
    drop(registry_guard);
    dynamic_world
        .write_to_world(staging.world_mut(), &mut EntityHashMap::default())
        .map_err(|error| error.to_string())?;

    let mut query = staging.world_mut().query::<(
        &SceneId,
        &SceneName,
        &SceneAssetKind,
        &SceneParent,
        &Transform,
    )>();
    let mut records = query
        .iter(staging.world())
        .map(|(id, name, kind, parent, transform)| SceneRecord {
            id: id.0,
            name: name.0.clone(),
            kind: *kind,
            parent: parent.0,
            transform: *transform,
        })
        .collect::<Vec<_>>();
    validate_records(&records)?;
    records.sort_by_key(|record| record.id);
    Ok(records)
}

fn validate_records(records: &[SceneRecord]) -> Result<(), String> {
    let ids = records
        .iter()
        .map(|record| record.id)
        .collect::<HashSet<_>>();
    if ids.len() != records.len() {
        return Err("중복된 SceneId가 있습니다".into());
    }
    for record in records {
        if record.parent == Some(record.id) {
            return Err(format!(
                "Entity #{}가 자기 자신을 부모로 참조합니다",
                record.id
            ));
        }
        if let Some(parent) = record.parent
            && !ids.contains(&parent)
        {
            return Err(format!(
                "Entity #{}의 부모 #{}를 찾을 수 없습니다",
                record.id, parent
            ));
        }
    }
    Ok(())
}

fn update_hierarchy(
    selection: Res<Selection>,
    objects: Query<(Entity, &SceneId, &SceneName, Option<&ChildOf>), With<Editable>>,
    names: Query<&SceneName, With<Editable>>,
    mut text: Single<&mut Text, With<HierarchyText>>,
) {
    let mut rows = objects
        .iter()
        .map(|(entity, id, name, child_of)| {
            let selected = if selection.0 == Some(entity) {
                ">"
            } else {
                " "
            };
            let parent = child_of
                .and_then(|relationship| names.get(relationship.parent()).ok())
                .map_or(String::new(), |name| format!("  ↳ parent: {}", name.0));
            format!("{selected} #{} {}{parent}", id.0, name.0)
        })
        .collect::<Vec<_>>();
    rows.sort();
    text.0 = format!(
        "HIERARCHY\n\n{}",
        if rows.is_empty() {
            "(empty scene)".into()
        } else {
            rows.join("\n")
        }
    );
}

fn update_inspector(
    selection: Res<Selection>,
    objects: Query<(&SceneId, &SceneName, &SceneAssetKind, &Transform), With<Editable>>,
    mut text: Single<&mut Text, With<InspectorText>>,
) {
    text.0 = selection
        .0
        .and_then(|entity| objects.get(entity).ok())
        .map_or_else(
            || "INSPECTOR\n\nNothing selected".into(),
            |(id, name, kind, transform)| {
                format!(
                    "INSPECTOR\n\n#{} {}\n{:?}\n\nTransform\nX: {:.2}\nY: {:.2}\nZ: {:.2}",
                    id.0,
                    name.0,
                    kind,
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z
                )
            },
        );
}

fn update_document_status(
    document: Res<SceneDocument>,
    mut text: Single<&mut Text, With<DocumentText>>,
) {
    text.0 = format!(
        "{}  |  {}\n{}",
        if document.dirty {
            "MODIFIED *"
        } else {
            "SAVED"
        },
        document.path.display(),
        document.message
    );
}

fn draw_selection(
    selection: Res<Selection>,
    objects: Query<&GlobalTransform, With<Editable>>,
    mut gizmos: Gizmos,
) {
    let Some(transform) = selection
        .0
        .and_then(|entity| objects.get(entity).ok())
        .map(GlobalTransform::compute_transform)
    else {
        return;
    };
    gizmos.axes(transform, 1.4);
    gizmos.cube(
        Transform::from_translation(transform.translation).with_scale(Vec3::splat(1.9)),
        Color::srgb(1.0, 0.85, 0.1),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    type SaveQuery = Query<'static, 'static, EditableData, With<Editable>>;

    fn serialize_records(records: &[SceneRecord]) -> String {
        let mut app = registered_app();
        for record in records {
            app.world_mut().spawn((
                SceneId(record.id),
                SceneName(record.name.clone()),
                record.kind,
                SceneParent(record.parent),
                record.transform,
            ));
        }
        let registry = app.world().resource::<AppTypeRegistry>().read();
        let ron = DynamicWorld::from_world_with(app.world(), &registry)
            .serialize(&registry)
            .unwrap();
        format!("{SCENE_HEADER}\n{ron}")
    }

    #[test]
    fn hierarchy_and_inspector_values_round_trip() {
        let expected = default_records();
        let restored = deserialize_records(&serialize_records(&expected)).unwrap();
        assert_eq!(restored, expected);
    }

    #[test]
    fn runtime_components_and_handles_are_not_serialized() {
        let serialized = serialize_records(&default_records());
        assert!(!serialized.contains("Editable"));
        assert!(!serialized.contains("Mesh3d"));
        assert!(!serialized.contains("MeshMaterial3d"));
        assert!(serialized.contains("SceneAssetKind"));
    }

    #[test]
    fn damaged_old_and_dangling_scenes_return_errors() {
        assert!(deserialize_records("(not valid ron)").is_err());
        assert!(deserialize_records("// world_editor_scene_version: 0\n()").is_err());

        let mut dangling = default_records();
        dangling[1].parent = Some(999);
        assert!(deserialize_records(&serialize_records(&dangling)).is_err());
    }

    #[test]
    fn saved_file_restores_in_a_fresh_world() {
        use bevy::ecs::system::SystemState;

        let mut source = World::new();
        let parent = source
            .spawn((
                Editable,
                SceneId(1),
                SceneName("Parent".into()),
                SceneAssetKind::Cube,
                Transform::from_xyz(1.0, 2.0, 3.0),
            ))
            .id();
        source.spawn((
            Editable,
            SceneId(2),
            SceneName("Child".into()),
            SceneAssetKind::Sphere,
            Transform::from_xyz(4.0, 5.0, 6.0),
            ChildOf(parent),
        ));

        let mut state: SystemState<SaveQuery> = SystemState::new(&mut source);
        let editables = state.get(&source).unwrap();
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../../target/40a_scene_io_test_{}.ron",
            std::process::id()
        ));
        save_scene(&path, &editables).unwrap();
        let restored = load_records(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(restored.len(), 2);
        assert_eq!(restored[0].transform.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(restored[1].parent, Some(1));
        assert_eq!(restored[1].transform.translation, Vec3::new(4.0, 5.0, 6.0));
    }
}
