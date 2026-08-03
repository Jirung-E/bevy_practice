use bevy::{
    camera::Viewport,
    input::mouse::{MouseMotion, MouseWheel},
    picking::pointer::PointerButton,
    prelude::*,
    window::WindowResolution,
};

#[derive(Resource, Clone, Copy)]
pub struct LessonConfig {
    pub inspector: bool,
    pub viewport: bool,
    pub assets: bool,
    pub console: bool,
}

impl LessonConfig {
    pub const HIERARCHY: Self = Self {
        inspector: false,
        viewport: false,
        assets: false,
        console: false,
    };
    pub const INSPECTOR: Self = Self {
        inspector: true,
        ..Self::HIERARCHY
    };
    pub const VIEWPORT: Self = Self {
        viewport: true,
        ..Self::INSPECTOR
    };
    pub const ASSETS: Self = Self {
        assets: true,
        ..Self::VIEWPORT
    };
    pub const COMPLETE: Self = Self {
        console: true,
        ..Self::ASSETS
    };
}

#[derive(Component)]
struct Editable;

#[derive(Component)]
struct SelectionBackground;

#[derive(Component)]
struct EditorName(String);

#[derive(Component)]
struct HierarchyText;

#[derive(Component)]
struct InspectorText;

#[derive(Component)]
struct ConsoleText;

#[derive(Component)]
struct EditorCamera;

#[derive(Component)]
struct ViewportInfoText;

const HIERARCHY_WIDTH: f32 = 250.0;
const INSPECTOR_WIDTH: f32 = 285.0;
const CONSOLE_HEIGHT: f32 = 158.0;

#[derive(Component, Clone, Copy)]
enum EditorAction {
    SelectNext,
    Delete,
    MoveX(f32),
    MoveY(f32),
    MoveZ(f32),
    CreateCube,
    CreateSphere,
    ClearConsole,
}

#[derive(Resource, Default)]
struct Selection(Option<Entity>);

#[derive(Resource)]
struct EditorLog {
    lines: Vec<String>,
}

impl Default for EditorLog {
    fn default() -> Self {
        Self {
            lines: vec!["Editor started".into()],
        }
    }
}

#[derive(Resource)]
struct Orbit {
    yaw: f32,
    pitch: f32,
    radius: f32,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            yaw: -0.55,
            pitch: -0.38,
            radius: 11.0,
        }
    }
}

#[derive(Resource)]
struct EditorAssets {
    cube: Handle<Mesh>,
    sphere: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
    sphere_material: Handle<StandardMaterial>,
}

