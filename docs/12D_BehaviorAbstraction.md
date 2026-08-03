# 12D. ECS 동작 추상화와 스킬 시스템

## 학습 목표

- Generic 정적 다형성, trait object 동적 다형성, ECS 조합 기반 디스패치, 스크립트 런타임 확장을 구분할 수 있다.
- 스킬을 별도 Entity로 만들고 소유자·비용·쿨다운·효과를 조합할 수 있다.
- Message를 사용해 입력과 스킬 실행을 분리할 수 있다.
- 공통 검증과 효과별 System을 나누어 새로운 스킬을 확장할 수 있다.
- generic, trait object, Component 조합, 스크립트의 적용 범위와 trade-off를 판단할 수 있다.

## 이 내용으로 만들 수 있는 것

- 캐릭터마다 서로 다른 스킬을 사용하면서 비용·쿨다운 검증은 공유하는 전투 시스템
- 투사체·피해·상태 이상·이동 효과를 데이터 조합으로 만드는 스킬 에디터
- 캐릭터 클래스 수정 없이 Plugin 단위로 추가할 수 있는 능력과 아이템 효과

## 이번에 만들 결과물

마법사는 파이어볼을 사용해 마나를 소비하고 훈련 대상에게 피해를 줍니다. 전사는 돌진을 사용해 스태미나를 소비하고 위치를 이동합니다. 두 스킬은 같은 요청·검증·쿨다운 파이프라인을 지나지만, 스킬 Entity에 붙은 효과 Component에 따라 서로 다른 System이 실행됩니다.

아래 명령은 저장소의 완성 샘플을 실행합니다.

```bash
cargo run -p ecs_basics --bin behavior_abstraction
```

출력은 다음과 같습니다.

```text
투사체 생성: 속도 12
마법사 마나: 80
전사 스태미나: 85
전사 위치: 5
훈련 대상 체력: 70
```

## 핵심 개념

### Rust와 Bevy에서 동작을 추상화하는 네 가지 방법

객체지향에서는 기반 인터페이스의 메서드를 호출하고 실제 객체 타입에 따라 구현을 선택합니다.

```rust,ignore
trait SkillBehavior {
    fn use_skill(&mut self, character: &mut Character);
}
```

Rust와 Bevy에서는 이 문제를 크게 네 가지 방식으로 풀 수 있습니다. 네 방식은 서로를 완전히 대체하지 않으며, 같은 프로젝트 안에서도 용도에 따라 함께 사용합니다.

| 방식 | 선택 시점 | 적합한 용도 | 주요 제약 |
|---|---|---|---|
| Generic `<S: SkillBehavior>` | 컴파일 타임 | 재사용 가능한 알고리즘, 순수 계산 | 호출할 때 구체 타입을 알아야 한다 |
| `Box<dyn SkillBehavior>` | 런타임 | 서로 다른 구현을 같은 인터페이스로 보관·호출 | Reflect·Scene 저장·ECS 병렬 접근에 추가 설계가 필요하다 |
| Component + System | ECS 스케줄 실행 시 | World의 많은 Entity를 Query로 일괄 처리하고 데이터로 조합 | 새로운 동작 범주에는 처리 System이 필요하다 |
| 스크립트 | 실행 중 스크립트 해석 시 | 재컴파일 없이 스킬 동작 추가·교체·핫 리로드 | 성능·디버깅·API 노출·안전 경계를 따로 설계해야 한다 |

#### Generic: 정적 다형성

Generic은 같은 알고리즘을 여러 구체 타입에 재사용합니다. 컴파일러가 타입별 코드를 만들 수 있어 인라인과 최적화에 유리하지만, 실행 중 서로 다른 스킬을 하나의 목록에서 고르는 용도에는 맞지 않습니다.

```rust,ignore
fn execute<S: SkillBehavior>(skill: &mut S, character: &mut Character) {
    skill.use_skill(character);
}
```

