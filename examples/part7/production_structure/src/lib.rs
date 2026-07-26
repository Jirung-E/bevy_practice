mod components;
mod plugins;
mod resources;
mod schedule;

use bevy::{prelude::*, window::WindowResolution};
use plugins::{
    AssetCatalogPlugin, CorePlugin, DiagnosticsPlugin, GameplayPlugin, PresentationPlugin,
};

#[derive(Resource, Clone, Copy)]
pub struct LessonConfig {
    pub assets: bool,
    pub gameplay: bool,
    pub optimized: bool,
}

impl LessonConfig {
    pub const PLUGINS: Self = Self {
        assets: false,
        gameplay: false,
        optimized: false,
    };
    pub const MODULES: Self = Self { ..Self::PLUGINS };
    pub const ASSETS: Self = Self {
        assets: true,
        ..Self::MODULES
    };
    pub const ARCHITECTURE: Self = Self {
        gameplay: true,
        ..Self::ASSETS
    };
    pub const OPTIMIZED: Self = Self {
        optimized: true,
        ..Self::ARCHITECTURE
    };
}

pub fn run(config: LessonConfig) {
    let mut app = App::new();
    app.insert_resource(config)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Production Arena - Bevy Practice".into(),
                resolution: WindowResolution::new(1100, 700),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CorePlugin, PresentationPlugin));

    if config.assets {
        app.add_plugins(AssetCatalogPlugin);
    }
    if config.gameplay {
        app.add_plugins(GameplayPlugin);
    }
    if config.optimized {
        app.add_plugins(DiagnosticsPlugin);
    }
    app.run();
}
