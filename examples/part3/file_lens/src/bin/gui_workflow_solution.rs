use std::{
    collections::HashSet,
    fs, io,
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
    time::SystemTime,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn responsive_orientation(window_width: f32) -> Orientation {
    if window_width < 720.0 {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UiIntent {
    ClearRequested,
    SaveRequested,
}

#[derive(Debug, Default)]
struct FileModel {
    files: Vec<FileEntry>,
    selected: Option<usize>,
}

impl FileModel {
    fn apply(&mut self, intent: UiIntent) -> bool {
        match intent {
            UiIntent::ClearRequested => {
                self.files.clear();
                self.selected = None;
                true
            }
            UiIntent::SaveRequested => !self.files.is_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FileEntry {
    path: PathBuf,
    size: u64,
    modified: Option<SystemTime>,
}

#[derive(Clone, Copy, Debug)]
struct DropPolicy {
    max_files: usize,
    max_size: u64,
}

fn filter_dropped(
    current: &[FileEntry],
    dropped: impl IntoIterator<Item = FileEntry>,
    policy: DropPolicy,
) -> Vec<FileEntry> {
    let mut known: HashSet<PathBuf> = current.iter().map(|entry| entry.path.clone()).collect();
    let mut accepted = Vec::new();

    for entry in dropped {
        if current.len() + accepted.len() >= policy.max_files {
            break;
        }
        if entry.size <= policy.max_size && known.insert(entry.path.clone()) {
            accepted.push(entry);
        }
    }

    accepted.sort_by(|left, right| {
        left.path
            .extension()
            .cmp(&right.path.extension())
            .then_with(|| left.size.cmp(&right.size))
            .then_with(|| left.path.file_name().cmp(&right.path.file_name()))
    });
    accepted
}

fn preview(bytes: &[u8], limit: usize) -> String {
    let bytes = &bytes[..bytes.len().min(limit)];
    match std::str::from_utf8(bytes) {
        Ok(text) => text.to_owned(),
        Err(_) => bytes
            .iter()
            .map(|byte| format!("{byte:02X}"))
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn read_in_background(path: PathBuf) -> Receiver<io::Result<Vec<u8>>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let _ = sender.send(fs::read(path));
    });
    receiver
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppMode {
    Empty,
    Reading,
    Ready,
    Error,
}

fn mode_after_read(result: &io::Result<Vec<u8>>) -> AppMode {
    match result {
        Ok(_) => AppMode::Ready,
        Err(_) => AppMode::Error,
    }
}

#[derive(Debug)]
struct FileEntity {
    id: u64,
    entry: FileEntry,
    selected: bool,
}

fn selected_entity(entities: &[FileEntity]) -> Option<u64> {
    entities
        .iter()
        .find(|entity| entity.selected)
        .map(|entity| entity.id)
}

fn main() {
    let mut model = FileModel {
        files: vec![FileEntry {
            path: PathBuf::from("README.md"),
            size: 128,
            modified: None,
        }],
        selected: Some(0),
    };
    let can_save = model.apply(UiIntent::SaveRequested);
    let layout = responsive_orientation(960.0);
    let sample = preview(b"Bevy File Lens", 1_024);
    let accepted = filter_dropped(
        &model.files,
        vec![FileEntry {
            path: PathBuf::from("notes.txt"),
            size: 64,
            modified: None,
        }],
        DropPolicy {
            max_files: 8,
            max_size: 1_024,
        },
    );
    let _background_reader: fn(PathBuf) -> Receiver<io::Result<Vec<u8>>> = read_in_background;
    let _initial_mode = AppMode::Empty;
    let _reading_mode = AppMode::Reading;
    let _mode_mapper: fn(&io::Result<Vec<u8>>) -> AppMode = mode_after_read;
    let _selected_lookup: fn(&[FileEntity]) -> Option<u64> = selected_entity;
    let _entry_size = FileEntity {
        id: 1,
        entry: model.files[0].clone(),
        selected: true,
    }
    .entry
    .size;
    let cleared = model.apply(UiIntent::ClearRequested);
    println!(
        "layout={layout:?}, can_save={can_save}, cleared={cleared}, accepted={}, preview={sample}",
        accepted.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(name: &str, size: u64) -> FileEntry {
        FileEntry {
            path: PathBuf::from(name),
            size,
            modified: None,
        }
    }

    #[test]
    fn layout_changes_at_small_width() {
        assert_eq!(responsive_orientation(960.0), Orientation::Horizontal);
        assert_eq!(responsive_orientation(600.0), Orientation::Vertical);
    }

    #[test]
    fn button_and_shortcut_share_the_same_intent() {
        let mut model = FileModel {
            files: vec![entry("a.txt", 10)],
            selected: Some(0),
        };
        assert!(model.apply(UiIntent::ClearRequested));
        assert!(model.files.is_empty());
        assert_eq!(model.selected, None);
        assert!(!model.apply(UiIntent::SaveRequested));
    }

    #[test]
    fn drop_policy_deduplicates_sorts_and_limits() {
        let current = vec![entry("same.txt", 10)];
        let accepted = filter_dropped(
            &current,
            vec![
                entry("same.txt", 10),
                entry("large.png", 2_000),
                entry("b.md", 40),
                entry("a.txt", 20),
                entry("c.md", 30),
            ],
            DropPolicy {
                max_files: 3,
                max_size: 1_000,
            },
        );
        assert_eq!(accepted, vec![entry("b.md", 40), entry("a.txt", 20)]);
    }

    #[test]
    fn binary_preview_is_hex_and_bounded() {
        assert_eq!(preview(&[0xFF, 0x00, 0xAB], 2), "FF 00");
        assert_eq!(preview(b"hello", 4), "hell");
    }

    #[test]
    fn entity_selection_returns_the_marked_id() {
        let entities = vec![
            FileEntity {
                id: 10,
                entry: entry("a", 1),
                selected: false,
            },
            FileEntity {
                id: 20,
                entry: entry("b", 1),
                selected: true,
            },
        ];
        assert_eq!(selected_entity(&entities), Some(20));
    }
}
