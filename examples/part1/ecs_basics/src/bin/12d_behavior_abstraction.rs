use bevy::prelude::*;

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Mage;

#[derive(Component)]
struct Warrior;

#[derive(Component, Debug)]
struct Health(u32);

#[derive(Component, Debug)]
struct Position(f32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnergyKind {
    Mana,
    Stamina,
}

#[derive(Component, Debug)]
struct Energy {
    kind: EnergyKind,
    current: u32,
}

#[derive(Component)]
struct Skill;

#[derive(Component)]
struct SkillOwner(Entity);

#[derive(Component)]
struct SkillCost {
    kind: EnergyKind,
    amount: u32,
}

#[derive(Component)]
struct Cooldown {
    duration: f32,
    remaining: f32,
}

#[derive(Component)]
struct ProjectileEffect {
    speed: f32,
}

#[derive(Component)]
struct DamageEffect {
    amount: u32,
}

#[derive(Component)]
struct DashEffect {
    distance: f32,
}

#[derive(Resource)]
struct DemoSkills {
    mage: Entity,
    warrior: Entity,
    target: Entity,
    fireball: Entity,
    dash: Entity,
}

#[derive(Message, Clone, Copy)]
struct UseSkill {
    caster: Entity,
    skill: Entity,
    target: Option<Entity>,
}

#[derive(Message, Clone, Copy)]
struct SkillApproved(UseSkill);

fn main() {
    let mut app = build_app();
    app.update();
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_message::<UseSkill>()
        .add_message::<SkillApproved>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                request_demo_skills,
                validate_skill,
                spawn_projectile,
                apply_damage,
                apply_dash,
                report_result,
            )
                .chain(),
        );
    app
}

fn setup(mut commands: Commands) {
    let mage = commands
        .spawn((
            Character,
            Mage,
            Energy {
                kind: EnergyKind::Mana,
                current: 100,
            },
            Position(0.0),
        ))
        .id();
    let warrior = commands
        .spawn((
            Character,
            Warrior,
            Energy {
                kind: EnergyKind::Stamina,
                current: 100,
            },
            Position(0.0),
        ))
        .id();
    let target = commands.spawn((Health(100), Position(12.0))).id();

    let fireball = commands
        .spawn((
            Skill,
            SkillOwner(mage),
            SkillCost {
                kind: EnergyKind::Mana,
                amount: 20,
            },
            Cooldown {
                duration: 2.0,
                remaining: 0.0,
            },
            ProjectileEffect { speed: 12.0 },
            DamageEffect { amount: 30 },
        ))
        .id();
    let dash = commands
        .spawn((
            Skill,
            SkillOwner(warrior),
            SkillCost {
                kind: EnergyKind::Stamina,
                amount: 15,
            },
            Cooldown {
                duration: 1.5,
                remaining: 0.0,
            },
            DashEffect { distance: 5.0 },
        ))
        .id();

    commands.insert_resource(DemoSkills {
        mage,
        warrior,
        target,
        fireball,
        dash,
    });
}

fn request_demo_skills(skills: Res<DemoSkills>, mut requests: MessageWriter<UseSkill>) {
    requests.write(UseSkill {
        caster: skills.mage,
        skill: skills.fireball,
        target: Some(skills.target),
    });
    requests.write(UseSkill {
        caster: skills.warrior,
        skill: skills.dash,
        target: None,
    });
}

fn validate_skill(
    mut requests: MessageReader<UseSkill>,
    mut approved: MessageWriter<SkillApproved>,
    mut skills: Query<(&SkillOwner, &SkillCost, &mut Cooldown), With<Skill>>,
    mut characters: Query<&mut Energy, With<Character>>,
) {
    for request in requests.read() {
        let Ok((owner, cost, mut cooldown)) = skills.get_mut(request.skill) else {
            continue;
        };
        if owner.0 != request.caster || cooldown.remaining > 0.0 {
            continue;
        }

        let Ok(mut energy) = characters.get_mut(request.caster) else {
            continue;
        };
        if energy.kind != cost.kind || energy.current < cost.amount {
            continue;
        }

        energy.current -= cost.amount;
        cooldown.remaining = cooldown.duration;
        approved.write(SkillApproved(*request));
    }
}

fn spawn_projectile(
    mut approved: MessageReader<SkillApproved>,
    projectiles: Query<&ProjectileEffect>,
) {
    for SkillApproved(request) in approved.read() {
        let Ok(projectile) = projectiles.get(request.skill) else {
            continue;
        };
        println!("투사체 생성: 속도 {}", projectile.speed);
    }
}

fn apply_damage(
    mut approved: MessageReader<SkillApproved>,
    damage_effects: Query<&DamageEffect>,
    mut healths: Query<&mut Health>,
) {
    for SkillApproved(request) in approved.read() {
        let (Ok(effect), Some(target)) = (damage_effects.get(request.skill), request.target) else {
            continue;
        };
        let Ok(mut health) = healths.get_mut(target) else {
            continue;
        };
        health.0 = health.0.saturating_sub(effect.amount);
    }
}

fn apply_dash(
    mut approved: MessageReader<SkillApproved>,
    dash_effects: Query<&DashEffect>,
    mut positions: Query<&mut Position, With<Character>>,
) {
    for SkillApproved(request) in approved.read() {
        let Ok(effect) = dash_effects.get(request.skill) else {
            continue;
        };
        let Ok(mut position) = positions.get_mut(request.caster) else {
            continue;
        };
        position.0 += effect.distance;
    }
}

fn report_result(
    demo: Res<DemoSkills>,
    energies: Query<&Energy>,
    positions: Query<&Position>,
    healths: Query<&Health>,
) {
    let mage_energy = energies.get(demo.mage).unwrap();
    let warrior_energy = energies.get(demo.warrior).unwrap();
    let warrior_position = positions.get(demo.warrior).unwrap();
    let target_health = healths.get(demo.target).unwrap();

    println!("마법사 마나: {}", mage_energy.current);
    println!("전사 스태미나: {}", warrior_energy.current);
    println!("전사 위치: {}", warrior_position.0);
    println!("훈련 대상 체력: {}", target_health.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_composition_dispatches_different_skill_effects() {
        let mut app = build_app();
        app.update();

        let demo = app.world().resource::<DemoSkills>();
        assert_eq!(app.world().get::<Energy>(demo.mage).unwrap().current, 80);
        assert_eq!(app.world().get::<Energy>(demo.warrior).unwrap().current, 85);
        assert_eq!(app.world().get::<Position>(demo.warrior).unwrap().0, 5.0);
        assert_eq!(app.world().get::<Health>(demo.target).unwrap().0, 70);
    }

    #[test]
    fn cooldown_rejects_repeated_requests() {
        let mut app = build_app();
        app.update();
        app.update();

        let demo = app.world().resource::<DemoSkills>();
        assert_eq!(app.world().get::<Energy>(demo.mage).unwrap().current, 80);
        assert_eq!(app.world().get::<Health>(demo.target).unwrap().0, 70);
    }
}
