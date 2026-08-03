use bevy::prelude::*;
use std::time::{Duration, Instant};

const BURST_SIZE: usize = 256;
const BURSTS: usize = 200;

#[derive(Component)]
struct TemporaryEffect {
    active: bool,
    position: Vec3,
}

#[derive(Debug)]
struct Measurement {
    elapsed: Duration,
    entity_allocations: usize,
    final_entities: usize,
}

fn main() {
    let spawn_despawn = measure_spawn_despawn(BURST_SIZE, BURSTS);
    let reuse = measure_pool_reuse(BURST_SIZE, BURSTS);

    println!("동일한 임시 효과 {BURST_SIZE}개 × {BURSTS}회");
    println!("spawn/despawn: {spawn_despawn:?}");
    println!("pool reuse   : {reuse:?}");
    println!(
        "측정 시간(참고): baseline {:?}, pool {:?}",
        spawn_despawn.elapsed, reuse.elapsed
    );
    println!(
        "할당 감소: {} → {} (시간은 release와 실제 게임 장면에서 다시 측정)",
        spawn_despawn.entity_allocations, reuse.entity_allocations
    );
    println!(
        "최종 Entity: baseline {}, pool {}",
        spawn_despawn.final_entities, reuse.final_entities
    );
}

fn measure_spawn_despawn(burst_size: usize, bursts: usize) -> Measurement {
    let mut world = World::new();
    let started = Instant::now();
    for burst in 0..bursts {
        let entities = (0..burst_size)
            .map(|index| {
                world
                    .spawn(TemporaryEffect {
                        active: true,
                        position: Vec3::new(index as f32, 0.0, burst as f32),
                    })
                    .id()
            })
            .collect::<Vec<_>>();
        for entity in entities {
            world.despawn(entity);
        }
    }
    Measurement {
        elapsed: started.elapsed(),
        entity_allocations: burst_size * bursts,
        final_entities: world
            .iter_entities()
            .filter(|entity| entity.contains::<TemporaryEffect>())
            .count(),
    }
}

fn measure_pool_reuse(pool_size: usize, bursts: usize) -> Measurement {
    let mut world = World::new();
    let pool = (0..pool_size)
        .map(|_| {
            world
                .spawn(TemporaryEffect {
                    active: false,
                    position: Vec3::ZERO,
                })
                .id()
        })
        .collect::<Vec<_>>();
    let started = Instant::now();
    for burst in 0..bursts {
        for (index, entity) in pool.iter().copied().enumerate() {
            let mut effect = world
                .get_mut::<TemporaryEffect>(entity)
                .expect("pooled effect must exist");
            effect.active = true;
            effect.position = Vec3::new(index as f32, 0.0, burst as f32);
        }
        for entity in pool.iter().copied() {
            world
                .get_mut::<TemporaryEffect>(entity)
                .expect("pooled effect must exist")
                .active = false;
        }
    }
    Measurement {
        elapsed: started.elapsed(),
        entity_allocations: pool_size,
        final_entities: world
            .iter_entities()
            .filter(|entity| entity.contains::<TemporaryEffect>())
            .count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_keeps_a_bounded_entity_count() {
        let result = measure_pool_reuse(8, 100);
        assert_eq!(result.entity_allocations, 8);
        assert_eq!(result.final_entities, 8);
    }

    #[test]
    fn baseline_releases_all_entities() {
        let result = measure_spawn_despawn(8, 100);
        assert_eq!(result.entity_allocations, 800);
        assert_eq!(result.final_entities, 0);
    }
}
