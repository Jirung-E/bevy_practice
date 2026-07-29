# 08. Query 과제 해설

[본문으로 돌아가기](../../08_Query.md#실습-과제)

## P1-C08-P1 · 체력 5인 세 번째 적

### 힌트

기존 두 적과 같은 Component 조합으로 생성합니다. `5_u32.saturating_sub(10)`의 결과는 0입니다.

### 확인 기준

- 세 번째 적만 특별 취급하는 분기를 만들지 않는다.
- 같은 `damage_enemies` System이 세 적을 모두 처리한다.
- 체력이 정수 범위를 넘어가지 않고 0에서 멈춘다.

## P1-C08-P2 · Without 제거 비교

`With<Enemy>`만으로도 `Enemy`가 없는 Player는 선택되지 않으므로 현재 결과는 같습니다. `Without<Player>`는 두 marker가 실수로 동시에 붙은 Entity까지 제외한다는 추가 제약입니다. 단순히 결과가 같다는 사실보다 데이터 불변식을 어디에서 보장할지 설명하는 것이 핵심입니다.

## P1-C08-P3 · Name으로 출력

### 수행 예시

```rust
fn damage_enemies(mut enemies: Query<(&Name, &mut Health), With<Enemy>>) {
    for (name, mut health) in &mut enemies {
        health.0 = health.0.saturating_sub(10);
        println!("{}의 남은 체력: {}", name.0, health.0);
    }
}
```

Entity ID는 디버깅에 유용하지만 사용자에게 보여 줄 이름은 별도 Component로 관리합니다.

## P1-C08-A1 · Changed 필터

### 접근 방법

1. 체력을 변경하는 System을 먼저 실행합니다.
2. `Changed<Health>`가 있는 Query로 변경된 적만 읽습니다.
3. 두 System을 `chain()`으로 연결해 같은 프레임의 변경을 관찰합니다.

```rust
fn report_changed(
    enemies: Query<(&Name, &Health), (With<Enemy>, Changed<Health>)>,
) {
    for (name, health) in &enemies {
        println!("변경됨: {} = {}", name.0, health.0);
    }
}
```

`Changed<T>`는 이전 값과 현재 값을 비교하지 않습니다. `&mut T`를 역참조해 쓰기 접근한 사실을 추적하므로 같은 값을 다시 대입해도 변경으로 표시될 수 있습니다. 값이 실제로 달라질 때만 표시해야 한다면 쓰기 전에 비교해야 합니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin query_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/query_solution.rs`

