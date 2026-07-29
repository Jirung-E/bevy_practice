use std::{
    any::TypeId,
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    asset::{AssetPath, LoadFromPath, UntypedHandle},
    ecs::entity::EntityHashMap,
    prelude::*,
    world_serialization::serde::WorldDeserializer,
};
use serde::de::DeserializeSeed;

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
struct Health(u32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct RuntimeSession(u64);

#[derive(Debug, PartialEq)]
struct RestoredEntity {
    position: Position,
    health: Health,
    has_runtime_session: bool,
}

struct NoAssetLoader;

impl LoadFromPath for NoAssetLoader {
    fn load_from_path_erased(
        &mut self,
        _type_id: TypeId,
        _path: AssetPath<'static>,
    ) -> UntypedHandle {
        panic!("this lesson world does not contain asset handles")
    }
}

fn registered_app() -> App {
    let mut app = App::new();
    app.register_type::<Position>().register_type::<Health>();
    app
}

fn build_lesson_world() -> App {
    let mut app = registered_app();
    app.world_mut().spawn((
        Position { x: -2.0, y: 1.5 },
        Health(80),
        RuntimeSession(10_001),
    ));
    app.world_mut().spawn((
        Position { x: 4.0, y: -3.0 },
        Health(35),
        RuntimeSession(10_002),
    ));
    app
}

fn serialize_world(app: &App) -> Result<String, ron::Error> {
    let world = app.world();
    let registry = world.resource::<AppTypeRegistry>().read();
    DynamicWorld::from_world_with(world, &registry).serialize(&registry)
}

fn restore_world(serialized: &str) -> Result<App, String> {
    let mut app = registered_app();
    let registry = app.world().resource::<AppTypeRegistry>().clone();
    let registry = registry.read();
    let mut ron_deserializer =
        ron::de::Deserializer::from_str(serialized).map_err(|error| error.to_string())?;
    let dynamic_world = WorldDeserializer {
        type_registry: &registry,
        load_from_path: &mut NoAssetLoader,
    }
    .deserialize(&mut ron_deserializer)
    .map_err(|error| error.to_string())?;
    drop(registry);

    dynamic_world
        .write_to_world(app.world_mut(), &mut EntityHashMap::default())
        .map_err(|error| error.to_string())?;
    Ok(app)
}

fn restored_entities(app: &mut App) -> Vec<RestoredEntity> {
    let mut query = app
        .world_mut()
        .query::<(&Position, &Health, Option<&RuntimeSession>)>();
    let mut entities: Vec<_> = query
        .iter(app.world())
        .map(|(position, health, runtime)| RestoredEntity {
            position: *position,
            health: *health,
            has_runtime_session: runtime.is_some(),
        })
        .collect();
    entities.sort_by(|left, right| left.position.x.total_cmp(&right.position.x));
    entities
}

fn output_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/12b_dynamic_world.scn.ron")
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let source = build_lesson_world();
    let serialized = serialize_world(&source)?;
    let path = output_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, &serialized)?;

    let loaded = fs::read_to_string(&path)?;
    let mut restored = restore_world(&loaded).map_err(std::io::Error::other)?;
    let entities = restored_entities(&mut restored);

    println!("saved: {}", path.display());
    println!("{serialized}");
    for entity in entities {
        println!(
            "restored position=({}, {}), health={}, runtime_session={}",
            entity.position.x, entity.position.y, entity.health.0, entity.has_runtime_session
        );
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("dynamic world round trip failed: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_and_health_round_trip_without_runtime_component() {
        let source = build_lesson_world();
        let serialized = serialize_world(&source).unwrap();

        assert!(serialized.contains("Position"));
        assert!(serialized.contains("Health"));
        assert!(!serialized.contains("RuntimeSession"));

        let mut restored = restore_world(&serialized).unwrap();
        assert_eq!(
            restored_entities(&mut restored),
            [
                RestoredEntity {
                    position: Position { x: -2.0, y: 1.5 },
                    health: Health(80),
                    has_runtime_session: false,
                },
                RestoredEntity {
                    position: Position { x: 4.0, y: -3.0 },
                    health: Health(35),
                    has_runtime_session: false,
                },
            ]
        );
    }

    #[test]
    fn damaged_ron_returns_an_error_instead_of_panicking() {
        let result = restore_world("(entities: definitely-not-valid)");
        assert!(result.is_err());
    }
}
