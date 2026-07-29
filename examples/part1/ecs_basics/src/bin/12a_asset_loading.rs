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

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Asset Loading - Bevy Practice".into(),
                        resolution: (900, 640).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Loading), begin_loading)
        .add_systems(Update, check_loading.run_if(in_state(AppState::Loading)))
        .add_systems(OnEnter(AppState::Ready), show_loaded_asset)
        .add_systems(OnEnter(AppState::Failed), show_fallback)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn begin_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let preview = asset_server.load(PREVIEW_PATH);
    commands.insert_resource(LessonAssets { preview });

    commands.spawn((
        DespawnOnExit(AppState::Loading),
        Text::new("LOADING ASSET..."),
        TextFont {
            font_size: FontSize::Px(36.0),
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.85, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50),
            top: percent(50),
            ..default()
        },
        Transform::from_translation(Vec3::new(-175.0, -18.0, 0.0)),
    ));
}

fn check_loading(
    asset_server: Res<AssetServer>,
    assets: Res<LessonAssets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match asset_server.get_load_state(assets.preview.id()) {
        Some(LoadState::Loaded) => next_state.set(AppState::Ready),
        Some(LoadState::Failed(error)) => {
            error!("failed to load {PREVIEW_PATH}: {error}");
            next_state.set(AppState::Failed);
        }
        Some(LoadState::NotLoaded | LoadState::Loading) | None => {}
    }
}

fn show_loaded_asset(mut commands: Commands, assets: Res<LessonAssets>) {
    commands.spawn((
        Sprite {
            image: assets.preview.clone(),
            custom_size: Some(Vec2::new(720.0, 504.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -30.0, 0.0),
    ));

    commands.spawn((
        Text::new("READY  |  assets/images/space_survivor_preview.png"),
        TextFont {
            font_size: FontSize::Px(23.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 1.0, 0.65)),
        Node {
            position_type: PositionType::Absolute,
            left: px(18),
            top: px(14),
            ..default()
        },
    ));
}

fn show_fallback(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.8, 0.15, 0.25), Vec2::new(480.0, 260.0)),
        Transform::from_xyz(0.0, -30.0, 0.0),
    ));

    commands.spawn((
        Text::new("LOAD FAILED\nFallback asset is active"),
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50),
            top: percent(50),
            ..default()
        },
        Transform::from_translation(Vec3::new(-170.0, -42.0, 1.0)),
    ));
}
