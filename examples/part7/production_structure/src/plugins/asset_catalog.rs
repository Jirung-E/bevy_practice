use crate::resources::ArenaAssets;
use bevy::prelude::*;

pub struct AssetCatalogPlugin;

impl Plugin for AssetCatalogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArenaAssets>();
    }
}