pub fn run(config: LessonConfig) {
    App::new()
        .insert_resource(config)
        .init_resource::<Selection>()
        .init_resource::<EditorLog>()
        .init_resource::<Orbit>()
        .insert_resource(ClearColor(Color::srgb(0.018, 0.022, 0.032)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "World Editor - Bevy Practice".into(),
                    resolution: WindowResolution::new(1280, 800),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
        ))
        .add_observer(select_from_viewport)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_editor_actions,
                update_hierarchy,
                update_inspector,
                update_console,
                update_editor_viewport,
                orbit_viewport,
                show_viewport_cursor,
                draw_selection_gizmo,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    config: Res<LessonConfig>,
    window: Single<&Window>,
    mut selection: ResMut<Selection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.5, 0.0)),
    ));
    commands.spawn((
        SelectionBackground,
        Pickable::default(),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.12, 0.14))),
    ));

    let editor_assets = EditorAssets {
        cube: meshes.add(Cuboid::from_length(1.5)),
        sphere: meshes.add(Sphere::new(0.9)),
        cube_material: materials.add(Color::srgb(0.12, 0.52, 0.9)),
        sphere_material: materials.add(Color::srgb(0.9, 0.3, 0.12)),
    };
    let first = spawn_editable(
        &mut commands,
        "Blue Cube",
        editor_assets.cube.clone(),
        editor_assets.cube_material.clone(),
        Vec3::new(-2.0, 0.75, 0.0),
    );
    spawn_editable(
        &mut commands,
        "Orange Sphere",
        editor_assets.sphere.clone(),
        editor_assets.sphere_material.clone(),
        Vec3::new(2.0, 0.9, -1.0),
    );
    selection.0 = Some(first);
    commands.insert_resource(editor_assets);

    let ui_camera = if config.viewport {
        commands.spawn((
            EditorCamera,
            Camera3d::default(),
            Camera {
                viewport: editor_viewport(&window, &config),
                ..default()
            },
            AmbientLight {
                color: Color::srgb(0.32, 0.38, 0.5),
                brightness: 190.0,
                ..default()
            },
            Transform::from_xyz(6.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
        commands
            .spawn((
                Camera2d,
                Camera {
                    order: 1,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
            ))
            .id()
    } else {
        commands.spawn(Camera2d).id()
    };

    setup_editor_ui(&mut commands, &config, ui_camera);
}

fn spawn_editable(
    commands: &mut Commands,
    name: &str,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) -> Entity {
    commands
        .spawn((
            Editable,
            Pickable::default(),
            EditorName(name.into()),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position),
        ))
        .id()
}

fn setup_editor_ui(commands: &mut Commands, config: &LessonConfig, ui_camera: Entity) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
            Pickable::IGNORE,
            UiTargetCamera(ui_camera),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: px(250),
                    height: percent(100),
                    padding: UiRect::all(px(14)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(10),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.035, 0.045, 0.065, 0.96)),
                children![
                    panel_title("HIERARCHY"),
                    (
                        HierarchyText,
                        Text::new(""),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        }
                    ),
                    editor_button("SELECT NEXT", EditorAction::SelectNext),
                    editor_button("DELETE", EditorAction::Delete)
                ],
            ));

            if config.inspector {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: px(0),
                        top: px(0),
                        width: px(285),
                        height: percent(100),
                        padding: UiRect::all(px(14)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.035, 0.045, 0.065, 0.96)),
                    children![
                        panel_title("INSPECTOR"),
                        (
                            InspectorText,
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(17.0),
                                ..default()
                            }
                        ),
                        editor_button("X -", EditorAction::MoveX(-0.25)),
                        editor_button("X +", EditorAction::MoveX(0.25)),
                        editor_button("Y -", EditorAction::MoveY(-0.25)),
                        editor_button("Y +", EditorAction::MoveY(0.25)),
                        editor_button("Z -", EditorAction::MoveZ(-0.25)),
                        editor_button("Z +", EditorAction::MoveZ(0.25))
                    ],
                ));
            }

            if config.assets {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(264),
                        bottom: if config.console { px(172) } else { px(14) },
                        width: px(360),
                        height: px(105),
                        padding: UiRect::all(px(12)),
                        flex_direction: FlexDirection::Row,
                        column_gap: px(10),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.035, 0.045, 0.065, 0.94)),
                    children![
                        panel_title("ASSETS"),
                        editor_button("CUBE", EditorAction::CreateCube),
                        editor_button("SPHERE", EditorAction::CreateSphere)
                    ],
                ));
            }

            if config.viewport {
                root.spawn((
                    ViewportInfoText,
                    Text::new("VIEWPORT: cursor outside"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.55, 0.85, 0.95)),
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(HIERARCHY_WIDTH + 14.0),
                        top: px(12),
                        ..default()
                    },
                ));
            }

            if config.console {
                root.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(250),
                        right: px(285),
                        bottom: px(0),
                        height: px(158),
                        padding: UiRect::all(px(12)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(5),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.02, 0.025, 0.035, 0.97)),
                    children![
                        panel_title("CONSOLE"),
                        (
                            ConsoleText,
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.65, 0.88, 0.68))
                        ),
                        editor_button("CLEAR LOG", EditorAction::ClearConsole)
                    ],
                ));
            }
        });
}

fn select_from_viewport(
    click: On<Pointer<Click>>,
    mut selection: ResMut<Selection>,
    mut log: ResMut<EditorLog>,
    editables: Query<&EditorName, With<Editable>>,
    backgrounds: Query<(), With<SelectionBackground>>,
) {
    if click.button != PointerButton::Primary {
        return;
    }
    if let Ok(name) = editables.get(click.entity) {
        selection.0 = Some(click.entity);
        log.lines.push(format!("Selected {} from viewport", name.0));
    } else if backgrounds.contains(click.entity) {
        selection.0 = None;
        log.lines.push("Cleared viewport selection".into());
    }
    if log.lines.len() > 5 {
        log.lines.remove(0);
    }
}

fn panel_title(label: &str) -> impl Bundle {
    (
        Text::new(label),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.3, 0.85, 1.0)),
    )
}

