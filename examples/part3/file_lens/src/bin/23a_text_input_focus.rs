use bevy::{
    input_focus::{
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
        AutoFocus, InputFocus,
    },
    prelude::*,
    text::{EditableText, TextCursorStyle},
    window::WindowResolution,
};

#[derive(Component)]
struct SubmittedText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Text Input & IME - Bevy Practice".into(),
                resolution: WindowResolution::new(900, 560),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TabNavigationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (submit_focused_input, paint_focus))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(28)),
            row_gap: px(18),
            ..default()
        })
        .with_children(|root| {
            root.spawn((
                Text::new("TEXT INPUT / FOCUS / IME"),
                TextFont {
                    font: FontSource::SansSerif,
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(Color::srgb(0.35, 0.9, 1.0)),
            ));
            root.spawn((
                Text::new("Tab: focus 이동  |  Enter: 현재 입력 확정\n한글을 조합하는 동안에도 글자가 끊기지 않아야 합니다."),
                TextFont {
                    font: FontSource::SansSerif,
                    font_size: FontSize::Px(19.0),
                    ..default()
                },
            ));
            root.spawn((
                AutoFocus,
                TabGroup::new(0),
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    ..default()
                },
            ))
            .with_children(|form| {
                spawn_input(form, "이름을 입력하세요", 0);
                spawn_input(form, "검색어를 입력하세요", 1);
            });
            root.spawn((
                SubmittedText,
                Text::new("SUBMITTED: -"),
                TextFont {
                    font: FontSource::SansSerif,
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.82, 0.28)),
            ));
        });
}

fn spawn_input(parent: &mut ChildSpawnerCommands, initial: &str, tab_index: i32) {
    parent.spawn((
        Name::new(format!("Input {tab_index}")),
        Node {
            width: px(620),
            min_height: px(58),
            border: UiRect::all(px(2)),
            padding: UiRect::all(px(12)),
            ..default()
        },
        EditableText::new(initial),
        TextLayout::no_wrap(),
        TextFont {
            font: FontSource::SansSerif,
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextCursorStyle::default(),
        TabIndex(tab_index),
        BorderColor::all(Color::srgb(0.25, 0.35, 0.48)),
        BackgroundColor(Color::srgb(0.055, 0.07, 0.11)),
    ));
}

fn submit_focused_input(
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    inputs: Query<(&EditableText, &Name)>,
    mut output: Single<&mut Text, With<SubmittedText>>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Some(entity) = focus.get() else { return };
    let Ok((input, name)) = inputs.get(entity) else { return };
    output.0 = format!("SUBMITTED ({name}): {}", input.value());
}

fn paint_focus(
    focus: Res<InputFocus>,
    mut inputs: Query<(Entity, &mut BorderColor), With<EditableText>>,
) {
    if !focus.is_changed() {
        return;
    }
    for (entity, mut border) in &mut inputs {
        *border = BorderColor::all(if focus.get() == Some(entity) {
            Color::srgb(0.25, 0.9, 1.0)
        } else {
            Color::srgb(0.25, 0.35, 0.48)
        });
    }
}
