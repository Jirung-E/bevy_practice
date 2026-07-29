# 17. 게임 UI

## 학습 목표

- 월드 공간 Text2d와 화면 공간 Text를 구분할 수 있다.
- Node의 절대 위치로 HUD를 배치할 수 있다.
- Resource 변경 내용을 UI에 반영할 수 있다.

## 이번에 만들 결과물

화면 왼쪽 위에 점수, HP, 최고 점수, 조작법이 표시되는 HUD를 추가합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 17_ui
```

## 핵심 개념

HUD는 카메라가 움직여도 화면에 고정되어야 하므로 UI `Text`와 `Node`를 사용합니다. 게임 세계의 좌표를 따라다니는 이름표나 피해 숫자는 `Text2d`가 적합합니다.

표시할 값의 원본은 Score, PlayerHealth, HighScore Resource입니다. UI 문자열을 별도 진실 공급원으로 사용하지 말고 매 프레임 원본 데이터에서 만들어야 불일치가 생기지 않습니다.

## 샘플 코드

```rust
commands.spawn((
    Hud,
    Text::new(""),
    TextFont {
        font_size: FontSize::Px(26.0),
        ..default()
    },
    Node {
        position_type: PositionType::Absolute,
        top: px(16),
        left: px(18),
        ..default()
    },
));
```

```rust
fn update_hud(
    score: Res<Score>,
    health: Res<PlayerHealth>,
    mut hud: Single<&mut Text, With<Hud>>,
) {
    hud.0 = format!("SCORE {:05}   HP {}", score.0, health.0);
}
```

## 코드 설명

- `Text`는 UI 문구, `TextFont`는 크기, Node는 레이아웃을 담당합니다.
- `PositionType::Absolute`는 부모 영역의 지정한 가장자리에서 위치를 잡습니다.
- `px(16)`은 Bevy 0.19의 명시적인 UI 픽셀 단위입니다.
- `{:05}`는 점수를 다섯 자리로 0 채움 표시합니다.
- `Hud` 표식으로 갱신 대상 Text만 Query합니다.

완성 코드에서는 UI가 꺼진 앞 챕터도 같은 라이브러리를 쓰므로 `Option<Single<...>>`로 HUD가 없는 경우를 처리합니다.

## 실습 과제

1. HUD를 오른쪽 위로 옮기세요.
2. 체력이 1일 때 글자를 빨간색으로 바꾸세요.
3. 중앙 상단에 생존 시간을 표시하세요.

## 심화 과제

체력을 텍스트 숫자 대신 세 개의 사각형 또는 하트 아이콘으로 표현하고, 체력이 바뀔 때만 UI 자식을 갱신하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part2/17_game_ui.md)

## 다음 챕터

적을 처치할 때 외부 사운드 파일 없이 생성되는 짧은 효과음을 재생합니다.
