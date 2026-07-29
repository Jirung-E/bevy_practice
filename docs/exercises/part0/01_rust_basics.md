# 01. Rust 기초 과제 해설

[본문으로 돌아가기](../../01_RustBasics.md#실습-과제)

## P0-C01-P1 · 음수 방향 이동

`direction`을 `-1.0`으로 전달하면 `0.0 + (-1.0 × 120.0 × 0.5)`이므로 위치는 `-60.0`입니다. 실행 전에 값을 계산한 뒤 출력과 비교하세요.

## P0-C01-P2 · health 필드

`Player`에 `health: u32`를 추가하면 모든 생성 지점에도 초기값을 넣어야 합니다. 예제에서는 `health: 100`을 사용합니다.

## P0-C01-P3 · Dead 상태

### 확인 기준

- `PlayerState::Dead`가 명시적으로 존재한다.
- 체력이 0이면 위치가 바뀌지 않는다.
- 죽은 플레이어의 상태를 Moving으로 덮어쓰지 않는다.

이 조건은 `move_player`의 시작 부분에서 먼저 검사하는 편이 읽기 쉽습니다.

## P0-C01-A1 · saturating damage

```rust
fn take_damage(player: &mut Player, amount: u32) {
    player.health = player.health.saturating_sub(amount);
    if player.health == 0 {
        player.state = PlayerState::Dead;
    }
}
```

`saturating_sub`는 20에서 50을 빼도 underflow하지 않고 0을 반환합니다. 일반 뺄셈 뒤에 0과 비교하면 debug 빌드에서 그 전에 panic할 수 있습니다.

### 전체 코드 실행

```bash
cargo run -p hello_bevy --bin rust_basics_solution
cargo test -p hello_bevy --bin rust_basics_solution
```

전체 코드: `examples/part0/hello_bevy/src/bin/rust_basics_solution.rs`

