use bevy::{input_focus::InputFocus, prelude::*, window::WindowResolution};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Resource, Clone, Copy)]
pub struct LessonConfig {
    pub ui: bool,
    pub buttons: bool,
    pub drag_drop: bool,
    pub file_io: bool,
    pub state: bool,
}

impl LessonConfig {
    pub const APP: Self = Self {
        ui: false,
        buttons: false,
        drag_drop: false,
        file_io: false,
        state: false,
    };
    pub const UI: Self = Self {
        ui: true,
        ..Self::APP
    };
    pub const EVENTS: Self = Self {
        buttons: true,
        ..Self::UI
    };
    pub const DRAG_DROP: Self = Self {
        drag_drop: true,
        ..Self::EVENTS
    };
    pub const FILE_IO: Self = Self {
        file_io: true,
        ..Self::DRAG_DROP
    };
    pub const COMPLETE: Self = Self {
        state: true,
        ..Self::FILE_IO
    };
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum AppMode {
    #[default]
    Empty,
    Ready,
    Error,
}

#[derive(Resource)]
struct FileModel {
    files: Vec<FileEntry>,
    status: String,
}

impl Default for FileModel {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            status: "Ready".into(),
        }
    }
}

struct FileEntry {
    path: PathBuf,
    size: u64,
    preview: String,
}

#[derive(Component)]
enum ViewText {
    Status,
    FileList,
    Preview,
}

#[derive(Component)]
struct ModeText;

#[derive(Component)]
struct ClearButton;

#[derive(Component)]
struct SaveButton;

pub fn run(config: LessonConfig) {
    App::new()
        .insert_resource(config)
        .init_resource::<FileModel>()
        .init_resource::<InputFocus>()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.032, 0.05)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "File Lens - Bevy GUI Practice".into(),
                resolution: WindowResolution::new(1080, 700),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppMode>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_file_drop,
                handle_clear_button,
                handle_save_button,
                paint_buttons,
                update_view,
                update_mode,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, config: Res<LessonConfig>) {
    commands.spawn(Camera2d);
    if !config.ui {
        commands.spawn((
            Text::new("File Lens\nGUI chapter starts here"),
            TextFont {
                font_size: FontSize::Px(42.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            Node {
                position_type: PositionType::Absolute,
                top: percent(38),
                left: percent(25),
                width: percent(50),
                ..default()
            },
        ));
        return;
    }

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(20)),
            row_gap: px(14),
            ..default()
        })
        .with_children(|root| {
            root.spawn((
                Text::new("FILE LENS"),
                TextFont {
                    font_size: FontSize::Px(34.0),
                    ..default()
                },
                TextColor(Color::srgb(0.35, 0.9, 1.0)),
            ));
            root.spawn((
                ModeText,
                Text::new("MODE: EMPTY"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.7, 0.85)),
            ));
            root.spawn((
                Node {
                    width: percent(100),
                    height: px(0),
                    min_height: px(0),
                    flex_grow: 1.0,
                    column_gap: px(14),
                    ..default()
                },
                children![
                    (
                        Node {
                            width: percent(38),
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(px(18)),
                            row_gap: px(12),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.055, 0.07, 0.11)),
                        children![
                            (
                                Text::new("DROP FILES HERE"),
                                TextFont {
                                    font_size: FontSize::Px(24.0),
                                    ..default()
                                }
                            ),
                            (
                                ViewText::FileList,
                                Text::new("No files"),
                                TextFont {
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.72, 0.78, 0.9))
                            )
                        ]
                    ),
                    (
                        Node {
                            width: percent(62),
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(px(18)),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.04, 0.05, 0.075)),
                        children![
                            (
                                Text::new("PREVIEW"),
                                TextFont {
                                    font_size: FontSize::Px(22.0),
                                    ..default()
                                }
                            ),
                            (
                                ViewText::Preview,
                                Text::new("Drop a text file to inspect its contents."),
                                TextFont {
                                    font_size: FontSize::Px(17.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.75, 0.8, 0.86))
                            )
                        ]
                    )
                ],
            ));
            root.spawn((
                Node {
                    width: percent(100),
                    height: px(52),
                    column_gap: px(10),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    action_button::<ClearButton>("CLEAR"),
                    action_button::<SaveButton>("SAVE REPORT"),
                    (
                        ViewText::Status,
                        Text::new("Ready"),
                        TextFont {
                            font_size: FontSize::Px(17.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.72, 0.78))
                    )
                ],
            ));
        });
}

fn action_button<M: Component + Default>(label: &str) -> impl Bundle {
    (
        M::default(),
        Button,
        Node {
            width: px(150),
            height: px(42),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.11, 0.16, 0.24)),
        BorderColor::all(Color::srgb(0.25, 0.45, 0.58)),
        children![(
            Text::new(label),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            }
        )],
    )
}

impl Default for ClearButton {
    fn default() -> Self {
        Self
    }
}

impl Default for SaveButton {
    fn default() -> Self {
        Self
    }
}

