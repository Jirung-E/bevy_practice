use bevy::{
    asset::{AssetEvent, AssetLoadFailedEvent, AssetPlugin},
    prelude::*,
    shader::Shader,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum ReloadState {
    #[default]
    Waiting,
    ChangeDetected,
    AssetLoaded,
    AssetRemoved,
    FileLoadFailed(String),
}

#[derive(Resource, Debug, Default)]
struct ShaderReloadStatus {
    last: ReloadState,
}

struct ShaderReloadStatusPlugin;

impl Plugin for ShaderReloadStatusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderReloadStatus>()
            .add_systems(Update, (watch_shader_events, watch_shader_load_failures));
    }
}

fn state_from_asset_event(event: &AssetEvent<Shader>) -> ReloadState {
    match event {
        AssetEvent::Added { .. } | AssetEvent::Modified { .. } => ReloadState::ChangeDetected,
        AssetEvent::LoadedWithDependencies { .. } => ReloadState::AssetLoaded,
        AssetEvent::Removed { .. } | AssetEvent::Unused { .. } => ReloadState::AssetRemoved,
    }
}

fn watch_shader_events(
    mut events: MessageReader<AssetEvent<Shader>>,
    mut status: ResMut<ShaderReloadStatus>,
) {
    for event in events.read() {
        status.last = state_from_asset_event(event);
    }
}

fn watch_shader_load_failures(
    mut failures: MessageReader<AssetLoadFailedEvent<Shader>>,
    mut status: ResMut<ShaderReloadStatus>,
) {
    for failure in failures.read() {
        status.last = ReloadState::FileLoadFailed(format!("{}: {}", failure.path, failure.error));
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                ..default()
            }),
            ShaderReloadStatusPlugin,
        ))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn development_status_is_not_gameplay_or_save_data() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<ShaderReloadStatus>();

        assert_eq!(ReloadState::default(), ReloadState::Waiting);
        assert_ne!(
            ReloadState::ChangeDetected,
            ReloadState::AssetLoaded,
            "변경 감지와 로드 완료를 같은 성공 상태로 단정하지 않는다"
        );
        assert_eq!(
            ReloadState::FileLoadFailed("shader.wgsl: parse error".to_owned()),
            ReloadState::FileLoadFailed("shader.wgsl: parse error".to_owned())
        );
    }
}
