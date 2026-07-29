use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use bevy::{asset::AssetPlugin, prelude::*, window::WindowResolution};
use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 2;
const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 640.0;
const PLAYER_SPEED: f32 = 280.0;

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
                position: [0.0, -180.0],
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
    NewGame,
    Current,
    MigratedV1,
    Fallback,
}

#[derive(Resource, Debug, Clone, PartialEq)]
struct Session {
    score: u32,
    high_score: u32,
    player_position: Vec2,
    health: u32,
    stage: u32,
    defeated_enemies: u32,
    status: String,
}

impl Default for Session {
    fn default() -> Self {
        Self::from_save(SaveGame::default(), "NEW GAME")
    }
}

impl Session {
    fn from_save(save: SaveGame, status: impl Into<String>) -> Self {
        Self {
            score: save.score,
            high_score: save.high_score,
            player_position: Vec2::from_array(save.player.position),
            health: save.player.health,
            stage: save.progress.stage,
            defeated_enemies: save.progress.defeated_enemies,
            status: status.into(),
        }
    }

    fn to_save(&self) -> SaveGame {
        SaveGame {
            version: SAVE_VERSION,
            score: self.score,
            high_score: self.high_score.max(self.score),
            player: SavedPlayer {
                position: self.player_position.to_array(),
                health: self.health,
            },
            progress: SavedProgress {
                stage: self.stage,
                defeated_enemies: self.defeated_enemies,
            },
        }
    }
}

#[derive(Resource)]
struct SavePath(PathBuf);

#[derive(Component)]
struct PlayerView;

#[derive(Component)]
struct Hud;

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

fn write_lesson_fixture(path: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn save_atomic(path: &Path, save: &SaveGame) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let temporary = sibling_with_suffix(path, ".tmp");
    let backup = sibling_with_suffix(path, ".bak");
    let contents = encode(save).map_err(|error| error.to_string())?;
    write_and_sync(&temporary, &contents).map_err(|error| error.to_string())?;

    let had_previous = path.exists();
    if had_previous {
        if backup.exists() {
            fs::remove_file(&backup).map_err(|error| error.to_string())?;
        }
        fs::rename(path, &backup).map_err(|error| error.to_string())?;
    }

    match fs::rename(&temporary, path) {
        Ok(()) => {
            if had_previous {
                fs::remove_file(backup).map_err(|error| error.to_string())?;
            }
            Ok(())
        }
        Err(error) => {
            if had_previous {
                let _ = fs::rename(&backup, path);
            }
            let _ = fs::remove_file(temporary);
            Err(error.to_string())
        }
    }
}

fn load_path(path: &Path) -> (SaveGame, LoadOrigin, Option<String>) {
    if !path.exists() {
        return (SaveGame::default(), LoadOrigin::NewGame, None);
    }

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            return (
                SaveGame::default(),
                LoadOrigin::Fallback,
                Some(error.to_string()),
            );
        }
    };
    match decode(&source) {
        Ok((save, origin)) => (save, origin, None),
        Err(error) => (SaveGame::default(), LoadOrigin::Fallback, Some(error)),
    }
}

fn default_save_path() -> PathBuf {
    if let Some(path) = std::env::var_os("BEVY_PRACTICE_SAVE_PATH") {
        return PathBuf::from(path);
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/19a_save_game/save.ron")
}

fn setup(mut commands: Commands, session: Res<Session>, path: Res<SavePath>) {
    commands.spawn(Camera2d);
    commands.spawn((
        PlayerView,
        Sprite::from_color(Color::srgb(0.15, 0.75, 1.0), Vec2::new(48.0, 48.0)),
        Transform::from_translation(session.player_position.extend(0.0)),
    ));
    commands.spawn((
        Hud,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: px(18),
            top: px(18),
            ..default()
        },
    ));
    info!("save path: {}", path.0.display());
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    path: Res<SavePath>,
    mut session: ResMut<Session>,
) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    session.player_position += direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    session.player_position.x = session
        .player_position
        .x
        .clamp(-WINDOW_WIDTH / 2.0 + 24.0, WINDOW_WIDTH / 2.0 - 24.0);
    session.player_position.y = session
        .player_position
        .y
        .clamp(-WINDOW_HEIGHT / 2.0 + 24.0, WINDOW_HEIGHT / 2.0 - 24.0);

    if keys.just_pressed(KeyCode::KeyH) {
        session.health = session.health.saturating_sub(1);
        session.status = "DAMAGE APPLIED".to_owned();
    }
    if keys.just_pressed(KeyCode::KeyK) {
        session.score += 100;
        session.high_score = session.high_score.max(session.score);
        session.defeated_enemies += 1;
        session.status = "ENEMY DEFEATED".to_owned();
    }
    if keys.just_pressed(KeyCode::KeyP) {
        session.stage += 1;
        session.status = "STAGE ADVANCED".to_owned();
    }
    if keys.just_pressed(KeyCode::KeyR) {
        *session = Session::default();
        session.status = "SESSION RESET (NOT SAVED)".to_owned();
    }
    if keys.just_pressed(KeyCode::F5) {
        match save_atomic(&path.0, &session.to_save()) {
            Ok(()) => session.status = "SAVED".to_owned(),
            Err(error) => session.status = format!("SAVE ERROR: {error}"),
        }
    }
    if keys.just_pressed(KeyCode::F9) {
        let (save, origin, error) = load_path(&path.0);
        *session = Session::from_save(save, format!("LOADED: {origin:?}"));
        if let Some(error) = error {
            session.status = format!("FALLBACK: {error}");
        }
    }
    if keys.just_pressed(KeyCode::F6) {
        let legacy = SaveGameV1 {
            version: 1,
            score: session.score,
            player_x: session.player_position.x,
            player_y: session.player_position.y,
        };
        match ron::ser::to_string_pretty(&legacy, ron::ser::PrettyConfig::default())
            .map_err(|error| error.to_string())
            .and_then(|source| write_lesson_fixture(&path.0, &source))
        {
            Ok(()) => session.status = "WROTE VERSION 1 (PRESS F9)".to_owned(),
            Err(error) => session.status = format!("LEGACY WRITE ERROR: {error}"),
        }
    }
    if keys.just_pressed(KeyCode::F7) {
        match write_lesson_fixture(&path.0, "damaged save data") {
            Ok(()) => session.status = "CORRUPTED FILE (PRESS F9)".to_owned(),
            Err(error) => session.status = format!("CORRUPT WRITE ERROR: {error}"),
        }
    }
}

