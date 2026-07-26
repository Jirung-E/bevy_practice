use bevy::{
    audio::{AddAudioSource, ChannelCount, SampleRate, Source, Volume},
    math::ops,
    prelude::*,
    reflect::TypePath,
    window::WindowResolution,
};
use std::{fs, path::PathBuf, time::Duration};

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 640.0;
const PLAYER_SPEED: f32 = 420.0;
const BULLET_SPEED: f32 = 620.0;
const ENEMY_SPEED: f32 = 135.0;

#[derive(Resource, Clone, Copy)]
pub struct LessonConfig {
    pub shooting: bool,
    pub enemies: bool,
    pub collisions: bool,
    pub ui: bool,
    pub sound: bool,
    pub saving: bool,
    pub game_over: bool,
}

impl LessonConfig {
    pub const MOVEMENT: Self = Self {
        shooting: false,
        enemies: false,
        collisions: false,
        ui: false,
        sound: false,
        saving: false,
        game_over: false,
    };

    pub const SHOOTING: Self = Self {
        shooting: true,
        ..Self::MOVEMENT
    };

    pub const ENEMIES: Self = Self {
        enemies: true,
        ..Self::SHOOTING
    };

    pub const COLLISIONS: Self = Self {
        collisions: true,
        ..Self::ENEMIES
    };

    pub const UI: Self = Self {
        ui: true,
        ..Self::COLLISIONS
    };

    pub const SOUND: Self = Self {
        sound: true,
        ..Self::UI
    };

    pub const SAVING: Self = Self {
        saving: true,
        ..Self::SOUND
    };

    pub const COMPLETE: Self = Self {
        game_over: true,
        ..Self::SAVING
    };
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct GameplayEntity;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct HitBox(Vec2);

#[derive(Component)]
struct Lifetime(Timer);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct GameOverOverlay;

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource)]
struct PlayerHealth(u32);

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource, Default)]
struct SpawnSequence(u32);

#[derive(Resource, Default)]
struct HighScore(u32);

#[derive(Message, Debug)]
struct EnemyDefeated {
    points: u32,
}

#[derive(Asset, TypePath)]
struct BeepAudio {
    frequency: f32,
    duration: Duration,
}

struct BeepDecoder {
    progress: f32,
    progress_per_sample: f32,
    remaining_samples: usize,
    total_samples: usize,
    sample_rate: SampleRate,
}

impl Iterator for BeepDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_samples == 0 {
            return None;
        }
        self.remaining_samples -= 1;
        self.progress = (self.progress + self.progress_per_sample) % 1.0;
        let fade = self.remaining_samples as f32 / self.total_samples as f32;
        Some(ops::sin(std::f32::consts::TAU * self.progress) * fade)
    }
}

impl Source for BeepDecoder {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.remaining_samples)
    }

    fn channels(&self) -> ChannelCount {
        ChannelCount::new(1).expect("mono channel count is valid")
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.total_samples as f32 / self.sample_rate.get() as f32,
        ))
    }
}

impl Decodable for BeepAudio {
    type Decoder = BeepDecoder;

    fn decoder(&self) -> Self::Decoder {
        let sample_rate = 44_100;
        let total_samples = (self.duration.as_secs_f32() * sample_rate as f32) as usize;
        BeepDecoder {
            progress: 0.0,
            progress_per_sample: self.frequency / sample_rate as f32,
            remaining_samples: total_samples,
            total_samples,
            sample_rate: SampleRate::new(sample_rate).expect("44.1 kHz is valid"),
        }
    }
}

#[derive(Resource)]
struct DefeatSound(Handle<BeepAudio>);

