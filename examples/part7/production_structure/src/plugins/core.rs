use crate::{resources::Score, schedule::GameSet};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_message::<EnemyDefeated>()
            .configure_sets(
                Update,
                (GameSet::Input, GameSet::Simulation, GameSet::Feedback).chain(),
            );
    }
}

#[derive(Message)]
pub struct EnemyDefeated {
    pub points: u32,
}
