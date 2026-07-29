# 23. 버튼과 이벤트

## 학습 목표

- Button과 Interaction Component를 사용할 수 있다.
- 변경 감지로 이벤트성 입력만 처리할 수 있다.
- UI 입력과 애플리케이션 로직을 분리할 수 있다.

## 이번에 만들 결과물

마우스를 올리거나 누르면 색이 바뀌는 버튼을 만들고 Clear 버튼으로 FileModel을 초기화합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p file_lens --bin 23_events
```

## 핵심 개념

버튼은 별도의 콜백 객체가 아니라 `Button`, `Interaction`, Node를 가진 Entity입니다. Interaction은 None, Hovered, Pressed 중 하나이며 Bevy 입력 시스템이 갱신합니다.

`Changed<Interaction>` Filter를 사용하면 모든 프레임에 모든 버튼을 처리하지 않고 상태가 바뀐 버튼만 다룹니다.

## 샘플 코드

```rust
fn handle_clear_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<ClearButton>)>,
    mut model: ResMut<FileModel>,
) {
    if interactions
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        model.files.clear();
        model.status = "Cleared".into();
    }
}
```

버튼 시각 피드백은 별도 `paint_buttons` System이 Interaction에 따라 BackgroundColor를 바꿉니다.

## 코드 설명

- ClearButton은 특정 버튼의 의미를 나타내는 표식 Component입니다.
- 입력 System은 모델만 수정하고 Text를 직접 찾지 않습니다.
- View System은 FileModel의 변경을 감지해 목록과 상태 문구를 갱신합니다.
- 로직과 표현을 분리하면 키보드 단축키나 메뉴 입력도 같은 모델 작업을 재사용할 수 있습니다.

Bevy 0.19의 접근성 시스템과 키보드 포커스를 위해 `InputFocus` Resource도 초기화합니다.

## 실습 과제

1. Clear 버튼의 hover·pressed 색을 바꾸세요.
2. Delete 키로도 같은 초기화 동작을 실행하세요.
3. 파일이 없을 때 Save 버튼 색을 흐리게 표시하세요.

## 심화 과제

`ClearRequested` Event와 Observer를 추가해 버튼과 단축키가 같은 작업 이벤트를 trigger하도록 재구성하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part3/23_gui_events.md)

## 다음 챕터

운영체제에서 창으로 끌어온 파일 경로를 FileDragAndDrop Message로 받습니다.