fn editor_button(label: &str, action: EditorAction) -> impl Bundle {
    (
        Button,
        action,
        Node {
            min_width: px(82),
            height: px(34),
            padding: UiRect::horizontal(px(10)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.11, 0.16, 0.24)),
        children![(
            Text::new(label),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            }
        )],
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "에디터 명령이 선택, 에셋, 로그, 월드 변경을 조정하는 단일 경계 시스템이다"
)]
fn handle_editor_actions(
    mut commands: Commands,
    mut buttons: Query<(&Interaction, &EditorAction, &mut BackgroundColor), Changed<Interaction>>,
    config: Res<LessonConfig>,
    assets: Res<EditorAssets>,
    mut selection: ResMut<Selection>,
    mut log: ResMut<EditorLog>,
    editables: Query<(Entity, &EditorName), With<Editable>>,
    mut transforms: Query<&mut Transform, With<Editable>>,
) {
    for (interaction, action, mut color) in &mut buttons {
        color.0 = match interaction {
            Interaction::Pressed => Color::srgb(0.12, 0.5, 0.68),
            Interaction::Hovered => Color::srgb(0.14, 0.28, 0.4),
            Interaction::None => Color::srgb(0.11, 0.16, 0.24),
        };
        if *interaction != Interaction::Pressed {
            continue;
        }

        match *action {
            EditorAction::SelectNext => {
                let entities = editables
                    .iter()
                    .map(|(entity, _)| entity)
                    .collect::<Vec<_>>();
                if !entities.is_empty() {
                    let current = selection
                        .0
                        .and_then(|selected| entities.iter().position(|entity| *entity == selected))
                        .unwrap_or(entities.len() - 1);
                    selection.0 = Some(entities[(current + 1) % entities.len()]);
                    log.lines.push("Selection changed".into());
                }
            }
            EditorAction::Delete => {
                if let Some(entity) = selection.0.take() {
                    commands.entity(entity).despawn();
                    log.lines.push(format!("Deleted {entity:?}"));
                }
            }
            EditorAction::MoveX(amount) if config.inspector => {
                move_selected(selection.0, &mut transforms, Vec3::X * amount, &mut log);
            }
            EditorAction::MoveY(amount) if config.inspector => {
                move_selected(selection.0, &mut transforms, Vec3::Y * amount, &mut log);
            }
            EditorAction::MoveZ(amount) if config.inspector => {
                move_selected(selection.0, &mut transforms, Vec3::Z * amount, &mut log);
            }
            EditorAction::CreateCube if config.assets => {
                let entity = spawn_editable(
                    &mut commands,
                    "New Cube",
                    assets.cube.clone(),
                    assets.cube_material.clone(),
                    Vec3::new(0.0, 0.75, 0.0),
                );
                selection.0 = Some(entity);
                log.lines.push(format!("Created cube {entity:?}"));
            }
            EditorAction::CreateSphere if config.assets => {
                let entity = spawn_editable(
                    &mut commands,
                    "New Sphere",
                    assets.sphere.clone(),
                    assets.sphere_material.clone(),
                    Vec3::new(0.0, 0.9, 0.0),
                );
                selection.0 = Some(entity);
                log.lines.push(format!("Created sphere {entity:?}"));
            }
            EditorAction::ClearConsole if config.console => log.lines.clear(),
            _ => {}
        }
    }
}

fn move_selected(
    selected: Option<Entity>,
    transforms: &mut Query<&mut Transform, With<Editable>>,
    delta: Vec3,
    log: &mut EditorLog,
) {
    let Some(entity) = selected else {
        return;
    };
    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.translation += delta;
        log.lines.push(format!("Moved {entity:?} by {delta:.2}"));
    }
}

fn update_hierarchy(
    selection: Res<Selection>,
    editables: Query<(Entity, &EditorName), With<Editable>>,
    mut text: Single<&mut Text, With<HierarchyText>>,
) {
    let mut rows = editables
        .iter()
        .map(|(entity, name)| {
            let marker = if selection.0 == Some(entity) {
                ">"
            } else {
                " "
            };
            format!("{marker} {}  [{entity:?}]", name.0)
        })
        .collect::<Vec<_>>();
    rows.sort();
    text.0 = if rows.is_empty() {
        "(empty world)".into()
    } else {
        rows.join("\n")
    };
}

fn update_inspector(
    selection: Res<Selection>,
    selected: Query<(&EditorName, &Transform), With<Editable>>,
    text: Option<Single<&mut Text, With<InspectorText>>>,
) {
    let Some(mut text) = text else {
        return;
    };
    text.0 = selection
        .0
        .and_then(|entity| selected.get(entity).ok())
        .map_or_else(
            || "Nothing selected".into(),
            |(name, transform)| {
                format!(
                    "{}\n\nTransform\n  X: {:.2}\n  Y: {:.2}\n  Z: {:.2}",
                    name.0,
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z
                )
            },
        );
}

fn update_console(log: Res<EditorLog>, text: Option<Single<&mut Text, With<ConsoleText>>>) {
    if !log.is_changed() {
        return;
    }
    if let Some(mut text) = text {
        let start = log.lines.len().saturating_sub(5);
        text.0 = log.lines[start..].join("\n");
    }
}