#### trait object: 동적 다형성

`dyn SkillBehavior`는 객체지향의 가상 메서드 호출과 가장 직접적으로 대응합니다. 호출자는 실제 타입이 `Fireball`인지 `Dash`인지 몰라도 같은 메서드로 실행할 수 있습니다.

```rust,ignore
let mut skills: Vec<Box<dyn SkillBehavior>> = vec![
    Box::new(Fireball),
    Box::new(Dash),
];

skills[selected].use_skill(&mut character);
```

이 방식은 다형성을 포기하지 않습니다. 다만 trait object 내부 데이터는 Bevy가 자동으로 Query하거나 Reflect하지 않으므로, 스킬을 Scene/RON에 저장하거나 수많은 Entity를 병렬 처리하려면 별도의 등록·직렬화 구조가 필요합니다.

#### Component + System: Bevy World 안의 정석적인 구성

Bevy 게임플레이 데이터에는 보통 “어떤 객체의 가상 메서드를 호출할 것인가”보다 “어떤 Component 조합을 어떤 System이 처리할 것인가”를 사용합니다. 이것은 subtype polymorphism이 아니라 **조합 기반 디스패치**입니다. 다형성의 다른 이름이 아니라, ECS의 데이터 배치·Query·병렬 실행을 활용하기 위해 선택하는 별도의 설계 방식입니다.

| 객체지향 | Bevy ECS |
|---|---|
| `SkillBehavior` 인터페이스 | 스킬 처리에 필요한 Component 계약 |
| 캐릭터별 `use_skill()` 구현 | 효과 Component를 처리하는 System |
| 가상 메서드 디스패치 | Query 조건에 따른 조합 기반 디스패치 |
| 클래스의 스킬 필드 | 별도 스킬 Entity와 데이터 Component |
| 메서드 호출 | Message 또는 Event로 실행 요청 |
| 기반 클래스의 공통 검증 | 비용·쿨다운을 처리하는 공통 System |
| 새로운 파생 클래스 | 새로운 효과 Component와 System 또는 Plugin |

#### 스크립트: 재컴파일 없는 런타임 확장

스크립트는 Rust의 정적·동적 다형성과는 구분해야 합니다. 스크립트 런타임이 공통 콜백을 찾아 실행하는 구조이며, 스킬별 동작을 Rust 컴파일 영역 밖으로 옮깁니다. 스킬 Entity에는 코드 자체보다 실행할 스크립트 Asset의 Handle을 둡니다.

```rust,ignore
#[derive(Component)]
struct ScriptSkill {
    script: Handle<SkillScript>,
}
```

실행 흐름은 다음과 같이 구성할 수 있습니다.

```text
UseSkill 요청
  → Rust System에서 소유자·비용·쿨다운 검증
  → ScriptSkill의 Asset 확인
  → 스크립트의 use_skill(context) 콜백 실행
  → 스크립트에 허용된 피해·이동·투사체 명령 발행
  → Rust System이 실제 World 변경
```

이 구조에서는 새 스킬을 스크립트 파일로 추가하고 실행 중 다시 불러올 수 있습니다. 그러나 스크립트가 사용할 수 있는 기능은 Rust가 노출한 API로 제한됩니다. `damage`, `move_character`, `spawn_projectile`은 조합할 수 있어도 `stop_time` API가 없다면 시간 정지 기능과 바인딩은 Rust에서 먼저 구현해야 합니다. 새로운 동작의 복잡성이 사라지는 것이 아니라, 일반적인 스킬 조합 코드를 재빌드 가능한 Rust 영역에서 핫 리로드 가능한 스크립트 영역으로 옮기는 것입니다.

실전에서는 ECS가 체력·위치·쿨다운 같은 상태를 소유하고, Rust System이 안전한 기본 동작을 제공하며, 스크립트는 그 동작의 순서와 조건을 조합하는 혼합 구조가 일반적입니다. 스크립트가 `World`를 무제한으로 직접 수정하게 하기보다 제한된 Command나 Message만 발행하게 만들면 검증·저장·리플레이도 관리하기 쉽습니다.