pub fn run(config: LessonConfig) {
    let mut app = App::new();
    app.insert_resource(config)
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.055)))
        .insert_resource(PlayerHealth(3))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.9,
            TimerMode::Repeating,
        )))
        .init_resource::<Score>()
        .init_resource::<SpawnSequence>()
        .init_resource::<HighScore>()
        .add_message::<EnemyDefeated>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Survivor - Bevy Practice".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_audio_source::<BeepAudio>()
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_audio, load_high_score))
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            Update,
            (
                move_player,
                shoot,
                spawn_enemies,
                move_dynamic_entities,
                expire_lifetimes,
                detect_collisions,
                apply_score_messages,
                update_hud,
                check_game_over,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), show_game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameState::GameOver)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_audio(mut commands: Commands, mut assets: ResMut<Assets<BeepAudio>>) {
    let handle = assets.add(BeepAudio {
        frequency: 880.0,
        duration: Duration::from_millis(90),
    });
    commands.insert_resource(DefeatSound(handle));
}

fn setup_game(
    mut commands: Commands,
    config: Res<LessonConfig>,
    mut score: ResMut<Score>,
    mut health: ResMut<PlayerHealth>,
) {
    score.0 = 0;
    health.0 = 3;

    commands.spawn((
        Player,
        GameplayEntity,
        HitBox(Vec2::new(44.0, 36.0)),
        Sprite::from_color(Color::srgb(0.2, 0.85, 1.0), Vec2::new(44.0, 36.0)),
        Transform::from_xyz(0.0, -250.0, 1.0),
    ));

    if config.ui {
        commands.spawn((
            Hud,
            Text::new(""),
            TextFont {
                font_size: FontSize::Px(26.0),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: px(16),
                left: px(18),
                ..default()
            },
        ));
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    let half_size = Vec2::new(22.0, 18.0);
    player.translation +=
        (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
    player.translation.x = player.translation.x.clamp(
        -WINDOW_WIDTH / 2.0 + half_size.x,
        WINDOW_WIDTH / 2.0 - half_size.x,
    );
    player.translation.y = player.translation.y.clamp(
        -WINDOW_HEIGHT / 2.0 + half_size.y,
        WINDOW_HEIGHT / 2.0 - half_size.y,
    );
}

fn shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<LessonConfig>,
    player: Single<&Transform, With<Player>>,
) {
    if !config.shooting || !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    commands.spawn((
        Bullet,
        GameplayEntity,
        Velocity(Vec2::Y * BULLET_SPEED),
        Lifetime(Timer::from_seconds(1.4, TimerMode::Once)),
        HitBox(Vec2::new(8.0, 20.0)),
        Sprite::from_color(Color::srgb(1.0, 0.9, 0.3), Vec2::new(8.0, 20.0)),
        Transform::from_translation(player.translation + Vec3::Y * 34.0),
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<LessonConfig>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut sequence: ResMut<SpawnSequence>,
) {
    if !config.enemies || !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    sequence.0 += 1;
    let x = ops::sin(sequence.0 as f32 * 2.17) * (WINDOW_WIDTH / 2.0 - 35.0);
    commands.spawn((
        Enemy,
        GameplayEntity,
        Velocity(Vec2::NEG_Y * ENEMY_SPEED),
        Lifetime(Timer::from_seconds(6.0, TimerMode::Once)),
        HitBox(Vec2::splat(38.0)),
        Sprite::from_color(Color::srgb(1.0, 0.25, 0.35), Vec2::splat(38.0)),
        Transform::from_xyz(x, WINDOW_HEIGHT / 2.0 + 25.0, 1.0),
    ));
}

fn move_dynamic_entities(
    time: Res<Time>,
    mut moving: Query<(&Velocity, &mut Transform), Without<Player>>,
) {
    for (velocity, mut transform) in &mut moving {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

fn expire_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut living: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut living {
        if lifetime.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collisions(
    mut commands: Commands,
    config: Res<LessonConfig>,
    bullets: Query<(Entity, &Transform, &HitBox), With<Bullet>>,
    enemies: Query<(Entity, &Transform, &HitBox), With<Enemy>>,
    player: Single<(&Transform, &HitBox), With<Player>>,
    mut health: ResMut<PlayerHealth>,
    mut defeated: MessageWriter<EnemyDefeated>,
) {
    if !config.collisions {
        return;
    }

    for (bullet_entity, bullet_transform, bullet_box) in &bullets {
        for (enemy_entity, enemy_transform, enemy_box) in &enemies {
            if overlaps(
                bullet_transform.translation.truncate(),
                bullet_box.0,
                enemy_transform.translation.truncate(),
                enemy_box.0,
            ) {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                defeated.write(EnemyDefeated { points: 100 });
            }
        }
    }

    for (enemy_entity, enemy_transform, enemy_box) in &enemies {
        if overlaps(
            player.0.translation.truncate(),
            player.1.0,
            enemy_transform.translation.truncate(),
            enemy_box.0,
        ) {
            commands.entity(enemy_entity).despawn();
            health.0 = health.0.saturating_sub(1);
        }
    }
}

fn overlaps(a_position: Vec2, a_size: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let distance = (a_position - b_position).abs();
    distance.x < (a_size.x + b_size.x) / 2.0 && distance.y < (a_size.y + b_size.y) / 2.0
}

fn apply_score_messages(
    mut commands: Commands,
    config: Res<LessonConfig>,
    mut messages: MessageReader<EnemyDefeated>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    sound: Res<DefeatSound>,
) {
    for message in messages.read() {
        score.0 += message.points;
        if config.sound {
            commands.spawn((
                AudioPlayer(sound.0.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.15)),
            ));
        }
        if config.saving && score.0 > high_score.0 {
            high_score.0 = score.0;
            save_high_score(high_score.0);
        }
    }
}

fn update_hud(
    config: Res<LessonConfig>,
    score: Res<Score>,
    health: Res<PlayerHealth>,
    high_score: Res<HighScore>,
    hud: Option<Single<&mut Text, With<Hud>>>,
) {
    if !config.ui {
        return;
    }
    if let Some(mut text) = hud {
        text.0 = format!(
            "SCORE {:05}   HP {}   BEST {:05}\nWASD / ARROWS: MOVE   SPACE: FIRE",
            score.0, health.0, high_score.0
        );
    }
}

fn check_game_over(
    config: Res<LessonConfig>,
    health: Res<PlayerHealth>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if config.game_over && health.0 == 0 {
        next_state.set(GameState::GameOver);
    }
}

fn show_game_over(
    mut commands: Commands,
    gameplay: Query<Entity, With<GameplayEntity>>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    for entity in &gameplay {
        commands.entity(entity).despawn();
    }
    if score.0 > high_score.0 {
        high_score.0 = score.0;
        save_high_score(high_score.0);
    }
    commands.spawn((
        GameOverOverlay,
        Text::new(format!(
            "GAME OVER\nSCORE {}\nBEST {}\n\nPRESS ENTER TO RETRY",
            score.0, high_score.0
        )),
        TextFont {
            font_size: FontSize::Px(42.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(30),
            left: percent(25),
            width: percent(50),
            ..default()
        },
    ));
}

fn restart_game(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    overlays: Query<Entity, With<GameOverOverlay>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
    next_state.set(GameState::Playing);
}

fn save_path() -> PathBuf {
    PathBuf::from("save").join("high_score.txt")
}

fn load_high_score(config: Res<LessonConfig>, mut high_score: ResMut<HighScore>) {
    if !config.saving {
        return;
    }
    if let Ok(contents) = fs::read_to_string(save_path())
        && let Ok(value) = contents.trim().parse()
    {
        high_score.0 = value;
    }
}

fn save_high_score(value: u32) {
    let path = save_path();
    if let Some(parent) = path.parent()
        && let Err(error) = fs::create_dir_all(parent)
    {
        warn!("저장 폴더 생성 실패: {error}");
        return;
    }
    if let Err(error) = fs::write(path, value.to_string()) {
        warn!("최고 점수 저장 실패: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separated_rectangles_do_not_overlap() {
        assert!(!overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(20.0, 0.0),
            Vec2::splat(10.0)
        ));
    }

    #[test]
    fn touching_rectangles_overlap_after_penetration() {
        assert!(overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(9.0, 0.0),
            Vec2::splat(10.0)
        ));
    }
}
