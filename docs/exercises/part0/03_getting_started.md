# 03. 첫 Bevy 프로젝트 과제 해설

[본문으로 돌아가기](../../03_GettingStarted.md#실습-과제)

## P0-C03-P1~P4 · 창과 화면 설정

### 확인 기준

- 창 제목이 기본값과 다르다.
- 해상도가 정확히 `1280 × 720`이다.
- 배경색 RGB 변경을 화면에서 구분할 수 있다.
- 안내 문구, 크기, 색이 모두 변경된다.

색상은 `Color::srgb(r, g, b)`에서 각 채널이 `0.0..=1.0` 범위임을 지키세요. 한 번에 모두 바꾸기보다 한 채널씩 바꾸면 각 값의 영향을 확인하기 쉽습니다.

## P0-C03-A1 · 두 번째 Text2d

```rust
commands.spawn((
    Text2d::new("Press SPACE to start"),
    TextFont {
        font_size: FontSize::Px(28.0),
        ..default()
    },
    TextColor(Color::srgb(0.55, 0.8, 1.0)),
    Transform::from_xyz(0.0, -70.0, 0.0),
));
```

첫 Text2d와 별도 Entity이므로 두 문구가 각자 TextFont, TextColor, Transform을 가질 수 있습니다.

### 전체 코드 실행

```bash
cargo run -p hello_bevy --bin getting_started_solution
```

전체 코드: `examples/part0/hello_bevy/src/bin/getting_started_solution.rs`