이 챕터에서는 스크립트 방식의 선택 기준만 다룹니다. 사용자 정의 Asset으로 스크립트를 연결하고 변경 내용을 다시 불러오는 전체 과정은 [40B. Entity 스크립트 연결과 실행](40B_EntityScripts.md)에서 실습합니다.

### 스킬을 별도 Entity로 만들기

캐릭터에 모든 스킬 데이터를 직접 넣으면 스킬 교체·아이템 부여·에디터 저장이 어려워집니다. 샘플은 캐릭터와 스킬을 별도 Entity로 분리합니다.

```text
Mage Entity                  Fireball Skill Entity
├─ Character                ├─ Skill
├─ Mage                     ├─ SkillOwner(Mage)
├─ Energy(Mana, 100)        ├─ SkillCost(Mana, 20)
└─ Position                 ├─ Cooldown(2.0)
                            ├─ ProjectileEffect(12.0)
                            └─ DamageEffect(30)

Warrior Entity               Dash Skill Entity
├─ Character                ├─ Skill
├─ Warrior                  ├─ SkillOwner(Warrior)
├─ Energy(Stamina, 100)     ├─ SkillCost(Stamina, 15)
└─ Position                 ├─ Cooldown(1.5)
                            └─ DashEffect(5.0)
```

파이어볼은 “파이어볼 클래스”가 아니라 투사체와 피해 효과의 조합입니다. 같은 `DamageEffect`를 근접 공격이나 폭발에도 붙일 수 있고, `BurnEffect` 같은 Component를 추가해 기존 피해 코드를 복사하지 않고 기능을 확장할 수 있습니다.

### 요청·검증·효과 실행 파이프라인

입력 System이 구체적인 스킬 구현을 직접 호출하지 않고 요청만 보냅니다.

```rust
#[derive(Message, Clone, Copy)]
struct UseSkill {
    caster: Entity,
    skill: Entity,
    target: Option<Entity>,
}
```

실행 흐름은 다음과 같습니다.

```text
UseSkill 요청
  → SkillOwner 검사
  → Energy 종류와 비용 검사
  → Cooldown 검사
  → 비용 차감과 쿨다운 시작
  → SkillApproved 발행
  → ProjectileEffect / DamageEffect / DashEffect System 실행
```

공통 검증 System은 스킬의 구체적인 효과를 알 필요가 없습니다. 효과 System도 비용 정책을 알 필요가 없습니다. 두 영역은 `SkillApproved` Message를 계약으로 연결합니다.

### 효과별 System이 동작을 선택한다

각 효과 System은 승인된 요청을 모두 읽지만 자신의 Component가 붙은 스킬만 처리합니다.

```rust
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
```

이것이 ECS의 조합 기반 동작 선택입니다. `DashEffect`가 없는 파이어볼 요청은 돌진 System이 건너뛰고, `DamageEffect`가 없는 돌진 요청은 피해 System이 건너뜁니다. 이는 `dyn Trait`의 동적 다형성과는 다른 메커니즘입니다.

### 스킬이 늘어날 때마다 System도 늘어나는가?

수치와 조합만 다른 스킬이라면 늘어나지 않지만, **기존 효과로 표현할 수 없는 새로운 동작 범주가 추가되면 System도 늘어납니다.** 복잡성 자체가 사라지는 것은 아닙니다. 이 방식은 System을 스킬 이름이 아니라 재사용 가능한 효과 종류를 기준으로 만들어 증가 속도를 줄입니다. `fireball()`, `ice_bolt()`, `poison_arrow()`처럼 스킬마다 함수를 만드는 대신 `apply_damage`, `spawn_projectile`, `apply_dash`처럼 여러 스킬이 공유할 작은 실행 단위를 만듭니다.

