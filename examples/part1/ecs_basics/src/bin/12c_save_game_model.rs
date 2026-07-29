use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SaveGameV1 {
    version: u32,
    score: u32,
    player_x: f32,
    player_y: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedPlayer {
    position: [f32; 2],
    health: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedProgress {
    stage: u32,
    defeated_enemies: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SaveGame {
    version: u32,
    score: u32,
    high_score: u32,
    player: SavedPlayer,
    progress: SavedProgress,
}

impl Default for SaveGame {
    fn default() -> Self {
        Self {
            version: SAVE_VERSION,
            score: 0,
            high_score: 0,
            player: SavedPlayer {
                position: [0.0, -220.0],
                health: 3,
            },
            progress: SavedProgress {
                stage: 1,
                defeated_enemies: 0,
            },
        }
    }
}

impl From<SaveGameV1> for SaveGame {
    fn from(old: SaveGameV1) -> Self {
        Self {
            version: SAVE_VERSION,
            score: old.score,
            high_score: old.score,
            player: SavedPlayer {
                position: [old.player_x, old.player_y],
                health: 3,
            },
            progress: SavedProgress {
                stage: 1,
                defeated_enemies: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoadOrigin {
    Current,
    MigratedV1,
}

fn encode(save: &SaveGame) -> Result<String, ron::Error> {
    ron::ser::to_string_pretty(save, ron::ser::PrettyConfig::default())
}

fn decode(source: &str) -> Result<(SaveGame, LoadOrigin), String> {
    if let Ok(current) = ron::from_str::<SaveGame>(source)
        && current.version == SAVE_VERSION
    {
        return Ok((current, LoadOrigin::Current));
    }

    if let Ok(old) = ron::from_str::<SaveGameV1>(source)
        && old.version == 1
    {
        return Ok((old.into(), LoadOrigin::MigratedV1));
    }

    Err("unsupported or damaged save data".to_owned())
}

fn load_or_default(source: &str) -> (SaveGame, Option<String>) {
    match decode(source) {
        Ok((save, _)) => (save, None),
        Err(error) => (SaveGame::default(), Some(error)),
    }
}

fn main() {
    let current = SaveGame {
        score: 1_300,
        high_score: 2_500,
        player: SavedPlayer {
            position: [120.0, -80.0],
            health: 2,
        },
        progress: SavedProgress {
            stage: 4,
            defeated_enemies: 38,
        },
        ..SaveGame::default()
    };
    let ron = encode(&current).expect("save data should serialize");
    let (restored, origin) = decode(&ron).expect("current save should load");
    let (fallback, fallback_error) = load_or_default("damaged");
    println!("{ron}");
    println!("origin={origin:?}, restored={restored:#?}");
    println!(
        "damaged fallback health={}, error={fallback_error:?}",
        fallback.player.health
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_save_round_trips_all_progress() {
        let save = SaveGame {
            score: 1_300,
            high_score: 2_500,
            player: SavedPlayer {
                position: [120.0, -80.0],
                health: 2,
            },
            progress: SavedProgress {
                stage: 4,
                defeated_enemies: 38,
            },
            ..SaveGame::default()
        };

        assert_eq!(
            decode(&encode(&save).unwrap()).unwrap(),
            (save, LoadOrigin::Current)
        );
    }

    #[test]
    fn version_one_migrates_missing_fields_with_explicit_defaults() {
        let old = SaveGameV1 {
            version: 1,
            score: 700,
            player_x: 20.0,
            player_y: -30.0,
        };
        let source = ron::to_string(&old).unwrap();
        let (migrated, origin) = decode(&source).unwrap();

        assert_eq!(origin, LoadOrigin::MigratedV1);
        assert_eq!(migrated.version, SAVE_VERSION);
        assert_eq!(migrated.high_score, 700);
        assert_eq!(migrated.player.health, 3);
        assert_eq!(migrated.progress.stage, 1);
    }

    #[test]
    fn damaged_or_unknown_version_uses_default_without_panicking() {
        let (damaged, damaged_error) = load_or_default("not ron");
        assert_eq!(damaged, SaveGame::default());
        assert!(damaged_error.is_some());

        let unknown = SaveGame {
            version: 99,
            ..SaveGame::default()
        };
        let (fallback, unknown_error) = load_or_default(&encode(&unknown).unwrap());
        assert_eq!(fallback, SaveGame::default());
        assert!(unknown_error.is_some());
    }
}