fn update_view(
    session: Res<Session>,
    path: Res<SavePath>,
    mut player: Query<&mut Transform, With<PlayerView>>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    if !session.is_changed() {
        return;
    }
    for mut transform in &mut player {
        transform.translation = session.player_position.extend(0.0);
    }
    for mut text in &mut hud {
        **text = format!(
            "SAVE GAME V{SAVE_VERSION}\n\
             WASD: MOVE  H: DAMAGE  K: +SCORE  P: NEXT STAGE\n\
             F5: SAVE  F9: LOAD  F6: WRITE V1  F7: CORRUPT  R: RESET\n\n\
             SCORE {:05}  BEST {:05}  HP {}  STAGE {}  DEFEATED {}\n\
             POSITION ({:.1}, {:.1})\n\
             STATUS: {}\n\
             PATH: {}",
            session.score,
            session.high_score,
            session.health,
            session.stage,
            session.defeated_enemies,
            session.player_position.x,
            session.player_position.y,
            session.status,
            path.0.display()
        );
    }
}

fn main() {
    let path = default_save_path();
    let (save, origin, error) = load_path(&path);
    let status = error
        .map(|error| format!("STARTUP FALLBACK: {error}"))
        .unwrap_or_else(|| format!("STARTUP: {origin:?}"));

    App::new()
        .insert_resource(Session::from_save(save, status))
        .insert_resource(SavePath(path))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Save Game Round Trip - Bevy Practice".to_owned(),
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as u32,
                            WINDOW_HEIGHT as u32,
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_view).chain())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_save() -> SaveGame {
        SaveGame {
            version: SAVE_VERSION,
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
        }
    }

    fn test_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "bevy_practice_19a_{}_{}_save.ron",
            std::process::id(),
            label
        ))
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(sibling_with_suffix(path, ".tmp"));
        let _ = fs::remove_file(sibling_with_suffix(path, ".bak"));
    }

    #[test]
    fn current_save_round_trips_position_health_and_progress() {
        let path = test_path("current");
        cleanup(&path);
        let expected = sample_save();
        save_atomic(&path, &expected).unwrap();

        let (actual, origin, error) = load_path(&path);
        assert_eq!(actual, expected);
        assert_eq!(origin, LoadOrigin::Current);
        assert_eq!(error, None);
        cleanup(&path);
    }

    #[test]
    fn version_one_migrates_to_current_defaults() {
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
    fn damaged_file_falls_back_without_overwriting_the_evidence() {
        let path = test_path("damaged");
        cleanup(&path);
        fs::write(&path, "damaged save data").unwrap();

        let (save, origin, error) = load_path(&path);
        assert_eq!(save, SaveGame::default());
        assert_eq!(origin, LoadOrigin::Fallback);
        assert!(error.is_some());
        assert_eq!(fs::read_to_string(&path).unwrap(), "damaged save data");
        cleanup(&path);
    }

    #[test]
    fn session_conversion_does_not_store_runtime_status() {
        let session = Session::from_save(sample_save(), "TRANSIENT UI MESSAGE");
        let ron = encode(&session.to_save()).unwrap();
        assert!(!ron.contains("TRANSIENT"));
        assert_eq!(decode(&ron).unwrap().0, sample_save());
    }
}
