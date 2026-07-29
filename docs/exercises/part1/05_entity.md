# 05. Entity 과제 해설

[본문으로 돌아가기](../../05_Entity.md#실습-과제)

먼저 본문의 과제를 직접 수행한 뒤 아래 힌트와 예시를 확인하세요.

## P1-C05-P1 · 세 번째 Entity

### 힌트

플레이어와 적을 만든 방식 그대로 `spawn_empty()`를 한 번 더 호출하고 반환된 ID를 보관합니다.

### 확인 기준

- player, enemy, npc 세 ID를 모두 출력한다.
- 같은 실행에서 세 ID가 서로 다르다.

### 수행 예시

```rust
let player = world.spawn_empty().id();
let enemy = world.spawn_empty().id();
let npc = world.spawn_empty().id();

println!("{player:?}, {enemy:?}, {npc:?}");
```

## P1-C05-P2 · ID 비교

### 힌트

`Entity`는 `PartialEq`와 `Eq`를 구현하므로 `==`와 `!=`로 비교할 수 있습니다.

### 확인 기준

- 생성 순서나 ID 숫자의 크기가 아니라 동등성으로 비교한다.
- 세 쌍을 모두 비교한다.

### 수행 예시

```rust
let all_distinct = player != enemy && player != npc && enemy != npc;
println!("모든 ID가 서로 다름: {all_distinct}");
```

## P1-C05-P3 · 생성 순서와 변수 이름

### 확인 방법

적을 먼저 생성하고 플레이어를 나중에 생성하도록 두 줄의 순서를 바꿔 실행합니다.

```rust
let enemy = commands.spawn_empty().id();
let player = commands.spawn_empty().id();

println!("플레이어 Entity: {player:?}");
println!("적 Entity: {enemy:?}");
```

`player`라는 변수 이름은 Rust 코드에서 그 ID를 부르는 이름일 뿐입니다. 아직 Component가 없는 두 Entity는 World 안에서 구조적으로 동일합니다.

## P1-C05-A1 · 여러 Entity ID 보관

### 접근 방법

`Vec<Entity>`를 만들고 반복문에서 생성한 ID를 넣습니다.

```rust
let mut entities = Vec::new();

for _ in 0..5 {
    entities.push(commands.spawn_empty().id());
}

for entity in &entities {
    println!("{entity:?}");
}
```

각 ID는 현재 World 안의 서로 다른 대상을 가리킵니다. 숫자의 크기나 연속성을 게임 규칙에 사용하지 않습니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin entity_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/entity_solution.rs`
