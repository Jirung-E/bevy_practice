# 12E. 입력 Action 과제 해설

[본문으로 돌아가기](../../12E_InputActions.md#실습-과제)

## P1-C12E-P1 · Pause Action

`PlayerCommand::Pause`를 추가하고 Escape와 `GamepadButton::Start`가 같은 명령을 만들게 합니다. gameplay은 어느 장치가 Pause를 발생시켰는지 몰라도 됩니다.

## P1-C12E-P2 · 여러 키 binding

한 Action에 `Vec<KeyCode>` 또는 작은 배열을 저장하고 `any_pressed`로 검사합니다. WASD와 방향키가 동시에 눌렸을 때 방향을 두 번 더하지 않도록 Action별 bool을 먼저 계산합니다.

## P1-C12E-P3 · Dead zone

축 길이가 dead zone보다 작으면 `Vec2::ZERO`, 크면 정규화된 방향을 반환하는 순수 함수를 분리합니다. 경계 바로 아래·같음·바로 위 값을 테스트합니다.

## P1-C12E-A1 · Rebinding 상태

`Rebinding(Action)` 상태에서는 다음 물리 입력을 binding에 저장하고 gameplay 번역 System은 실행하지 않습니다. 중복 키는 기존 binding을 해제하거나 요청을 거부하는 정책 중 하나를 명시합니다.
