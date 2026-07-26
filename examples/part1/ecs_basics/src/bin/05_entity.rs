use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_entities);
    app.update();
}

fn spawn_entities(mut commands: Commands) {
    let player = commands.spawn_empty().id();
    let enemy = commands.spawn_empty().id();

    println!("플레이어 Entity: {player:?}");
    println!("적 Entity: {enemy:?}");
}
