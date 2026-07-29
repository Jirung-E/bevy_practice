use bevy::prelude::*;

#[derive(Component)]
struct HitFlash;

#[derive(Component)]
struct Poisoned;

#[derive(Component)]
struct Shielded;

#[derive(Clone, Copy, Debug, PartialEq)]
struct EffectUniform {
    color: Vec3,
    strength: f32,
}

const NORMAL: EffectUniform = EffectUniform {
    color: Vec3::ONE,
    strength: 0.0,
};
const POISON: EffectUniform = EffectUniform {
    color: Vec3::new(0.2, 1.0, 0.25),
    strength: 0.55,
};
const SHIELD: EffectUniform = EffectUniform {
    color: Vec3::new(0.2, 0.9, 1.0),
    strength: 0.65,
};

fn flash_strength(elapsed: f32, duration: f32) -> f32 {
    if duration <= 0.0 {
        return 0.0;
    }
    (1.0 - elapsed / duration).clamp(0.0, 1.0)
}

fn effect_from_state(
    hit_elapsed: Option<f32>,
    hit_duration: f32,
    shielded: bool,
    poisoned: bool,
) -> EffectUniform {
    if let Some(elapsed) = hit_elapsed {
        let strength = flash_strength(elapsed, hit_duration);
        if strength > 0.0 {
            return EffectUniform {
                color: Vec3::ONE,
                strength,
            };
        }
    }
    if shielded {
        SHIELD
    } else if poisoned {
        POISON
    } else {
        NORMAL
    }
}

fn atlas_uv_rect(column: usize, columns: usize, rows: usize) -> Vec4 {
    let width = 1.0 / columns as f32;
    let height = 1.0 / rows as f32;
    Vec4::new(column as f32 * width, 0.0, width, height)
}

fn main() {
    let _markers = (HitFlash, Poisoned, Shielded);
    for state in [
        effect_from_state(None, 1.0, false, false),
        effect_from_state(None, 1.0, false, true),
        effect_from_state(None, 1.0, true, true),
        effect_from_state(Some(0.25), 1.0, true, true),
    ] {
        println!("렌더 uniform: {state:?}");
    }
    println!("세 번째 Idle 프레임 UV: {}", atlas_uv_rect(2, 4, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flash_duration_changes_decay_without_changing_endpoints() {
        assert_eq!(flash_strength(0.0, 0.1), 1.0);
        assert_eq!(flash_strength(0.1, 0.1), 0.0);
        assert_eq!(flash_strength(0.0, 1.0), 1.0);
        assert_eq!(flash_strength(1.0, 1.0), 0.0);
    }

    #[test]
    fn effect_priority_is_explicit_and_deterministic() {
        assert_eq!(effect_from_state(None, 1.0, false, false), NORMAL);
        assert_eq!(effect_from_state(None, 1.0, false, true), POISON);
        assert_eq!(effect_from_state(None, 1.0, true, true), SHIELD);
        assert_eq!(
            effect_from_state(Some(0.25), 1.0, true, true),
            EffectUniform {
                color: Vec3::ONE,
                strength: 0.75,
            }
        );
    }

    #[test]
    fn four_column_atlas_offsets_select_each_frame() {
        assert_eq!(atlas_uv_rect(1, 4, 2), Vec4::new(0.25, 0.0, 0.25, 0.5));
        assert_eq!(atlas_uv_rect(2, 4, 2), Vec4::new(0.5, 0.0, 0.25, 0.5));
        assert_eq!(atlas_uv_rect(3, 4, 2), Vec4::new(0.75, 0.0, 0.25, 0.5));
    }
}