예를 들어 다음 스킬들은 새로운 System 없이 기존 효과 Component의 조합과 수치만 바꾸어 만들 수 있습니다.

| 스킬 | 조합하는 효과 Component |
|---|---|
| 파이어볼 | `ProjectileEffect` + `DamageEffect` |
| 강한 파이어볼 | `ProjectileEffect` + 더 큰 `DamageEffect` |
| 독화살 | `ProjectileEffect` + `DamageEffect` + `StatusEffect(Poison)` |
| 근접 강타 | `DamageEffect` + `KnockbackEffect` |
| 회피 돌진 | `DashEffect` + `InvincibleEffect` |

따라서 스킬 100개가 있다고 반드시 System도 100개가 생기지는 않습니다. 게임에 존재하는 **효과의 문법**이 10종류라면 대체로 10종류 안팎의 효과 System을 조합해 많은 스킬을 표현합니다. 그러나 100개가 모두 서로 다른 고유 동작이라면 그에 대응하는 실행 코드도 필요합니다. 효과별 System, 중앙 인터프리터의 분기, 외부 스크립트 중 어디에 둘지가 달라질 뿐입니다.

```text
스킬 수 증가
  → 기존 효과 Component의 조합과 수치 데이터 추가

새로운 효과 범주 증가
  → 해당 효과 Component와 처리 System 추가
```

### 효과 종류 자체가 많아지면 어떻게 나누는가?

효과가 많아져 한 파일의 System 목록이 길어지면 기능 단위 Plugin으로 나눕니다. 예를 들어 `ProjectilePlugin`, `DamagePlugin`, `MovementSkillPlugin`, `StatusEffectPlugin`이 각자의 Component와 System을 등록하게 만들 수 있습니다. 스킬 실행 파이프라인은 여전히 `SkillApproved`만 발행하므로 다른 효과 모듈의 내부 구현을 알 필요가 없습니다.

비슷한 효과가 이름만 달라 계속 늘어난다면 새 Component를 만들기 전에 데이터로 일반화할 수 있는지 확인합니다. `FireDamage`, `IceDamage`, `PoisonDamage`를 각각 만들기보다 다음처럼 하나의 데이터 모델로 합칠 수 있습니다.

```rust,ignore
#[derive(Clone, Copy)]
enum DamageKind {
    Physical,
    Fire,
    Ice,
    Poison,
}

#[derive(Component)]
struct DamageEffect {
    kind: DamageKind,
    amount: u32,
}
```

이 경우 `apply_damage` 하나가 공통 피해 계산을 담당하고, 속성 저항이나 상태 이상처럼 정말 다른 규칙만 별도 System으로 분리합니다. 반대로 모든 효과를 거대한 `SkillKind` enum 하나와 `match` 하나에 몰아넣으면 새 효과를 추가할 때 중앙 함수를 계속 수정해야 하므로, 조합 가능성이 큰 게임에서는 다시 결합도가 높아집니다.

정리하면 확장 순서는 다음과 같습니다.

1. 같은 동작이고 수치만 다르면 Component 데이터만 추가합니다.
2. 여러 스킬이 공유할 새 동작이면 효과 Component와 System을 한 쌍 추가합니다.
3. 관련 효과가 많아지면 기능 단위 Plugin으로 묶습니다.
4. 완전히 임의적인 사용자 스크립트가 필요할 때만 스크립팅 계층을 연결합니다.

### 네 방식 중 선택하기

