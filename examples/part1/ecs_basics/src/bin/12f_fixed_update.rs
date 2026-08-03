use std::collections::VecDeque;

use bevy::{prelude::*, time::Fixed};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlayerCommand {
    Attack,
}

#[derive(Resource, Default)]
struct RawInput {
    attack_just_pressed: bool,
}

#[derive(Resource, Default)]
struct InputBuffer(VecDeque<PlayerCommand>);

#[derive(Resource, Default, Debug)]
struct Simulation {
    ticks: u32,
    attacks: u32,
    position: f32,
}

#[derive(Resource)]
struct Velocity(f32);

fn main() {
    let mut app = build_app();
    app.world_mut()
        .resource_mut::<RawInput>()
        .attack_just_pressed = true;

    app.world_mut().run_schedule(Update);
    app.world_mut().run_schedule(FixedUpdate);
    app.world_mut().run_schedule(FixedUpdate);

    println!("{:#?}", app.world().resource::<Simulation>());
}

fn build_app() -> App {
    let mut app = App::new();
    app.init_resource::<RawInput>()
        .init_resource::<InputBuffer>()
        .init_resource::<Simulation>()
        .insert_resource(Velocity(6.0))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Update, buffer_input)
        .add_systems(FixedUpdate, simulate);
    app
}

fn buffer_input(mut raw: ResMut<RawInput>, mut buffer: ResMut<InputBuffer>) {
    if raw.attack_just_pressed {
        buffer.0.push_back(PlayerCommand::Attack);
        raw.attack_just_pressed = false;
    }
}

fn simulate(
    fixed_time: Res<Time<Fixed>>,
    velocity: Res<Velocity>,
    mut buffer: ResMut<InputBuffer>,
    mut simulation: ResMut<Simulation>,
) {
    simulation.ticks += 1;
    simulation.position += velocity.0 * fixed_time.timestep().as_secs_f32();
    while let Some(command) = buffer.0.pop_front() {
        match command {
            PlayerCommand::Attack => simulation.attacks += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_update_press_is_consumed_by_fixed_update_exactly_once() {
        let mut app = build_app();
        app.world_mut()
            .resource_mut::<RawInput>()
            .attack_just_pressed = true;

        app.world_mut().run_schedule(Update);
        app.world_mut().run_schedule(FixedUpdate);
        app.world_mut().run_schedule(FixedUpdate);

        let simulation = app.world().resource::<Simulation>();
        assert_eq!(simulation.ticks, 2);
        assert_eq!(simulation.attacks, 1);
        assert!((simulation.position - 0.2).abs() < f32::EPSILON);
    }
}
