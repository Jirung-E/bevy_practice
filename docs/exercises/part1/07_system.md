# 07. System 과제 해설

[본문으로 돌아가기](../../07_System.md#실습-과제)

아래 예시는 실습 세 항목과 심화 과제를 한 프로그램에 합친 결과입니다.

## P1-C07-P1 · 속도 변경과 출력 예측

Velocity를 `10.0`으로 바꾸면 초기 위치가 0일 때 첫 Update 뒤 위치는 10이 됩니다. 실행 전에 값을 예상하고 실제 출력과 비교해야 단순한 값 변경이 아니라 System 실행 시점을 확인할 수 있습니다.

## P1-C07-P2 · Update 추가

### 확인 기준

- `app.update()`를 세 번 호출한다.
- Startup의 `setup`은 한 번만 실행된다.
- Update의 이동 System은 세 번 실행된다.

## P1-C07-P3 · 2차원 위치와 속도

### 힌트

스칼라 tuple struct 대신 x와 y를 가진 구조체를 사용하고 두 축을 각각 더합니다.

```rust
position.x += velocity.x;
position.y += velocity.y;
```

`Vec2`를 사용하는 구현도 가능합니다. 이 챕터에서는 Component의 필드와 System의 데이터 접근을 드러내기 위해 직접 만든 구조체를 사용합니다.

## P1-C07-A1 · clamp_position System

### 접근 방법

1. `move_player`가 위치를 변경합니다.
2. `clamp_position`이 두 축을 허용 범위로 제한합니다.
3. `print_position`이 최종 결과를 읽습니다.
4. 세 System을 `chain()`으로 연결합니다.

### 확인 기준

- 각 축이 `-10.0..=10.0`을 벗어나지 않는다.
- 출력 System이 제한되기 전 값이 아니라 제한된 값을 읽는다.
- 충분한 프레임을 실행해 상한과 하한을 확인한다.

### 수행 예시

```rust
app.add_systems(
    Update,
    (move_player, clamp_position, print_position).chain(),
);
```

System 순서를 등록 순서에 암묵적으로 의존하지 않고 명시한 점이 중요합니다. 위치 제한이 이동보다 먼저 실행되면 한 프레임 동안 범위를 벗어난 값이 남을 수 있습니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin system_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/system_solution.rs`

