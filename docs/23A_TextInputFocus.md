# 23A. 텍스트 입력, 포커스와 IME

## 학습 목표

- `EditableText`로 실제 편집 가능한 입력 필드를 만듭니다.
- `InputFocus`, `TabGroup`, `TabIndex`로 키보드 입력의 목적지를 정합니다.
- 한글처럼 조합 과정이 필요한 문자를 IME를 통해 안전하게 입력합니다.

## 이 내용으로 만들 수 있는 것

- 검색창, 이름 입력창, 채팅창과 콘솔 명령 입력기
- Tab 키로 이동할 수 있는 설정 화면과 데이터 입력 폼
- 한글·일본어·중국어 입력을 지원하는 데스크톱 도구

## 이번에 만들 결과물

두 입력 필드 사이를 Tab으로 이동하고, 한글을 입력한 뒤 Enter로 현재 값을 제출하는 폼을 만듭니다. 아래 명령은 교재 저장소에 포함된 완성 샘플을 실행합니다.

```bash
cargo run -p file_lens --bin text_input_focus
```

## 핵심 개념

키가 눌렸다는 사실과 그 키를 받을 UI는 별개의 문제입니다. `InputFocus`는 현재 입력 대상 Entity를 보관하고, `TabIndex`는 Tab 이동 순서를 정의합니다. 포커스되지 않은 필드가 같은 키를 함께 처리하지 않도록 입력 목적지를 먼저 확정해야 합니다.

`EditableText`는 문자열, 커서, 선택 영역과 IME 조합 상태를 함께 관리합니다. 한글 입력을 `KeyboardInput`의 문자 하나씩 이어 붙여 구현하면 조합 중인 `ㅎ`, `하`, `한` 상태와 확정 문자열을 구분하기 어렵습니다. 텍스트 편집은 `EditableText`에 맡기고 애플리케이션은 제출된 값만 읽는 편이 안전합니다.

시스템 글꼴을 사용하기 위해 샘플의 Bevy dependency에는 `system_font_discovery` 기능을 켰습니다. 운영체제에 해당 문자 글리프가 있는 글꼴이 설치되어 있어야 실제 문자가 표시됩니다.

## 샘플 코드

```rust
fn submit_focused_input(
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    inputs: Query<(&EditableText, &Name)>,
    mut output: Single<&mut Text, With<SubmittedText>>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Some(entity) = focus.get() else { return };
    let Ok((input, name)) = inputs.get(entity) else { return };
    output.0 = format!("SUBMITTED ({name}): {}", input.value());
}
```

전체 실행 코드는 [23a_text_input_focus.rs](source/part3.md#23a--텍스트-입력과-ime)에서 확인할 수 있습니다.

## 코드 설명

- `TabNavigationPlugin`이 `TabGroup` 안의 `TabIndex` 순서대로 포커스를 이동합니다.
- `AutoFocus`는 폼이 생성됐을 때 첫 입력 필드가 바로 키를 받도록 합니다.
- `InputFocus::get()`으로 현재 Entity를 찾은 뒤 그 Entity의 `EditableText`만 읽습니다.
- 포커스가 바뀌면 테두리 색을 바꿔 현재 입력 위치를 눈으로 확인합니다.
- 입력 위젯이 운영체제 IME의 조합 문자열과 확정 문자열을 처리하므로 게임 로직에서 한글 조합기를 다시 만들지 않습니다.

## 실습 과제

1. 두 입력 필드에 서로 다른 라벨을 추가하고 제출 결과에 라벨을 표시하세요.
2. Shift+Tab으로 포커스가 역순으로 이동하는지 확인하세요.
3. 입력값이 비어 있으면 제출하지 않고 테두리를 빨간색으로 표시하세요.

## 심화 과제

Esc로 포커스를 해제하고, Ctrl+Enter로만 여러 줄 입력을 제출하는 메모 필드를 추가하세요. 일반 Enter는 줄바꿈이어야 하며 IME 조합 중에는 제출되지 않아야 합니다.

[선택형 과제 해설과 수행 예시 보기](exercises/part3/23a_text_input_focus.md)

## 다음 챕터

운영체제에서 창으로 끌어온 파일 경로를 `FileDragAndDrop` Message로 받습니다.
