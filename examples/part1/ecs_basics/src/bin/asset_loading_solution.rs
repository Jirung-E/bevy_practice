use bevy::{
    asset::{AssetPlugin, LoadState},
    prelude::*,
};

const PREVIEW_PATH: &str = "images/space_survivor_preview.png";

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    Loading,
    Ready,
    Failed,
}

#[derive(Resource)]
struct LessonAssets {
    preview: Handle<Image>,
}

#[derive(Clone, Copy)]
enum TrackedState {
    Loading,
    Loaded,
    Failed,
}

#[derive(Debug, PartialEq)]
struct LoadProgress<'a> {
    total: usize,
    completed: usize,
    failed_paths: Vec<&'a str>,
}

fn summarize_progress<'a>(
    assets: impl IntoIterator<Item = (&'a str, TrackedState)>,
) -> LoadProgress<'a> {
    let mut progress = LoadProgress {
        total: 0,
        completed: 0,
        failed_paths: Vec::new(),
    };
    for (path, state) in assets {
        progress.total += 1;
        match state {
            TrackedState::Loading => {}
            TrackedState::Loaded => progress.completed += 1,
            TrackedState::Failed => progress.failed_paths.push(path),
        }
    }
    progress
}

fn main() {
    let example_progress = summarize_progress([
        ("images/preview.png", TrackedState::Loaded),
        ("audio/hit.ogg", TrackedState::Loading),
        ("scenes/arena.glb", TrackedState::Failed),
    ]);
    println!("진행률 계산 예시: {example_progress:?}");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Asset Loading Solution - Bevy Practice".into(),
                        resolution: (900, 640).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_systems(
            OnEnter(AppState::Loading),
            (log_enter_loading, begin_loading),
        )
        .add_systems(OnExit(AppState::Loading), log_exit_loading)
        .add_systems(Update, check_loading.run_if(in_state(AppState::Loading)))
        .add_systems(OnEnter(AppState::Ready), (log_enter_ready, show_ready))
        .add_systems(OnExit(AppState::Ready), log_exit_ready)
        .add_systems(OnEnter(AppState::Failed), log_enter_failed)
        .run();
}

fn begin_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LessonAssets {
        preview: asset_server.load(PREVIEW_PATH),
    });
}

fn check_loading(
    asset_server: Res<AssetServer>,
    assets: Res<LessonAssets>,
    mut next: ResMut<NextState<AppState>>,
) {
    match asset_server.get_load_state(assets.preview.id()) {
        Some(LoadState::Loaded) => next.set(AppState::Ready),
        Some(LoadState::Failed(error)) => {
            error!("failed to load {PREVIEW_PATH}: {error}");
            next.set(AppState::Failed);
        }
        Some(LoadState::NotLoaded | LoadState::Loading) | None => {}
    }
}

fn show_ready(mut commands: Commands, assets: Res<LessonAssets>) {
    for x in [-190.0, 190.0] {
        commands.spawn((
            Sprite {
                image: assets.preview.clone(),
                custom_size: Some(Vec2::new(340.0, 238.0)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }
}

fn log_enter_loading() {
    info!("enter Loading");
}
fn log_exit_loading() {
    info!("exit Loading");
}
fn log_enter_ready() {
    info!("enter Ready");
}
fn log_exit_ready() {
    info!("exit Ready");
}
fn log_enter_failed() {
    info!("enter Failed; image fallback, silent audio, or default scene required");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_counts_loaded_assets_and_collects_failures() {
        let progress = summarize_progress([
            ("images/player.png", TrackedState::Loaded),
            ("audio/hit.ogg", TrackedState::Loading),
            ("scenes/arena.glb", TrackedState::Failed),
        ]);

        assert_eq!(
            progress,
            LoadProgress {
                total: 3,
                completed: 1,
                failed_paths: vec!["scenes/arena.glb"],
            }
        );
    }
}
