use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum PluginId {
    Core,
    Assets,
    Gameplay,
    Presentation,
    Diagnostics,
    Pause,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppProfile {
    Client,
    DedicatedServer,
    AutomatedTest,
}

fn plugins_for(profile: AppProfile) -> Vec<PluginId> {
    let mut plugins = vec![PluginId::Core, PluginId::Assets, PluginId::Gameplay];
    match profile {
        AppProfile::Client => {
            plugins.extend([
                PluginId::Presentation,
                PluginId::Diagnostics,
                PluginId::Pause,
            ]);
        }
        AppProfile::DedicatedServer => plugins.push(PluginId::Diagnostics),
        AppProfile::AutomatedTest => {}
    }
    plugins
}

trait GameplayPort {
    fn queue_attack(&mut self, actor: u64);
}

#[derive(Debug, Default)]
struct GameplayService {
    queued_attacks: Vec<u64>,
}

impl GameplayPort for GameplayService {
    fn queue_attack(&mut self, actor: u64) {
        self.queued_attacks.push(actor);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LoadState {
    Loading,
    Loaded,
    Failed,
    Fallback,
}

#[derive(Debug, Default)]
struct AssetLoading {
    states: HashMap<&'static str, LoadState>,
}

impl AssetLoading {
    fn progress(&self) -> f32 {
        if self.states.is_empty() {
            return 1.0;
        }
        let finished = self
            .states
            .values()
            .filter(|state| **state != LoadState::Loading)
            .count();
        finished as f32 / self.states.len() as f32
    }

    fn use_fallbacks(&mut self) {
        for state in self.states.values_mut() {
            if *state == LoadState::Failed {
                *state = LoadState::Fallback;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InputCommand {
    MoveLeft,
    MoveRight,
    Attack,
}

#[derive(Debug, Default)]
struct InputBuffer {
    commands: VecDeque<InputCommand>,
}

impl InputBuffer {
    fn push_from_update(&mut self, command: InputCommand) {
        self.commands.push_back(command);
    }

    fn drain_for_fixed_update(&mut self) -> impl Iterator<Item = InputCommand> + '_ {
        self.commands.drain(..)
    }
}

#[derive(Clone, Copy, Debug)]
struct PerformanceSample {
    frame_ms: f64,
    entity_count: usize,
    enemy_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Regression {
    Pass,
    FrameTime,
    ScenarioMismatch,
}

fn compare_performance(
    baseline: PerformanceSample,
    candidate: PerformanceSample,
    tolerance: f64,
) -> Regression {
    if baseline.entity_count != candidate.entity_count
        || baseline.enemy_count != candidate.enemy_count
    {
        return Regression::ScenarioMismatch;
    }
    if candidate.frame_ms > baseline.frame_ms * (1.0 + tolerance) {
        Regression::FrameTime
    } else {
        Regression::Pass
    }
}

fn main() {
    let client = plugins_for(AppProfile::Client);
    let server = plugins_for(AppProfile::DedicatedServer);
    let test = plugins_for(AppProfile::AutomatedTest);
    let mut gameplay = GameplayService::default();
    gameplay.queue_attack(7);

    let mut loading = AssetLoading::default();
    loading.states.insert("arena", LoadState::Loaded);
    loading.states.insert("music", LoadState::Failed);
    loading.use_fallbacks();

    let mut input = InputBuffer::default();
    input.push_from_update(InputCommand::MoveLeft);
    input.push_from_update(InputCommand::MoveRight);
    input.push_from_update(InputCommand::Attack);
    let commands: Vec<_> = input.drain_for_fixed_update().collect();

    let result = compare_performance(
        PerformanceSample {
            frame_ms: 8.0,
            entity_count: 1_100,
            enemy_count: 1_000,
        },
        PerformanceSample {
            frame_ms: 8.4,
            entity_count: 1_100,
            enemy_count: 1_000,
        },
        0.1,
    );

    println!(
        "client={client:?}, server={server:?}, test={test:?}, progress={}, commands={commands:?}, benchmark={result:?}",
        loading.progress()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_have_only_required_plugins() {
        assert!(plugins_for(AppProfile::Client).contains(&PluginId::Presentation));
        assert!(!plugins_for(AppProfile::DedicatedServer).contains(&PluginId::Presentation));
        assert_eq!(
            plugins_for(AppProfile::AutomatedTest),
            [PluginId::Core, PluginId::Assets, PluginId::Gameplay]
        );
    }

    #[test]
    fn public_port_hides_service_storage() {
        let mut service = GameplayService::default();
        let port: &mut dyn GameplayPort = &mut service;
        port.queue_attack(42);
        assert_eq!(service.queued_attacks, [42]);
    }

    #[test]
    fn failed_assets_can_finish_with_fallback() {
        let mut loading = AssetLoading::default();
        loading.states.insert("mesh", LoadState::Loaded);
        loading.states.insert("music", LoadState::Failed);
        assert_eq!(loading.progress(), 1.0);
        loading.use_fallbacks();
        assert_eq!(loading.states["music"], LoadState::Fallback);
    }

    #[test]
    fn fixed_update_consumes_each_buffered_input_once() {
        let mut buffer = InputBuffer::default();
        buffer.push_from_update(InputCommand::Attack);
        assert_eq!(
            buffer.drain_for_fixed_update().collect::<Vec<_>>(),
            [InputCommand::Attack]
        );
        assert_eq!(buffer.drain_for_fixed_update().count(), 0);
    }

    #[test]
    fn benchmark_rejects_scenario_changes_and_slowdowns() {
        let baseline = PerformanceSample {
            frame_ms: 10.0,
            entity_count: 1_100,
            enemy_count: 1_000,
        };
        assert_eq!(
            compare_performance(
                baseline,
                PerformanceSample {
                    frame_ms: 11.5,
                    ..baseline
                },
                0.1
            ),
            Regression::FrameTime
        );
        assert_eq!(
            compare_performance(
                baseline,
                PerformanceSample {
                    enemy_count: 999,
                    ..baseline
                },
                0.1
            ),
            Regression::ScenarioMismatch
        );
    }
}