fn handle_file_drop(
    mut dropped: MessageReader<FileDragAndDrop>,
    config: Res<LessonConfig>,
    mut model: ResMut<FileModel>,
    mut next_mode: ResMut<NextState<AppMode>>,
) {
    if !config.drag_drop {
        dropped.clear();
        return;
    }

    for event in dropped.read() {
        let FileDragAndDrop::DroppedFile { path_buf, .. } = event else {
            continue;
        };
        match inspect_file(path_buf, config.file_io) {
            Ok(entry) => {
                model.status = format!("Loaded {}", entry.path.display());
                model.files.push(entry);
                if config.state {
                    next_mode.set(AppMode::Ready);
                }
            }
            Err(error) => {
                model.status = error;
                if config.state {
                    next_mode.set(AppMode::Error);
                }
            }
        }
    }
}

fn inspect_file(path: &Path, read_contents: bool) -> Result<FileEntry, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("Cannot inspect {}: {error}", path.display()))?;
    if !metadata.is_file() {
        return Err(format!("{} is not a regular file", path.display()));
    }

    let preview = if read_contents {
        match fs::read(path) {
            Ok(bytes) => preview_bytes(&bytes, 600),
            Err(error) => format!("Preview unavailable: {error}"),
        }
    } else {
        "File I/O preview is enabled in chapter 25.".to_string()
    };
    Ok(FileEntry {
        path: path.to_path_buf(),
        size: metadata.len(),
        preview,
    })
}

fn preview_bytes(bytes: &[u8], limit: usize) -> String {
    let slice = &bytes[..bytes.len().min(limit)];
    match std::str::from_utf8(slice) {
        Ok(text) => text.to_owned(),
        Err(_) => format!(
            "Binary or non-UTF-8 file\n{} bytes shown as hexadecimal:\n{}",
            slice.len(),
            slice
                .iter()
                .take(64)
                .map(|byte| format!("{byte:02X}"))
                .collect::<Vec<_>>()
                .join(" ")
        ),
    }
}

fn handle_clear_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<ClearButton>)>,
    config: Res<LessonConfig>,
    mut model: ResMut<FileModel>,
    mut next_mode: ResMut<NextState<AppMode>>,
) {
    if config.buttons
        && interactions
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
    {
        model.files.clear();
        model.status = "Cleared".into();
        if config.state {
            next_mode.set(AppMode::Empty);
        }
    }
}

fn handle_save_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    config: Res<LessonConfig>,
    mut model: ResMut<FileModel>,
) {
    if !config.file_io
        || !interactions
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
    {
        return;
    }

    match write_report(&model.files) {
        Ok(path) => model.status = format!("Report saved to {}", path.display()),
        Err(error) => model.status = error,
    }
}

fn write_report(files: &[FileEntry]) -> Result<PathBuf, String> {
    let output_dir = PathBuf::from("output");
    fs::create_dir_all(&output_dir)
        .map_err(|error| format!("Cannot create output directory: {error}"))?;
    let path = output_dir.join("file_report.txt");
    let report = if files.is_empty() {
        "File Lens Report\nNo files loaded.\n".to_string()
    } else {
        let rows = files
            .iter()
            .map(|file| format!("{}\t{} bytes", file.path.display(), file.size))
            .collect::<Vec<_>>()
            .join("\n");
        format!("File Lens Report\n{rows}\n")
    };
    fs::write(&path, report).map_err(|error| format!("Cannot save report: {error}"))?;
    Ok(path)
}

fn paint_buttons(mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>) {
    for (interaction, mut color) in &mut buttons {
        color.0 = match interaction {
            Interaction::Pressed => Color::srgb(0.15, 0.55, 0.7),
            Interaction::Hovered => Color::srgb(0.14, 0.28, 0.4),
            Interaction::None => Color::srgb(0.11, 0.16, 0.24),
        };
    }
}

fn update_view(model: Res<FileModel>, mut texts: Query<(&mut Text, &ViewText)>) {
    if !model.is_changed() {
        return;
    }
    for (mut text, kind) in &mut texts {
        match kind {
            ViewText::FileList => {
                text.0 = if model.files.is_empty() {
                    "No files".into()
                } else {
                    model
                        .files
                        .iter()
                        .enumerate()
                        .map(|(index, file)| {
                            let name = file
                                .path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or("<unknown>");
                            format!("{}. {name} ({} bytes)", index + 1, file.size)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                };
            }
            ViewText::Preview => {
                text.0 = model.files.last().map_or_else(
                    || "Drop a file to inspect it.".into(),
                    |file| file.preview.clone(),
                );
            }
            ViewText::Status => text.0.clone_from(&model.status),
        }
    }
}

fn update_mode(
    mode: Res<State<AppMode>>,
    config: Res<LessonConfig>,
    text: Option<Single<&mut Text, With<ModeText>>>,
) {
    if !config.state || !mode.is_changed() {
        return;
    }
    if let Some(mut text) = text {
        text.0 = format!("MODE: {:?}", mode.get()).to_uppercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_preview_is_limited() {
        assert_eq!(preview_bytes(b"abcdef", 3), "abc");
    }

    #[test]
    fn binary_preview_uses_hex() {
        let result = preview_bytes(&[0xFF, 0x00], 10);
        assert!(result.contains("FF 00"));
    }
}
