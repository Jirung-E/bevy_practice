use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
};

#[derive(Clone, Debug, PartialEq)]
struct EditorEntity {
    id: u64,
    name: String,
    parent: Option<u64>,
    translation: [f32; 3],
    scale: [f32; 3],
    rotation_y_degrees: f32,
}

fn hierarchy_lines(entities: &[EditorEntity]) -> Vec<String> {
    fn visit(
        id: u64,
        depth: usize,
        by_id: &HashMap<u64, &EditorEntity>,
        children: &HashMap<Option<u64>, Vec<u64>>,
        visited: &mut HashSet<u64>,
        output: &mut Vec<String>,
    ) {
        if !visited.insert(id) {
            return;
        }
        let entity = by_id[&id];
        output.push(format!("{}{}", "  ".repeat(depth), entity.name));
        if let Some(child_ids) = children.get(&Some(id)) {
            for child in child_ids {
                visit(*child, depth + 1, by_id, children, visited, output);
            }
        }
    }

    let by_id: HashMap<_, _> = entities.iter().map(|entity| (entity.id, entity)).collect();
    let mut children: HashMap<Option<u64>, Vec<u64>> = HashMap::new();
    for entity in entities {
        children.entry(entity.parent).or_default().push(entity.id);
    }
    for ids in children.values_mut() {
        ids.sort_by_key(|id| &by_id[id].name);
    }

    let mut output = Vec::new();
    let mut visited = HashSet::new();
    if let Some(roots) = children.get(&None) {
        for root in roots {
            visit(*root, 0, &by_id, &children, &mut visited, &mut output);
        }
    }
    output
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EditorAction {
    SetScale { entity: u64, scale: [f32; 3] },
    RotateY { entity: u64, degrees: f32 },
    MoveX { entity: u64, amount: f32 },
    SpawnCube,
    SelectNext,
}

fn apply_action(entities: &mut [EditorEntity], action: EditorAction) -> bool {
    match action {
        EditorAction::SetScale { entity, scale } => {
            let Some(target) = entities.iter_mut().find(|item| item.id == entity) else {
                return false;
            };
            target.scale = scale;
            true
        }
        EditorAction::RotateY { entity, degrees } => {
            let Some(target) = entities.iter_mut().find(|item| item.id == entity) else {
                return false;
            };
            target.rotation_y_degrees += degrees;
            true
        }
        EditorAction::MoveX { entity, amount } => {
            let Some(target) = entities.iter_mut().find(|item| item.id == entity) else {
                return false;
            };
            target.translation[0] += amount;
            true
        }
        EditorAction::SpawnCube | EditorAction::SelectNext => true,
    }
}

fn physical_viewport(logical: [f32; 4], scale_factor: f32) -> [u32; 4] {
    logical.map(|value| (value * scale_factor).round().max(0.0) as u32)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssetKind {
    Scene,
    Image,
    Audio,
    Unknown,
}

fn classify_asset(path: &Path) -> AssetKind {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("gltf" | "glb") => AssetKind::Scene,
        Some("png" | "jpg" | "jpeg") => AssetKind::Image,
        Some("ogg" | "wav") => AssetKind::Audio,
        _ => AssetKind::Unknown,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
struct EditorLog {
    elapsed_seconds: f32,
    level: LogLevel,
    message: String,
}

#[derive(Debug)]
struct Console {
    capacity: usize,
    entries: VecDeque<EditorLog>,
}

impl Console {
    fn push(&mut self, entry: EditorLog) {
        if self.entries.len() == self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }
}

fn parse_command(source: &str, selected: Option<u64>) -> Result<EditorAction, String> {
    let words: Vec<_> = source.split_whitespace().collect();
    match words.as_slice() {
        ["spawn", "cube"] => Ok(EditorAction::SpawnCube),
        ["select", "next"] => Ok(EditorAction::SelectNext),
        ["move", "x", amount] => Ok(EditorAction::MoveX {
            entity: selected.ok_or("no selected entity")?,
            amount: amount.parse().map_err(|_| "invalid amount")?,
        }),
        _ => Err("unknown command".to_owned()),
    }
}

fn sample_entity(id: u64, name: &str, parent: Option<u64>) -> EditorEntity {
    EditorEntity {
        id,
        name: name.to_owned(),
        parent,
        translation: [0.0; 3],
        scale: [1.0; 3],
        rotation_y_degrees: 0.0,
    }
}

fn main() {
    let mut entities = vec![
        sample_entity(1, "Root", None),
        sample_entity(2, "Cube", Some(1)),
    ];
    let lines = hierarchy_lines(&entities);
    let action = parse_command("move x 1.0", Some(2)).expect("valid command");
    apply_action(&mut entities, action);
    apply_action(
        &mut entities,
        EditorAction::SetScale {
            entity: 2,
            scale: [2.0; 3],
        },
    );
    apply_action(
        &mut entities,
        EditorAction::RotateY {
            entity: 2,
            degrees: 15.0,
        },
    );
    let viewport = physical_viewport([250.0, 32.0, 746.0, 640.0], 1.5);
    let kind = classify_asset(Path::new("assets/robot.glb"));
    let mut console = Console {
        capacity: 2,
        entries: VecDeque::new(),
    };
    console.push(EditorLog {
        elapsed_seconds: 0.0,
        level: LogLevel::Info,
        message: "Editor started".to_owned(),
    });
    console.push(EditorLog {
        elapsed_seconds: 0.1,
        level: LogLevel::Warning,
        message: "Example warning".to_owned(),
    });
    console.push(EditorLog {
        elapsed_seconds: 0.2,
        level: LogLevel::Error,
        message: "Example error".to_owned(),
    });
    let newest = console.entries.back().expect("console has an entry");
    println!(
        "tree={lines:?}, viewport={viewport:?}, asset={kind:?}, logs={}, newest=({:.1}, {:?}, {})",
        console.entries.len(),
        newest.elapsed_seconds,
        newest.level,
        newest.message
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchy_is_recursive_and_indented() {
        let entities = vec![
            sample_entity(1, "Root", None),
            sample_entity(2, "Child", Some(1)),
            sample_entity(3, "Grandchild", Some(2)),
        ];
        assert_eq!(
            hierarchy_lines(&entities),
            ["Root", "  Child", "    Grandchild"]
        );
    }

    #[test]
    fn inspector_actions_modify_only_the_target() {
        let mut entities = vec![sample_entity(1, "A", None), sample_entity(2, "B", None)];
        assert!(apply_action(
            &mut entities,
            EditorAction::RotateY {
                entity: 2,
                degrees: 15.0
            }
        ));
        assert_eq!(entities[0].rotation_y_degrees, 0.0);
        assert_eq!(entities[1].rotation_y_degrees, 15.0);
    }

    #[test]
    fn viewport_uses_physical_pixels() {
        assert_eq!(
            physical_viewport([250.0, 32.0, 746.0, 640.0], 1.5),
            [375, 48, 1119, 960]
        );
    }

    #[test]
    fn console_discards_oldest_entry() {
        let mut console = Console {
            capacity: 2,
            entries: VecDeque::new(),
        };
        for index in 0..3 {
            console.push(EditorLog {
                elapsed_seconds: index as f32,
                level: LogLevel::Info,
                message: index.to_string(),
            });
        }
        assert_eq!(console.entries.len(), 2);
        assert_eq!(console.entries[0].message, "1");
    }

    #[test]
    fn console_command_reuses_editor_action() {
        assert_eq!(
            parse_command("move x 1.5", Some(42)).unwrap(),
            EditorAction::MoveX {
                entity: 42,
                amount: 1.5
            }
        );
        assert!(parse_command("move x nope", Some(42)).is_err());
    }
}
