# 12D. ECS 동작 추상화와 스킬 시스템

## 학습 목표

- 객체지향의 가상 메서드 호출과 ECS의 Component/System 디스패치를 비교할 수 있다.
- 스킬을 별도 Entity로 만들고 소유자·비용·쿨다운·효과를 조합할 수 있다.
- Message를 사용해 입력과 스킬 실행을 분리할 수 있다.
- 공통 검증과 효과별 System을 나누어 새로운 스킬을 확장할 수 있다.
- enum, Component 조합, trait object의 적용 범위와 trade-off를 판단할 수 있다.

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

### ECS의 동작 추상화

객체지향에서는 기반 인터페이스의 메서드를 호출하고 실제 객체 타입에 따라 구현을 선택합니다.

```rust,ignore
trait SkillBehavior {
    fn use_skill(&mut self, character: &mut Character);
}
```

ECS에서는 “어떤 객체의 가상 메서드를 호출할 것인가”보다 “어떤 Component 조합을 어떤 System이 처리할 것인가”를 결정합니다.

| 객체지향 | Bevy ECS |
|---|---|
| `SkillBehavior` 인터페이스 | 스킬 처리에 필요한 Component 계약 |
| 캐릭터별 `use_skill()` 구현 | 효과 Component를 처리하는 System |
| 가상 메서드 디스패치 | Query 조건에 따른 System 디스패치 |
| 클래스의 스킬 필드 | 별도 스킬 Entity와 데이터 Component |
| 메서드 호출 | Message 또는 Event로 실행 요청 |
| 기반 클래스의 공통 검증 | 비용·쿨다운을 처리하는 공통 System |
| 새로운 파생 클래스 | 새로운 효과 Component와 System 또는 Plugin |

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

이것이 ECS의 다형적 동작 선택입니다. `DashEffect`가 없는 파이어볼 요청은 돌진 System이 건너뛰고, `DamageEffect`가 없는 돌진 요청은 피해 System이 건너뜁니다.

### enum, Component 조합, trait object 중 선택하기

| 상황 | 권장 방식 | 이유 |
|---|---|---|
| 종류가 적고 고정된 메뉴 동작 | `SkillKind` enum과 `match` | 전체 경우를 한곳에서 파악하기 쉽다 |
| 독립된 능력을 코드로 확장 | 효과 Component와 System | 기존 스킬 코드를 수정하지 않고 추가할 수 있다 |
| 에디터에서 조합·저장할 스킬 | 스킬 Entity와 효과 Component | 데이터와 실행 대상을 분리할 수 있다 |
| 순수 계산 전략 교체 | Rust trait 또는 generic | World 접근 없이 테스트하기 쉽다 |
| 런타임 모드·사용자 스크립트 | 스크립팅 계층과 ECS 명령 연결 | 실행 중 새 동작을 로드할 수 있다 |

`Box<dyn SkillBehavior>`를 Component에 넣는 것도 가능하지만 기본 선택으로 삼지는 않습니다. Bevy는 특정 trait 구현체를 자동으로 Query하지 않으며, trait object 내부 값은 Reflect·Scene 저장·복제·병렬 데이터 접근에 추가 설계가 필요합니다. ECS 바깥의 순수 전략에는 trait을 사용하고, World 안의 실행 계약에는 Component와 System을 우선하는 편이 자연스럽습니다.

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
