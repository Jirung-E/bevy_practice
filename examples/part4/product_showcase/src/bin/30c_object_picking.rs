use bevy::{
    asset::AssetPlugin,
    picking::{mesh_picking::MeshPickingSettings, pointer::PointerButton},
    prelude::*,
    window::WindowResolution,
};

#[derive(Component)]
struct SelectableObject {
    name: &'static str,
}

#[derive(Component)]
struct SelectionBackground;

#[derive(Component)]
struct SelectionStatus;

#[derive(Component, Clone)]
struct PickingMaterials {
    normal: Handle<StandardMaterial>,
    hover: Handle<StandardMaterial>,
    selected: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct Selection(Option<Entity>);

fn main() {
    App::new()
        .init_resource::<Selection>()
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.03)))
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "3D Object Picking - Bevy Practice".into(),
                        resolution: WindowResolution::new(1100, 720),
                        ..default()
                    }),
                    ..default()
                }),
            MeshPickingPlugin,
        ))
        .add_observer(handle_click)
        .add_observer(handle_over)
        .add_observer(handle_out)
        .add_systems(Startup, setup)
        .add_systems(Update, update_status)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 13_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.6, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.5, 6.5, 11.0).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));

    let hover = materials.add(Color::srgb(0.15, 0.9, 0.9));
    let selected = materials.add(Color::srgb(1.0, 0.78, 0.08));
    spawn_object(
        &mut commands,
        "Blue Cube",
        meshes.add(Cuboid::from_length(2.0)),
        materials.add(Color::srgb(0.1, 0.38, 0.9)),
        hover.clone(),
        selected.clone(),
        Vec3::new(-2.3, 1.0, 0.0),
    );
    spawn_object(
        &mut commands,
        "Orange Sphere",
        meshes.add(Sphere::new(1.2)),
        materials.add(Color::srgb(0.95, 0.28, 0.08)),
        hover,
        selected,
        Vec3::new(2.1, 1.2, -0.6),
    );

    commands.spawn((
        SelectionBackground,
        Pickable::default(),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.12, 0.14, 0.18))),
    ));

    commands.spawn((
        Text::new("3D OBJECT PICKING\nLMB: SELECT   GROUND: CLEAR"),
        TextFont::from_font_size(25.0),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            top: px(18),
            ..default()
        },
        Pickable::IGNORE,
    ));
    commands.spawn((
        SelectionStatus,
        Text::new("SELECTED: NONE"),
        TextFont::from_font_size(23.0),
        TextColor(Color::srgb(1.0, 0.82, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            left: px(20),
            bottom: px(18),
            ..default()
        },
        Pickable::IGNORE,
    ));
}

#[allow(clippy::too_many_arguments)]
fn spawn_object(
    commands: &mut Commands,
    name: &'static str,
    mesh: Handle<Mesh>,
    normal: Handle<StandardMaterial>,
    hover: Handle<StandardMaterial>,
    selected: Handle<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        SelectableObject { name },
        Pickable::default(),
        PickingMaterials {
            normal: normal.clone(),
            hover,
            selected,
        },
        Mesh3d(mesh),
        MeshMaterial3d(normal),
        Transform::from_translation(position),
    ));
}

fn handle_click(
    click: On<Pointer<Click>>,
    mut selection: ResMut<Selection>,
    objects: Query<(), With<SelectableObject>>,
    backgrounds: Query<(), With<SelectionBackground>>,
    mut visuals: Query<(&PickingMaterials, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    let next = selection_after_click(
        selection.0,
        objects.contains(click.entity).then_some(click.entity),
        backgrounds.contains(click.entity),
    );
    if next == selection.0 {
        return;
    }
    if let Some(previous) = selection.0
        && let Ok((materials, mut material)) = visuals.get_mut(previous)
    {
        material.0 = materials.normal.clone();
    }
    selection.0 = next;
    if let Some(entity) = next
        && let Ok((materials, mut material)) = visuals.get_mut(entity)
    {
        material.0 = materials.selected.clone();
    }
}

fn handle_over(
    over: On<Pointer<Over>>,
    selection: Res<Selection>,
    mut objects: Query<(&PickingMaterials, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if selection.0 == Some(over.entity) {
        return;
    }
    if let Ok((materials, mut material)) = objects.get_mut(over.entity) {
        material.0 = materials.hover.clone();
    }
}

fn handle_out(
    out: On<Pointer<Out>>,
    selection: Res<Selection>,
    mut objects: Query<(&PickingMaterials, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if selection.0 == Some(out.entity) {
        return;
    }
    if let Ok((materials, mut material)) = objects.get_mut(out.entity) {
        material.0 = materials.normal.clone();
    }
}

fn update_status(
    selection: Res<Selection>,
    names: Query<&SelectableObject>,
    mut status: Single<&mut Text, With<SelectionStatus>>,
) {
    if !selection.is_changed() {
        return;
    }
    let name = selection
        .0
        .and_then(|entity| names.get(entity).ok())
        .map_or("NONE", |object| object.name);
    status.0 = format!("SELECTED: {name}");
}

fn selection_after_click(
    current: Option<Entity>,
    object_hit: Option<Entity>,
    background_hit: bool,
) -> Option<Entity> {
    object_hit.or(if background_hit { None } else { current })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_click_selects_and_background_clears() {
        let first = Entity::from_raw_u32(1).unwrap();
        let second = Entity::from_raw_u32(2).unwrap();
        assert_eq!(selection_after_click(None, Some(first), false), Some(first));
        assert_eq!(
            selection_after_click(Some(first), Some(second), false),
            Some(second)
        );
        assert_eq!(selection_after_click(Some(second), None, true), None);
    }

    #[test]
    fn unrelated_pointer_event_keeps_selection() {
        let selected = Entity::from_raw_u32(7).unwrap();
        assert_eq!(
            selection_after_click(Some(selected), None, false),
            Some(selected)
        );
    }
}
