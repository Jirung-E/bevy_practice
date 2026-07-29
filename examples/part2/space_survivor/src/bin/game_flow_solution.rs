use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 1;

#[derive(Debug, PartialEq)]
struct HeartHud {
    visible: [bool; 3],
    last_health: u8,
}

impl HeartHud {
    fn new(health: u8) -> Self {
        let mut hud = Self {
            visible: [false; 3],
            last_health: u8::MAX,
        };
        hud.update_if_changed(health);
        hud
    }

    fn update_if_changed(&mut self, health: u8) -> bool {
        if self.last_health == health {
            return false;
        }

        self.visible = std::array::from_fn(|index| index < usize::from(health.min(3)));
        self.last_health = health;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct AudioSettings {
    music_volume: f32,
    effects_volume: f32,
}

impl AudioSettings {
    fn normalized(self) -> Self {
        Self {
            music_volume: self.music_volume.clamp(0.0, 1.0),
            effects_volume: self.effects_volume.clamp(0.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct PlayStats {
    sessions: u32,
    defeated_enemies: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SaveGame {
    version: u32,
    settings: AudioSettings,
    high_score: u32,
    last_score: u32,
    stats: PlayStats,
}

impl SaveGame {
    fn new() -> Self {
        Self {
            version: SAVE_VERSION,
            settings: AudioSettings {
                music_volume: 0.6,
                effects_volume: 0.8,
            },
            high_score: 0,
            last_score: 0,
            stats: PlayStats::default(),
        }
    }

    fn encode(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
    }

    fn decode(source: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(source)
    }
}

fn sibling_with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(suffix);
    path.with_file_name(name)
}

fn write_and_sync(path: &Path, contents: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()
}

fn save_with_rollback(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temporary = sibling_with_suffix(path, ".tmp");
    let backup = sibling_with_suffix(path, ".bak");
    write_and_sync(&temporary, contents)?;

    let had_previous = path.exists();
    if had_previous {
        if backup.exists() {
            fs::remove_file(&backup)?;
        }
        fs::rename(path, &backup)?;
    }

    match fs::rename(&temporary, path) {
        Ok(()) => {
            if had_previous {
                fs::remove_file(backup)?;
            }
            Ok(())
        }
        Err(error) => {
            if had_previous {
                let _ = fs::rename(&backup, path);
            }
            let _ = fs::remove_file(temporary);
            Err(error)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Debug)]
struct GameFlow {
    state: GameState,
    survival_seconds: f32,
    spawn_elapsed: f32,
    spawn_interval: f32,
}

impl GameFlow {
    fn new(spawn_interval: f32) -> Self {
        Self {
            state: GameState::Menu,
            survival_seconds: 0.0,
            spawn_elapsed: 0.0,
            spawn_interval,
        }
    }

    fn start(&mut self) {
        self.state = GameState::Playing;
        self.survival_seconds = 0.0;
        self.spawn_elapsed = 0.0;
    }

    fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            state => state,
        };
    }

    fn game_over(&mut self) {
        if self.state == GameState::Playing {
            self.state = GameState::GameOver;
        }
    }

    fn update(&mut self, delta_seconds: f32) -> u32 {
        if self.state != GameState::Playing {
            return 0;
        }

        self.survival_seconds += delta_seconds;
        self.spawn_elapsed += delta_seconds;
        let spawned = (self.spawn_elapsed / self.spawn_interval).floor() as u32;
        self.spawn_elapsed -= spawned as f32 * self.spawn_interval;
        spawned
    }
}

fn main() {
    let mut hud = HeartHud::new(3);
    hud.update_if_changed(2);

    let settings = AudioSettings {
        music_volume: 0.55,
        effects_volume: 0.85,
    }
    .normalized();

    let mut flow = GameFlow::new(0.8);
    flow.start();
    let spawned = flow.update(1.7);
    flow.toggle_pause();
    flow.toggle_pause();
    flow.game_over();

    let mut save = SaveGame::new();
    save.settings = settings;
    save.high_score = 2_500;
    save.last_score = 1_300;
    save.stats = PlayStats {
        sessions: 4,
        defeated_enemies: 38,
    };
    let encoded = save.encode().expect("valid save data");
    let decoded = SaveGame::decode(&encoded).expect("round trip should work");

    if let Some(path) = std::env::var_os("BEVY_PRACTICE_WRITE_SAVE") {
        save_with_rollback(Path::new(&path), &encoded).expect("save replacement should work");
        println!("saved={}", Path::new(&path).display());
    }

    println!(
        "hearts={:?}, spawned={spawned}, state={:?}, high_score={}",
        hud.visible, flow.state, decoded.high_score
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hud_updates_only_when_health_changes() {
        let mut hud = HeartHud::new(3);
        assert!(!hud.update_if_changed(3));
        assert!(hud.update_if_changed(1));
        assert_eq!(hud.visible, [true, false, false]);
    }

    #[test]
    fn audio_channels_are_clamped_independently() {
        let settings = AudioSettings {
            music_volume: 1.5,
            effects_volume: -0.25,
        }
        .normalized();
        assert_eq!(settings.music_volume, 1.0);
        assert_eq!(settings.effects_volume, 0.0);
    }

    #[test]
    fn save_data_round_trips_through_ron() {
        let save = SaveGame::new();
        let encoded = save.encode().unwrap();
        assert_eq!(SaveGame::decode(&encoded).unwrap(), save);
    }

    #[test]
    fn replacement_keeps_a_readable_save() {
        let directory =
            std::env::temp_dir().join(format!("bevy_practice_game_flow_{}", std::process::id()));
        let path = directory.join("save.ron");
        save_with_rollback(&path, "old").unwrap();
        save_with_rollback(&path, "new").unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        assert!(!sibling_with_suffix(&path, ".tmp").exists());
        assert!(!sibling_with_suffix(&path, ".bak").exists());

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn pause_does_not_accumulate_spawn_time() {
        let mut flow = GameFlow::new(1.0);
        flow.start();
        assert_eq!(flow.update(0.75), 0);
        flow.toggle_pause();
        assert_eq!(flow.update(10.0), 0);
        flow.toggle_pause();
        assert_eq!(flow.update(0.25), 1);
        flow.game_over();
        assert_eq!(flow.update(10.0), 0);
    }
}
