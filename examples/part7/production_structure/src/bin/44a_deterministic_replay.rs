use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InputFrame {
    tick: u32,
    move_x: i8,
    fire: bool,
}

#[derive(Resource)]
struct ReplayInput {
    frames: Vec<InputFrame>,
    cursor: usize,
}

#[derive(Resource, Default)]
struct CurrentInput(Option<InputFrame>);

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
struct SimulationState {
    tick: u32,
    position_millimeters: i32,
    score: u32,
}

#[derive(Resource)]
struct DeterministicRng(u64);

impl DeterministicRng {
    fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 32) as u32
    }
}

fn main() {
    let recording = vec![
        InputFrame {
            tick: 0,
            move_x: 1,
            fire: false,
        },
        InputFrame {
            tick: 1,
            move_x: 1,
            fire: true,
        },
        InputFrame {
            tick: 2,
            move_x: 0,
            fire: false,
        },
        InputFrame {
            tick: 3,
            move_x: -1,
            fire: true,
        },
        InputFrame {
            tick: 4,
            move_x: 1,
            fire: true,
        },
    ];

    let first = run_replay(2026, &recording);
    let second = run_replay(2026, &recording);
    println!("recorded frames: {}", recording.len());
    println!("first : {first:?}");
    println!("replay: {second:?}");
    println!("deterministic match: {}", first == second);
}

fn run_replay(seed: u64, frames: &[InputFrame]) -> SimulationState {
    validate_recording(frames).expect("recording ticks must be contiguous");
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(ReplayInput {
            frames: frames.to_vec(),
            cursor: 0,
        })
        .init_resource::<CurrentInput>()
        .insert_resource(SimulationState {
            tick: 0,
            position_millimeters: 0,
            score: 0,
        })
        .insert_resource(DeterministicRng(seed))
        .add_systems(FixedUpdate, (read_recorded_input, simulate_tick).chain());

    for _ in frames {
        app.world_mut().run_schedule(FixedUpdate);
    }
    app.world().resource::<SimulationState>().clone()
}

fn read_recorded_input(mut replay: ResMut<ReplayInput>, mut current: ResMut<CurrentInput>) {
    current.0 = replay.frames.get(replay.cursor).copied();
    replay.cursor += 1;
}

fn simulate_tick(
    current: Res<CurrentInput>,
    mut state: ResMut<SimulationState>,
    mut rng: ResMut<DeterministicRng>,
) {
    let Some(input) = current.0 else { return };
    assert_eq!(input.tick, state.tick, "replay tick drift");
    state.position_millimeters += i32::from(input.move_x) * 100;
    if input.fire {
        state.score += 10 + rng.next_u32() % 5;
    }
    state.tick += 1;
}

fn validate_recording(frames: &[InputFrame]) -> Result<(), String> {
    for (expected, frame) in frames.iter().enumerate() {
        if frame.tick != expected as u32 {
            return Err(format!("expected tick {expected}, found {}", frame.tick));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<InputFrame> {
        vec![
            InputFrame {
                tick: 0,
                move_x: 1,
                fire: true,
            },
            InputFrame {
                tick: 1,
                move_x: -1,
                fire: true,
            },
        ]
    }

    #[test]
    fn same_seed_and_input_reproduce_exact_state() {
        assert_eq!(run_replay(7, &sample()), run_replay(7, &sample()));
    }

    #[test]
    fn rejects_missing_tick() {
        let invalid = [InputFrame {
            tick: 1,
            move_x: 0,
            fire: false,
        }];
        assert!(validate_recording(&invalid).is_err());
    }
}
