use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.03, 0.14)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Bevy Project".into(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("My First Bevy Screen"),
        TextFont {
            font_size: FontSize::Px(52.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.35)),
    ));
    commands.spawn((
        Text2d::new("Press SPACE to start"),
        TextFont {
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.8, 1.0)),
        Transform::from_xyz(0.0, -70.0, 0.0),
    ));
}
