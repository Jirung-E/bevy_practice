use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Debug)]
struct FireCooldown {
    interval: f32,
    accumulated: f32,
}

impl FireCooldown {
    fn shots_this_frame(&mut self, delta_seconds: f32, pressed: bool) -> u32 {
        if !pressed {
            return 0;
        }
        self.accumulated += delta_seconds;
        let shots = (self.accumulated / self.interval).floor() as u32;
        self.accumulated -= shots as f32 * self.interval;
        shots
    }
}

fn shot_direction(key: KeyCode) -> Vec2 {
    match key {
        KeyCode::KeyQ => Vec2::new(-1.0, 1.0).normalize(),
        KeyCode::KeyE => Vec2::new(1.0, 1.0).normalize(),
        _ => Vec2::Y,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Difficulty {
    stage: u32,
    spawn_interval: f32,
    enemy_speed: f32,
}

fn difficulty_at(elapsed_seconds: f32) -> Difficulty {
    let stage = (elapsed_seconds / 15.0).floor() as u32;
    Difficulty {
        stage,
        spawn_interval: (0.9 - stage as f32 * 0.1).max(0.25),
        enemy_speed: 135.0 + stage as f32 * 20.0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EnemySpec {
    speed: f32,
    size: f32,
}

fn enemy_spec(sequence: u32, difficulty: Difficulty) -> EnemySpec {
    EnemySpec {
        speed: difficulty.enemy_speed + (sequence % 3) as f32 * 15.0,
        size: if sequence.is_multiple_of(5) {
            64.0
        } else {
            38.0
        },
    }
}

#[derive(Clone, Copy)]
struct Body {
    entity: Entity,
    position: Vec2,
    size: Vec2,
}

#[derive(Debug, Default, PartialEq)]
struct CollisionResult {
    bullets: HashSet<Entity>,
    enemies: HashSet<Entity>,
    score: u32,
}

fn overlaps(a: Body, b: Body) -> bool {
    let distance = (a.position - b.position).abs();
    distance.x < (a.size.x + b.size.x) / 2.0 && distance.y < (a.size.y + b.size.y) / 2.0
}

fn resolve_collisions(bullets: &[Body], enemies: &[Body], points: u32) -> CollisionResult {
    let mut result = CollisionResult::default();
    for &bullet in bullets {
        if result.bullets.contains(&bullet.entity) {
            continue;
        }
        for &enemy in enemies {
            if result.enemies.contains(&enemy.entity) || !overlaps(bullet, enemy) {
                continue;
            }
            result.bullets.insert(bullet.entity);
            result.enemies.insert(enemy.entity);
            result.score += points;
            break;
        }
    }
    result
}

fn main() {
    let mut cooldown = FireCooldown {
        interval: 0.2,
        accumulated: 0.0,
    };
    let shots = cooldown.shots_this_frame(1.0, true);
    let difficulty = difficulty_at(45.0);
    let spec = enemy_spec(5, difficulty);

    let mut world = World::new();
    let bullet = Body {
        entity: world.spawn_empty().id(),
        position: Vec2::ZERO,
        size: Vec2::splat(8.0),
    };
    let enemy = Body {
        entity: world.spawn_empty().id(),
        position: Vec2::ZERO,
        size: Vec2::splat(spec.size),
    };
    let collision = resolve_collisions(&[bullet], &[enemy], 250);

    println!(
        "1초 발사 수: {shots}, 난이도: {difficulty:?}, 적: {spec:?}, 점수: {}, Q 방향: {}",
        collision.score,
        shot_direction(KeyCode::KeyQ)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body(world: &mut World, position: Vec2) -> Body {
        Body {
            entity: world.spawn_empty().id(),
            position,
            size: Vec2::splat(10.0),
        }
    }

    #[test]
    fn cooldown_produces_five_shots_per_second_independent_of_frame_chunks() {
        let mut coarse = FireCooldown {
            interval: 0.2,
            accumulated: 0.0,
        };
        let coarse_count = coarse.shots_this_frame(1.0, true);

        let mut fine = FireCooldown {
            interval: 0.2,
            accumulated: 0.0,
        };
        let fine_count: u32 = (0..100).map(|_| fine.shots_this_frame(0.01, true)).sum();

        assert_eq!(coarse_count, 5);
        assert_eq!(fine_count, 5);
        assert!((shot_direction(KeyCode::KeyQ).length() - 1.0).abs() < f32::EPSILON);
        assert!((shot_direction(KeyCode::KeyE).length() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn difficulty_is_bounded_and_every_fifth_enemy_is_large() {
        let difficulty = difficulty_at(300.0);
        assert_eq!(difficulty.spawn_interval, 0.25);
        assert!(difficulty.enemy_speed > 135.0);
        assert_eq!(enemy_spec(5, difficulty).size, 64.0);
        assert_eq!(enemy_spec(6, difficulty).size, 38.0);
    }

    #[test]
    fn one_bullet_overlapping_two_enemies_scores_once() {
        let mut world = World::new();
        let bullet = body(&mut world, Vec2::ZERO);
        let enemy_a = body(&mut world, Vec2::ZERO);
        let enemy_b = body(&mut world, Vec2::new(1.0, 0.0));

        let result = resolve_collisions(&[bullet], &[enemy_a, enemy_b], 250);

        assert_eq!(result.score, 250);
        assert_eq!(result.bullets.len(), 1);
        assert_eq!(result.enemies.len(), 1);
    }
}
