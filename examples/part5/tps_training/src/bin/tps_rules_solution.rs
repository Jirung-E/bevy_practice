use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
struct MovementSettings {
    walk_speed: f32,
    run_speed: f32,
    air_control: f32,
}

fn desired_velocity(input: Vec2, running: bool, settings: MovementSettings) -> Vec3 {
    let direction = input.clamp_length_max(1.0);
    let speed = if running {
        settings.run_speed
    } else {
        settings.walk_speed
    };
    Vec3::new(direction.x, 0.0, direction.y) * speed
}

fn camera_distance(desired: f32, hit_distance: Option<f32>, margin: f32) -> f32 {
    hit_distance
        .map(|distance| (distance - margin).clamp(0.25, desired))
        .unwrap_or(desired)
}

fn blend_weight(elapsed: f32, duration: f32) -> f32 {
    (elapsed / duration.max(f32::EPSILON)).clamp(0.0, 1.0)
}

fn is_walkable(normal: Vec3, maximum_slope_radians: f32) -> bool {
    normal.normalize_or_zero().dot(Vec3::Y) >= maximum_slope_radians.cos()
}

fn controlled_velocity(
    current: Vec3,
    desired: Vec3,
    grounded: bool,
    settings: MovementSettings,
) -> Vec3 {
    if grounded {
        desired
    } else {
        current.lerp(desired, settings.air_control.clamp(0.0, 1.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NavLinks {
    door_open: bool,
}

fn route(links: NavLinks) -> &'static [&'static str] {
    if links.door_open {
        &["start", "door", "goal"]
    } else {
        &["start", "hall", "corner", "goal"]
    }
}

fn next_waypoint(position: Vec3, path: &[Vec3], reached_distance: f32) -> Option<Vec3> {
    path.iter()
        .copied()
        .find(|waypoint| position.distance(*waypoint) > reached_distance)
}

fn main() {
    let settings = MovementSettings {
        walk_speed: 4.0,
        run_speed: 7.0,
        air_control: 0.2,
    };
    let velocity = desired_velocity(Vec2::ONE, true, settings);
    let distance = camera_distance(6.0, Some(2.5), 0.2);
    let blended = blend_weight(0.1, 0.2);
    let walkable = is_walkable(Vec3::Y, 45_f32.to_radians());
    let air_velocity = controlled_velocity(Vec3::ZERO, velocity, false, settings);
    let route = route(NavLinks { door_open: false });
    let waypoint = next_waypoint(Vec3::ZERO, &[Vec3::ZERO, Vec3::X], 0.1);
    println!(
        "velocity={velocity:.2}, camera={distance}, blend={blended}, walkable={walkable}, air={air_velocity:.2}, route={route:?}, waypoint={waypoint:?}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> MovementSettings {
        MovementSettings {
            walk_speed: 4.0,
            run_speed: 7.0,
            air_control: 0.25,
        }
    }

    #[test]
    fn diagonal_input_does_not_move_faster() {
        assert!((desired_velocity(Vec2::ONE, false, settings()).length() - 4.0).abs() < 0.0001);
    }

    #[test]
    fn camera_stops_before_wall() {
        assert_eq!(camera_distance(6.0, Some(2.0), 0.2), 1.8);
        assert_eq!(camera_distance(6.0, None, 0.2), 6.0);
    }

    #[test]
    fn animation_blends_for_point_two_seconds() {
        assert_eq!(blend_weight(0.0, 0.2), 0.0);
        assert_eq!(blend_weight(0.1, 0.2), 0.5);
        assert_eq!(blend_weight(0.2, 0.2), 1.0);
    }

    #[test]
    fn steep_surface_is_not_ground() {
        assert!(is_walkable(Vec3::Y, 45_f32.to_radians()));
        assert!(!is_walkable(Vec3::X, 45_f32.to_radians()));
    }

    #[test]
    fn closed_door_recalculates_route() {
        assert_eq!(
            route(NavLinks { door_open: true }),
            ["start", "door", "goal"]
        );
        assert_eq!(
            route(NavLinks { door_open: false }),
            ["start", "hall", "corner", "goal"]
        );
    }
}
