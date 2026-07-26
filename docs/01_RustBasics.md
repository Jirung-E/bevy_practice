# 01. 실습에 필요한 Rust 기초

## 학습 목표

- 변수, 구조체, 열거형, 함수의 기본 문법을 사용할 수 있다.
- 소유권을 복사와 빌림 관점에서 구분할 수 있다.
- Bevy 코드에서 자주 보는 `derive`와 `match`를 읽을 수 있다.

## 이번에 만들 결과물

플레이어의 위치와 상태를 표현하고, 입력에 따라 위치를 바꾸는 작은 콘솔 프로그램을 만듭니다. 아직 Bevy는 사용하지 않지만 이후 Component와 State를 이해하는 바탕이 됩니다.

## 핵심 개념

### 변수와 가변성

Rust 변수는 기본적으로 변경할 수 없습니다. 값을 바꾸려면 `mut`를 붙입니다.

```rust
let speed = 5.0;
let mut position = 0.0;
position += speed;
```

### 구조체와 구현 블록

구조체는 관련 데이터를 하나로 묶습니다. `impl` 블록에는 그 데이터를 다루는 연관 함수와 메서드를 작성합니다.

### 열거형과 패턴 매칭

열거형은 가능한 상태를 제한합니다. `match`는 모든 경우를 빠짐없이 처리하도록 컴파일러가 검사합니다.

### 소유권과 빌림

값을 함수에 넘길 때 소유권을 옮길 수도 있고, `&T` 또는 `&mut T`로 잠시 빌려줄 수도 있습니다. 위치를 갱신하는 함수는 플레이어 자체를 가져갈 필요가 없으므로 `&mut Player`를 받습니다.

## 샘플 코드

다음 코드를 별도의 Rust 프로젝트 `src/main.rs`에 입력해 실행해 보세요.

```rust
#[derive(Debug)]
struct Player {
    name: String,
    x: f32,
    speed: f32,
    state: PlayerState,
}

#[derive(Debug)]
enum PlayerState {
    Idle,
    Moving,
}

fn move_player(player: &mut Player, direction: f32, delta_seconds: f32) {
    if direction == 0.0 {
        player.state = PlayerState::Idle;
        return;
    }

    player.x += direction * player.speed * delta_seconds;
    player.state = PlayerState::Moving;
}

fn main() {
    let mut player = Player {
        name: String::from("Ferris"),
        x: 0.0,
        speed: 120.0,
        state: PlayerState::Idle,
    };

    move_player(&mut player, 1.0, 0.5);
    println!("{}: {player:?}", player.name);
}
```

실행:

```bash
cargo run
```

## 코드 설명

- `#[derive(Debug)]`는 `{:?}` 형식으로 값을 출력하는 구현을 컴파일러가 생성하게 합니다. Bevy에서는 같은 방식으로 `Component`, `Resource`, `States` 같은 특성을 파생합니다.
- `String`은 문자열 데이터를 소유합니다.
- `f32`는 Bevy의 위치, 회전, 시간 계산에서 자주 쓰는 32비트 실수입니다.
- `&mut Player`는 호출한 쪽이 소유한 Player를 독점적으로 빌려 수정합니다.
- `delta_seconds`를 곱하면 컴퓨터의 프레임 속도와 무관한 이동을 만들 수 있습니다.
- `return` 뒤에는 현재 함수의 나머지 코드가 실행되지 않습니다.

## 실습 과제

1. 방향을 `-1.0`으로 바꾸고 위치가 감소하는지 확인하세요.
2. `Player`에 `health: u32` 필드를 추가하세요.
3. `PlayerState`에 `Dead`를 추가하고 체력이 0일 때 이동하지 않게 만드세요.

## 심화 과제

`take_damage(player: &mut Player, amount: u32)` 함수를 작성하세요. 체력은 0보다 작아지지 않아야 하며 0이 되면 상태를 `Dead`로 바꿔야 합니다. `u32::saturating_sub`를 조사해 사용해 보세요.

## 다음 챕터

다음 챕터에서는 Cargo 패키지와 워크스페이스를 이해하고, 교재의 여러 실행 예제가 어떻게 함께 관리되는지 살펴봅니다.

