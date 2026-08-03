# 12E. 입력 Action과 장치 독립적인 명령

## 학습 목표

- 물리 입력과 게임 명령을 분리할 수 있다.
- 키보드와 게임패드를 같은 `PlayerCommand`로 변환할 수 있다.
- binding Resource를 바꿔 gameplay System 수정 없이 키를 재설정할 수 있다.
- 아날로그 스틱 dead zone과 방향 정규화의 목적을 설명할 수 있다.

## 이 내용으로 만들 수 있는 것

- 키보드·게임패드·AI가 같은 이동·발사 로직을 사용하는 게임
- 사용자 키 재설정과 입력 설정 저장
- UI 입력 중 gameplay 단축키를 차단하는 입력 계층

## 이번에 만들 결과물

WASD와 게임패드 왼쪽 스틱을 `Move`, Space와 게임패드 South 버튼을 `Fire` 명령으로 변환합니다. 예제는 W와 Space를 눌러 장치 독립적인 명령 두 개를 출력합니다.

```bash
cargo run -p ecs_basics --bin input_actions
```

## 핵심 개념

게임 로직이 `KeyCode::Space`를 직접 읽으면 키 재설정, 게임패드, AI, 리플레이를 추가할 때 발사 코드를 수정해야 합니다. 입력 계층은 장치 상태를 도메인 명령으로 번역합니다.

```text
키보드 / 게임패드 / AI / 네트워크
                 ↓
             PlayerCommand
                 ↓
             Gameplay System
```

```rust
enum PlayerCommand {
    Move(Vec2),
    Fire,
}
```

`InputBindings` Resource에는 물리 키와 버튼만 저장합니다. gameplay은 binding을 모르고 `PlayerCommand`만 소비합니다. 스틱의 작은 흔들림은 dead zone 안에서 무시하고, 키보드 대각선과 스틱 방향은 `normalize_or_zero()`로 길이를 제한합니다.

게임패드는 실행 중 연결·해제될 수 있으므로 특정 장치가 항상 존재한다고 가정하지 않습니다. 여러 장치를 허용할지 첫 번째 활성 장치만 소유할지도 정책으로 정해야 합니다.

## 샘플 코드

전체 코드: `examples/part1/ecs_basics/src/bin/12e_input_actions.rs`

```rust
if movement != Vec2::ZERO {
    commands.write(PlayerCommandMessage(PlayerCommand::Move(
        movement.normalize_or_zero(),
    )));
}
if fire {
    commands.write(PlayerCommandMessage(PlayerCommand::Fire));
}
```

## 코드 설명

- `InputBindings`를 수정하면 번역 System만 다른 물리 입력을 읽습니다.
- `PlayerCommandMessage`는 gameplay이 입력 장치 타입에 의존하지 않게 합니다.
- 키보드 방향과 게임패드 축을 같은 `Vec2`에 합칩니다.
- `Gamepad::just_pressed`는 버튼의 한 번 누름을 감지합니다.
- 테스트는 Space를 F로 재설정해도 결과가 여전히 `Fire`인지 검사합니다.

## 실습 과제

1. `Pause` Action과 기본 Escape/Start binding을 추가하세요.
2. 방향키를 WASD와 동시에 지원하도록 하나의 Action에 여러 키를 연결하세요.
3. 게임패드 dead zone을 0.1과 0.4로 바꾸고 작은 축 입력 결과를 비교하세요.

## 심화 과제

binding을 RON으로 저장하고 불러오며, 같은 키가 상충하는 두 Action에 배정될 때 거부하거나 교체하는 정책을 구현하세요. UI가 키 입력을 기다리는 rebinding 상태에서는 gameplay 명령이 발생하지 않게 하세요.

[힌트와 수행 예시](exercises/part1/12e_input_actions.md)

## 다음 챕터

Update에서 만들어진 짧은 입력 명령을 버퍼에 넣고 FixedUpdate가 정확히 한 번 소비하게 합니다.
