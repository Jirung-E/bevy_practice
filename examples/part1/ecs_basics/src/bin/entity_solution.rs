use bevy::prelude::*;

fn spawn_empty(world: &mut World) -> Entity {
    world.spawn_empty().id()
}

fn main() {
    let mut world = World::new();

    let player = spawn_empty(&mut world);
    let enemy = spawn_empty(&mut world);
    let npc = spawn_empty(&mut world);
    let all_distinct = player != enemy && player != npc && enemy != npc;

    println!("플레이어: {player:?}, 적: {enemy:?}, NPC: {npc:?}");
    println!("모든 ID가 서로 다름: {all_distinct}");

    world.despawn(npc);
    println!(
        "제거한 NPC를 다시 찾을 수 있음: {}",
        world.get_entity(npc).is_ok()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawned_entities_are_distinct_and_despawn_invalidates_the_id() {
        let mut world = World::new();
        let player = spawn_empty(&mut world);
        let enemy = spawn_empty(&mut world);
        let npc = spawn_empty(&mut world);

        assert_ne!(player, enemy);
        assert_ne!(player, npc);
        assert_ne!(enemy, npc);
        assert!(world.get_entity(npc).is_ok());

        world.despawn(npc);

        assert!(world.get_entity(npc).is_err());
    }
}
