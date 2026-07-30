# 08. Query: 원하는 데이터 찾기

## 학습 목표

- Query의 데이터 항목과 Filter를 구분할 수 있다.
- `With`, `Without`, `Entity`를 조합할 수 있다.
- 가변 Query가 만드는 데이터 접근 규칙을 이해한다.

## 이 내용으로 만들 수 있는 것

- 모든 적만 찾아 피해를 주거나 플레이어를 제외한 Collider만 검사할 수 있습니다.
- `With`, `Without`, `Changed` filter로 필요한 Entity만 처리해 로직과 비용을 줄일 수 있습니다.

## 이번에 만들 결과물

Player 한 명과 Enemy 두 명을 만든 뒤, Enemy의 체력만 10씩 감소시키고 각 Entity ID와 남은 체력을 출력합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin query
```

## 핵심 개념

`Query<D, F>`는 조건에 맞는 Entity의 Component를 빌립니다.

- `D`: System이 실제로 받을 데이터
- `F`: 어떤 Entity를 포함하거나 제외할지 정하는 Filter

`&Health`는 읽기, `&mut Health`는 쓰기 접근입니다. 같은 System에서 동일 Component를 겹치게 빌리면 Rust의 별칭 규칙을 위반할 수 있으므로 `Without` 또는 `ParamSet`으로 집합이 분리된다는 사실을 알려야 합니다.

## 샘플 코드

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, Debug)]
struct Health(u32);

fn damage_enemies(
    mut enemies: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>,
) {
    for (entity, mut health) in &mut enemies {
        health.0 = health.0.saturating_sub(10);
        println!("{entity:?}의 남은 체력: {}", health.0);
    }
}
```

`setup`에서는 `(Player, Health(100))` 하나와 `(Enemy, Health(...))` 둘을 생성합니다.

## 코드 설명

- `(Entity, &mut Health)`는 ID와 수정 가능한 체력을 함께 가져옵니다.
- `With<Enemy>`는 Enemy가 있는 Entity만 포함합니다.
- `Without<Player>`는 Player가 있는 Entity를 제외합니다.
- 필터 튜플은 모든 조건을 동시에 만족해야 합니다.
- `saturating_sub(10)`은 0 아래로 내려갈 때 정수 오버플로 대신 0을 반환합니다.

자주 쓰는 Query 메서드는 `iter`, `iter_mut`, `single`, `get`, `get_mut`입니다. 대상 수를 확신할 수 없다면 `single()`의 오류를 처리하거나 반복을 사용하세요.

## 실습 과제

1. 세 번째 Enemy를 체력 5로 생성하고 결과가 0인지 확인하세요.
2. `Without<Player>`를 제거하고 결과가 같은지 확인한 뒤, 왜 여전히 Player가 선택되지 않는지 설명하세요.
3. `Name` Component를 추가해 ID 대신 이름을 출력하세요.

## 심화 과제

`Changed<Health>` 필터를 사용하는 두 번째 System을 작성해 이번 프레임에 체력이 바뀐 대상만 출력하세요. 변경 감지가 값의 비교가 아니라 변경 접근을 추적한다는 점도 실험해 보세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/08_query.md)를 확인하세요.

## 다음 챕터

Entity에 속하지 않는 점수와 게임 규칙을 Resource로 관리합니다.
