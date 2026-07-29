use bevy::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(960.0, 640.0);
const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 48.0);
const MAX_SPEED: f32 = 420.0;
const ACCELERATION: f32 = 900.0;

#[derive(Component, Debug, Default, PartialEq)]
struct Velocity(Vec2);

fn input_direction(keyboard: &ButtonInput<KeyCode>) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyJ)
    {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight)
        || keyboard.pressed(KeyCode::KeyD)
        || keyboard.pressed(KeyCode::KeyL)
    {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyI)
    {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyK)
    {
        direction.y -= 1.0;
    }
    direction.normalize_or_zero()
}

fn step_velocity(
    velocity: &mut Velocity,
    direction: Vec2,
    delta_seconds: f32,
    max_speed: f32,
    acceleration: f32,
) {
    let target = direction.normalize_or_zero() * max_speed;
    velocity.0 = velocity
        .0
        .move_towards(target, acceleration * delta_seconds)
        .clamp_length_max(max_speed);
}

fn clamp_player(position: &mut Vec2) {
    let half_window = WINDOW_SIZE / 2.0;
    let half_player = PLAYER_SIZE / 2.0;
    position.x = position.x.clamp(
        -half_window.x + half_player.x,
        half_window.x - half_player.x,
    );
    position.y = position.y.clamp(
        -half_window.y + half_player.y,
        half_window.y - half_player.y,
    );
}

fn main() {
    let mut keyboard = ButtonInput::default();
    keyboard.press(KeyCode::KeyI);
    keyboard.press(KeyCode::KeyL);

    let mut velocity = Velocity::default();
    let mut position = Vec2::ZERO;
    for _ in 0..60 {
        step_velocity(
            &mut velocity,
            input_direction(&keyboard),
            1.0 / 60.0,
            MAX_SPEED,
            ACCELERATION,
        );
        position += velocity.0 / 60.0;
        clamp_player(&mut position);
    }
    println!("위치: {position}, 속도: {}", velocity.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ijkl_diagonal_is_normalized_and_acceleration_is_limited() {
        let mut keyboard = ButtonInput::default();
        keyboard.press(KeyCode::KeyI);
        keyboard.press(KeyCode::KeyL);
        let direction = input_direction(&keyboard);
        assert!((direction.length() - 1.0).abs() < f32::EPSILON);

        let mut velocity = Velocity::default();
        step_velocity(&mut velocity, direction, 0.1, 420.0, 900.0);
        assert!((velocity.0.length() - 90.0).abs() < 0.001);
    }

    #[test]
    fn releasing_input_decelerates_and_player_size_affects_bounds() {
        let mut velocity = Velocity(Vec2::new(420.0, 0.0));
        step_velocity(&mut velocity, Vec2::ZERO, 0.1, 420.0, 900.0);
        assert_eq!(velocity.0, Vec2::new(330.0, 0.0));

        let mut position = Vec2::splat(10_000.0);
        clamp_player(&mut position);
        assert_eq!(position, Vec2::new(448.0, 296.0));
    }
}