fn orbit_viewport(
    config: Res<LessonConfig>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut orbit: ResMut<Orbit>,
    window: Single<&Window>,
    camera: Option<Single<&mut Transform, With<EditorCamera>>>,
) {
    if !config.viewport {
        motion.clear();
        wheel.clear();
        return;
    }
    let delta = motion.read().map(|event| event.delta).sum::<Vec2>();
    let cursor_in_viewport = window
        .cursor_position()
        .is_some_and(|cursor| cursor_in_editor_viewport(cursor, &window, &config));
    if cursor_in_viewport && buttons.pressed(MouseButton::Right) {
        orbit.yaw -= delta.x * 0.005;
        orbit.pitch = (orbit.pitch - delta.y * 0.005).clamp(-1.3, 0.1);
    }
    for event in wheel.read() {
        if cursor_in_viewport {
            orbit.radius = (orbit.radius - event.y * 0.6).clamp(4.0, 20.0);
        }
    }
    if let Some(mut camera) = camera {
        let rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
        let target = Vec3::new(0.0, 1.0, 0.0);
        **camera =
            Transform::from_translation(target + rotation * Vec3::new(0.0, 0.0, orbit.radius))
                .looking_at(target, Vec3::Y);
    }
}

fn editor_viewport(window: &Window, config: &LessonConfig) -> Option<Viewport> {
    if !config.viewport {
        return None;
    }
    let scale = window.scale_factor();
    let left = (HIERARCHY_WIDTH * scale).round() as u32;
    let right = if config.inspector {
        (INSPECTOR_WIDTH * scale).round() as u32
    } else {
        0
    };
    let bottom = if config.console {
        (CONSOLE_HEIGHT * scale).round() as u32
    } else {
        0
    };
    Some(Viewport {
        physical_position: UVec2::new(left, 0),
        physical_size: UVec2::new(
            window.physical_width().saturating_sub(left + right).max(1),
            window.physical_height().saturating_sub(bottom).max(1),
        ),
        ..default()
    })
}

fn update_editor_viewport(
    config: Res<LessonConfig>,
    window: Single<&Window>,
    camera: Option<Single<&mut Camera, With<EditorCamera>>>,
) {
    let Some(mut camera) = camera else { return };
    let expected = editor_viewport(&window, &config);
    let changed = camera.viewport.as_ref().map(|viewport| {
        (viewport.physical_position, viewport.physical_size)
    }) != expected.as_ref().map(|viewport| {
        (viewport.physical_position, viewport.physical_size)
    });
    if changed {
        camera.viewport = expected;
    }
}

fn cursor_in_editor_viewport(cursor: Vec2, window: &Window, config: &LessonConfig) -> bool {
    let right = if config.inspector { INSPECTOR_WIDTH } else { 0.0 };
    let bottom = if config.console { CONSOLE_HEIGHT } else { 0.0 };
    cursor.x >= HIERARCHY_WIDTH
        && cursor.x < window.width() - right
        && cursor.y >= 0.0
        && cursor.y < window.height() - bottom
}

fn show_viewport_cursor(
    config: Res<LessonConfig>,
    window: Single<&Window>,
    camera: Option<Single<(&Camera, &GlobalTransform), With<EditorCamera>>>,
    text: Option<Single<&mut Text, With<ViewportInfoText>>>,
    mut gizmos: Gizmos,
) {
    let (Some(camera), Some(mut text), Some(cursor)) =
        (camera, text, window.cursor_position())
    else {
        return;
    };
    if !cursor_in_editor_viewport(cursor, &window, &config) {
        text.0 = "VIEWPORT: cursor outside".into();
        return;
    }
    let (camera, transform) = *camera;
    let Ok(ray) = camera.viewport_to_world(transform, cursor) else {
        return;
    };
    let Some(point) = ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
    else {
        return;
    };
    text.0 = format!(
        "LOGICAL ({:.0}, {:.0})  WORLD ({:.2}, {:.2}, {:.2})  DPI {:.2}",
        cursor.x,
        cursor.y,
        point.x,
        point.y,
        point.z,
        window.scale_factor()
    );
    gizmos.circle(
        Isometry3d::new(point + Vec3::Y * 0.02, Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        0.18,
        Color::srgb(0.2, 0.9, 1.0),
    );
}

fn draw_selection_gizmo(
    config: Res<LessonConfig>,
    selection: Res<Selection>,
    selected: Query<&Transform, With<Editable>>,
    mut gizmos: Gizmos,
) {
    if !config.viewport {
        return;
    }
    let Some(transform) = selection.0.and_then(|entity| selected.get(entity).ok()) else {
        return;
    };
    gizmos.axes(*transform, 1.4);
    gizmos.cube(
        Transform::from_translation(transform.translation).with_scale(Vec3::splat(1.9)),
        Color::srgb(1.0, 0.85, 0.1),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_window_keeps_at_most_five_recent_lines() {
        let lines = (0..8).map(|value| value.to_string()).collect::<Vec<_>>();
        let start = lines.len().saturating_sub(5);
        assert_eq!(&lines[start..], ["3", "4", "5", "6", "7"]);
    }
}
