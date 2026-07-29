use std::{any::TypeId, collections::BTreeMap};

use bevy::{
    asset::{AssetPath, LoadFromPath, UntypedHandle},
    ecs::entity::{Entity, EntityHashMap},
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

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
struct Mana(u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct DebugLabel(String);

struct NoAssetLoader;

impl LoadFromPath for NoAssetLoader {
    fn load_from_path_erased(
        &mut self,
        _type_id: TypeId,
        _path: AssetPath<'static>,
    ) -> UntypedHandle {
        panic!("the solution world contains no asset handles")
    }
}

fn app_with_registered_types() -> App {
    let mut app = App::new();
    app.register_type::<Position>()
        .register_type::<Health>()
        .register_type::<Mana>()
        .register_type::<DebugLabel>();
    app
}

fn serialize_allowlisted(app: &App, entities: &[Entity]) -> Result<String, ron::Error> {
    let world = app.world();
    let registry = world.resource::<AppTypeRegistry>().read();
    DynamicWorldBuilder::from_world(world, &registry)
        .deny_all_components()
        .allow_component::<Position>()
        .allow_component::<Health>()
        .allow_component::<Mana>()
        .extract_entities(entities.iter().copied())
        .remove_empty_entities()
        .build()
        .serialize(&registry)
}

fn restore_or_default(serialized: &str) -> (App, Option<String>) {
    let mut app = app_with_registered_types();
    let registry = app.world().resource::<AppTypeRegistry>().clone();
    let registry = registry.read();

    let result = ron::de::Deserializer::from_str(serialized)
        .map_err(|error| error.to_string())
        .and_then(|mut deserializer| {
            WorldDeserializer {
                type_registry: &registry,
                load_from_path: &mut NoAssetLoader,
            }
            .deserialize(&mut deserializer)
            .map_err(|error| error.to_string())
        });
    drop(registry);

    match result {
        Ok(dynamic_world) => {
            if let Err(error) =
                dynamic_world.write_to_world(app.world_mut(), &mut EntityHashMap::default())
            {
                return (app_with_registered_types(), Some(error.to_string()));
            }
            (app, None)
        }
        Err(error) => (app, Some(error)),
    }
}

fn snapshot(app: &mut App) -> BTreeMap<(i32, i32), (u32, Option<u32>)> {
    let mut query = app
        .world_mut()
        .query::<(&Position, &Health, Option<&Mana>)>();
    query
        .iter(app.world())
        .map(|(position, health, mana)| {
            (
                ((position.x * 10.0) as i32, (position.y * 10.0) as i32),
                (health.0, mana.map(|mana| mana.0)),
            )
        })
        .collect()
}

fn main() {
    let mut source = app_with_registered_types();
    let mage = source
        .world_mut()
        .spawn((
            Position { x: 1.0, y: 2.0 },
            Health(90),
            Mana(45),
            DebugLabel("runtime editor label".to_owned()),
        ))
        .id();
    let fighter = source
        .world_mut()
        .spawn((
            Position { x: -3.0, y: 0.5 },
            Health(120),
            DebugLabel("runtime editor label".to_owned()),
        ))
        .id();

    let serialized = serialize_allowlisted(&source, &[mage, fighter]).expect("valid world");
    let (mut restored, error) = restore_or_default(&serialized);
    println!("{serialized}");
    println!("restored={:?}, error={error:?}", snapshot(&mut restored));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mana_is_optional_and_allowlist_excludes_registered_debug_data() {
        let mut source = app_with_registered_types();
        let mage = source
            .world_mut()
            .spawn((
                Position { x: 1.0, y: 2.0 },
                Health(90),
                Mana(45),
                DebugLabel("do not save".to_owned()),
            ))
            .id();
        let fighter = source
            .world_mut()
            .spawn((
                Position { x: -3.0, y: 0.5 },
                Health(120),
                DebugLabel("do not save".to_owned()),
            ))
            .id();

        let serialized = serialize_allowlisted(&source, &[mage, fighter]).unwrap();
        assert!(serialized.contains("Mana"));
        assert!(!serialized.contains("DebugLabel"));

        let (mut restored, error) = restore_or_default(&serialized);
        assert_eq!(error, None);
        assert_eq!(
            snapshot(&mut restored),
            BTreeMap::from([((-30, 5), (120, None)), ((10, 20), (90, Some(45)))])
        );
    }

    #[test]
    fn damaged_ron_returns_an_empty_fallback_world() {
        let (mut restored, error) = restore_or_default("(broken world");
        assert!(error.is_some());
        assert!(snapshot(&mut restored).is_empty());
    }
}
