# 12D. ECS 동작 추상화와 스킬 시스템 과제 해설

[본문으로 돌아가기](../../12D_BehaviorAbstraction.md#실습-과제)

## P1-C12D-P1 · 스킬 데이터 변경

### 확인 기준

- 검증 System과 효과 System의 코드는 바꾸지 않는다.
- `SkillCost`와 `DamageEffect` 값만 바꾼다.
- 예상한 Energy와 Health가 테스트 결과와 일치한다.

스킬 수치가 Component 데이터로 분리되어 있다면 밸런스 변경 때문에 실행 로직을 수정할 필요가 없습니다.

## P1-C12D-P2 · 회복 효과 추가

### 힌트

`DamageEffect`와 같은 형태의 `HealingEffect` Component를 만들고 `SkillApproved`를 읽는 별도 System을 작성합니다. 스킬 Entity에 그 Component가 없으면 `get(request.skill)`이 실패하므로 해당 요청은 건너뜁니다.

### 확인 기준

- 기존 `apply_damage`에 회복 분기를 추가하지 않는다.
- 회복량은 `HealingEffect` 데이터에 있다.
- 비용과 쿨다운 검증은 기존 `validate_skill`을 재사용한다.
- 최대 체력이 있다면 회복 결과가 그 값을 넘지 않는다.

## P1-C12D-P3 · 잘못된 소유자 요청

### 접근 방법

1. `UseSkill.caster`에 전사 Entity를 넣고 `skill`에는 마법사의 파이어볼 Entity를 넣습니다.
2. 한 프레임을 실행합니다.
3. `SkillOwner` 검사에서 거부되어 전사의 Energy, 대상 Health, 스킬 Cooldown이 그대로인지 확인합니다.

이 테스트는 입력 UI가 잘못된 슬롯을 가리키거나 네트워크 요청이 위조되어도 검증 계층이 실행을 차단하는지 확인합니다.

## P1-C12D-A1 · 쿨다운과 스킬 슬롯

### 설계 순서

1. `SkillSlots(Vec<Entity>)`를 캐릭터 Component로 둡니다.
2. 입력 System은 숫자 키에 해당하는 슬롯의 Entity ID만 선택합니다.
3. `tick_cooldowns`는 `Time`과 `Query<&mut Cooldown>`만 사용합니다.
4. 입력은 구체적인 효과 타입을 모르고 `UseSkill`만 보냅니다.
5. 쿨다운 중 요청, 만료 직전 요청, 만료 후 요청을 테스트합니다.

실제 시간에 의존하는 테스트는 불안정할 수 있으므로 쿨다운 감소 계산을 순수 함수로 분리하거나 테스트에서 고정된 delta를 전달하는 구조가 좋습니다.
