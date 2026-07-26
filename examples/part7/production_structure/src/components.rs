use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct ArenaEntity;
