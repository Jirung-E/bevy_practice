use crate::{LessonConfig, components::Hud, resources::Score, schedule::GameSet};
use bevy::prelude::*;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_presentation)
            .add_systems(Update, update_hud.in_set(GameSet::Feedback));
    }
}

fn setup_presentation(mut commands: Commands, config: Res<LessonConfig>) {
    commands.spawn((
        DirectionalLight {
            illuminance: 11_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.85, -0.45, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        AmbientLight {
            color: Color::srgb(0.4, 0.46, 0.58),
            brightness: 170.0,
            ..default()
        },
        Transform::from_xyz(0.0, 13.0, 14.0).looking_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y),
    ));
    commands.spawn((
        Hud,
        Text::new(if config.gameplay {
            "SCORE 00000\nWASD: MOVE   SPACE: REMOVE NEAR ENEMY"
        } else {
            "PLUGIN SHELL READY"
        }),
        TextFont {
            font_size: FontSize::Px(21.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(18),
            ..default()
        },
    ));
}

fn update_hud(score: Res<Score>, config: Res<LessonConfig>, mut hud: Single<&mut Text, With<Hud>>) {
    if config.optimized && !score.is_changed() {
        return;
    }
    if config.gameplay {
        hud.0 = format!(
            "SCORE {:05}\nWASD: MOVE   SPACE: REMOVE NEAR ENEMY",
            score.0
        );
    }
}
