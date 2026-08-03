use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerCommand {
    Move(Vec2),
    Fire,
}

#[derive(Message, Clone, Copy, Debug)]
struct PlayerCommandMessage(PlayerCommand);

#[derive(Resource)]
struct InputBindings {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    fire: KeyCode,
    gamepad_fire: GamepadButton,
    dead_zone: f32,
}

impl Default for InputBindings {
    fn default() -> Self {
        Self {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            fire: KeyCode::Space,
            gamepad_fire: GamepadButton::South,
            dead_zone: 0.2,
        }
    }
}

#[derive(Resource, Default)]
struct ReceivedCommands(Vec<PlayerCommand>);

fn main() {
    let mut app = build_app();
    {
        let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keyboard.press(KeyCode::KeyW);
        keyboard.press(KeyCode::Space);
    }
    app.update();

    for command in &app.world().resource::<ReceivedCommands>().0 {
        println!("게임 명령: {command:?}");
    }
}

fn build_app() -> App {
    let mut app = App::new();
    app.init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<InputBindings>()
        .init_resource::<ReceivedCommands>()
        .add_message::<PlayerCommandMessage>()
        .add_systems(Update, (translate_input, collect_commands).chain());
    app
}

fn translate_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    gamepads: Query<&Gamepad>,
    mut commands: MessageWriter<PlayerCommandMessage>,
) {
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(bindings.left) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(bindings.right) {
        movement.x += 1.0;
    }
    if keyboard.pressed(bindings.up) {
        movement.y += 1.0;
    }
    if keyboard.pressed(bindings.down) {
        movement.y -= 1.0;
    }

    let mut fire = keyboard.just_pressed(bindings.fire);
    for gamepad in &gamepads {
        let stick = Vec2::new(
            gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0),
            gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0),
        );
        if stick.length() >= bindings.dead_zone {
            movement += stick;
        }
        fire |= gamepad.just_pressed(bindings.gamepad_fire);
    }

    if movement != Vec2::ZERO {
        commands.write(PlayerCommandMessage(PlayerCommand::Move(
            movement.normalize_or_zero(),
        )));
    }
    if fire {
        commands.write(PlayerCommandMessage(PlayerCommand::Fire));
    }
}

fn collect_commands(
    mut messages: MessageReader<PlayerCommandMessage>,
    mut received: ResMut<ReceivedCommands>,
) {
    received.0.extend(messages.read().map(|message| message.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_is_translated_to_device_independent_commands() {
        let mut app = build_app();
        {
            let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keyboard.press(KeyCode::KeyW);
            keyboard.press(KeyCode::KeyD);
            keyboard.press(KeyCode::Space);
        }
        app.update();

        let commands = &app.world().resource::<ReceivedCommands>().0;
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], PlayerCommand::Move(Vec2::ONE.normalize()));
        assert_eq!(commands[1], PlayerCommand::Fire);
    }

    #[test]
    fn rebinding_changes_input_without_changing_gameplay_command() {
        let mut app = build_app();
        app.world_mut().resource_mut::<InputBindings>().fire = KeyCode::KeyF;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        assert_eq!(
            app.world().resource::<ReceivedCommands>().0,
            [PlayerCommand::Fire]
        );
    }
}