| 상황 | 권장 방식 | 이유 |
|---|---|---|
| 종류가 적고 고정된 메뉴 동작 | `SkillKind` enum과 `match` | 전체 경우를 한곳에서 파악하기 쉽다 |
| 독립된 능력을 코드로 확장 | 효과 Component와 System | 기존 스킬 코드를 수정하지 않고 추가할 수 있다 |
| 에디터에서 조합·저장할 스킬 | 스킬 Entity와 효과 Component | 데이터와 실행 대상을 분리할 수 있다 |
| 타입이 컴파일 타임에 정해지는 순수 계산 | generic | 정적 디스패치와 인라인에 유리하다 |
| 실행 중 서로 다른 구현을 같은 인터페이스로 호출 | `dyn Trait` | subtype polymorphism을 직접 표현한다 |
| 재컴파일 없이 스킬별 실행 순서와 조건 추가 | 스크립트 | 런타임 로드와 핫 리로드가 가능하다 |

`Box<dyn SkillBehavior>`를 Component에 넣는 것도 가능하지만 Bevy World 안의 기본 선택으로 삼지는 않습니다. Bevy는 특정 trait 구현체를 자동으로 Query하지 않으며, trait object 내부 값은 Reflect·Scene 저장·복제·병렬 데이터 접근에 추가 설계가 필요합니다. 타입이 컴파일 타임에 정해진 순수 알고리즘에는 generic, 런타임 구현 교체 자체가 요구사항이면 `dyn Trait`, Entity 데이터의 조합·검색·병렬 실행이 중요하면 Component와 System, 재컴파일 없는 콘텐츠 확장이 중요하면 제한된 ECS API와 연결한 스크립트를 선택합니다.

## 샘플 코드

전체 코드: `examples/part1/ecs_basics/src/bin/12d_behavior_abstraction.rs`

스킬 Entity는 공통 정책과 구체 효과를 함께 조합합니다.

```rust
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
```

공통 검증을 통과한 요청만 효과 System에 전달합니다.

```rust
if energy.kind != cost.kind || energy.current < cost.amount {
    continue;
}

energy.current -= cost.amount;
cooldown.remaining = cooldown.duration;
approved.write(SkillApproved(*request));
```

## 코드 설명

- `SkillOwner`는 어떤 캐릭터가 그 스킬을 사용할 수 있는지 검증합니다.
- `SkillCost`와 `Energy`는 마나와 스태미나를 같은 검증 System으로 처리합니다.
- `Cooldown`은 모든 스킬이 공유하는 실행 정책입니다.
- `UseSkill`은 입력과 AI가 같은 실행 파이프라인을 사용할 수 있게 합니다.
- `SkillApproved`는 검증과 구체 효과 사이의 계약입니다.
- `ProjectileEffect`, `DamageEffect`, `DashEffect`는 데이터이며 각각의 System이 동작을 제공합니다.
- Message는 Reader마다 독립적으로 전달되므로 여러 효과 System이 같은 승인 요청을 처리할 수 있습니다.
- 두 번째 업데이트에서도 사용 요청은 생기지만 쿨다운이 남아 있어 비용과 효과가 다시 적용되지 않습니다.

## 실습 과제

1. 파이어볼의 피해량과 비용을 바꾸고 실행 결과를 먼저 예상한 뒤 확인하세요.
2. 체력을 회복하는 새 효과를 설계해 스킬 Entity에 붙이고, 그 효과가 있는 승인 요청만 처리하는 System을 추가하세요.
3. 소유자가 아닌 캐릭터가 스킬을 요청했을 때 Energy와 대상 상태가 변하지 않는 테스트를 작성하세요.

## 심화 과제

시간에 따라 `Cooldown.remaining`을 감소시키는 공통 System과 캐릭터별 `SkillSlots` Component를 추가하세요. 입력 번호가 스킬 구현을 직접 알지 않고 슬롯의 Entity ID만 `UseSkill`에 넣도록 구성하고, 쿨다운 종료 전과 종료 후의 재사용 결과를 테스트하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/12d_behavior_abstraction.md)를 확인하세요.

## 다음 챕터

[12E. 입력 Action과 장치 독립적인 명령](12E_InputActions.md)에서 키보드와 게임패드를 gameplay 명령으로 변환합니다.
