use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct ArenaAssets {
    pub player_mesh: Handle<Mesh>,
    pub enemy_mesh: Handle<Mesh>,
    pub player_material: Handle<StandardMaterial>,
    pub enemy_material: Handle<StandardMaterial>,
}

impl FromWorld for ArenaAssets {
    fn from_world(world: &mut World) -> Self {
        let player_mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Capsule3d::new(0.42, 0.9));
        let enemy_mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Cuboid::from_length(1.1));
        let player_material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(Color::srgb(0.12, 0.62, 1.0));
        let enemy_material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(Color::srgb(0.95, 0.22, 0.12));
        Self {
            player_mesh,
            enemy_mesh,
            player_material,
            enemy_material,
        }
    }
}
