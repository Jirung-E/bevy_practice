use bevy::{
    prelude::*,
    tasks::{futures::check_ready, IoTaskPool, Task},
    window::WindowResolution,
};
use std::{fs::File, io::Read, path::PathBuf};

const PREVIEW_LIMIT: u64 = 64 * 1024;

#[derive(Component)]
struct ReadFileTask(Task<Result<FilePreview, String>>);

#[derive(Component)]
struct StatusText;

struct FilePreview {
    path: PathBuf,
    bytes: Vec<u8>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Background File I/O - Bevy Practice".into(),
                resolution: WindowResolution::new(900, 560),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (start_read_on_drop, finish_reads))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        StatusText,
        Text::new("파일을 창으로 드롭하세요.\n읽는 동안 창은 계속 반응합니다."),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextLayout::justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            left: px(28),
            right: px(28),
            top: px(28),
            ..default()
        },
    ));
}

fn start_read_on_drop(
    mut commands: Commands,
    mut drops: MessageReader<FileDragAndDrop>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    for event in drops.read() {
        let FileDragAndDrop::DroppedFile { path_buf, .. } = event else {
            continue;
        };
        let path = path_buf.clone();
        status.0 = format!("LOADING: {}", path.display());
        let task = IoTaskPool::get().spawn(async move { read_preview(path) });
        commands.spawn(ReadFileTask(task));
    }
}

fn read_preview(path: PathBuf) -> Result<FilePreview, String> {
    let mut file = File::open(&path)
        .map_err(|error| format!("{} 열기 실패: {error}", path.display()))?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(PREVIEW_LIMIT)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("{} 읽기 실패: {error}", path.display()))?;
    Ok(FilePreview { path, bytes })
}

fn finish_reads(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ReadFileTask)>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    for (entity, mut task) in &mut tasks {
        let Some(result) = check_ready(&mut task.0) else {
            continue;
        };
        status.0 = match result {
            Ok(preview) => format!(
                "READY: {}\n\n{}",
                preview.path.display(),
                preview_text(&preview.bytes)
            ),
            Err(error) => format!("ERROR: {error}"),
        };
        commands.entity(entity).despawn();
    }
}

fn preview_text(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(text) => text.chars().take(900).collect(),
        Err(_) => format!(
            "바이너리 데이터 (앞 64바이트)\n{}",
            bytes
                .iter()
                .take(64)
                .map(|byte| format!("{byte:02X}"))
                .collect::<Vec<_>>()
                .join(" ")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_preview_uses_hex() {
        assert!(preview_text(&[0xff, 0x00]).contains("FF 00"));
    }
}
