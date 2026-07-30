# 22. GUI 레이아웃

## 학습 목표

- Node의 Flexbox 속성으로 화면을 분할할 수 있다.
- px와 percent 단위를 상황에 맞게 사용할 수 있다.
- UI 역할을 Component로 식별할 수 있다.

## 이 내용으로 만들 수 있는 것

- 탐색기처럼 사이드바와 본문이 나뉜 화면
- 상태 표시줄과 도구 모음이 있는 편집기
- 창 크기에 맞춰 비율이 달라지는 반응형 패널

## 이번에 만들 결과물

제목, 파일 목록 패널, 미리보기 패널, 하단 버튼과 상태 표시줄을 갖춘 File Lens 인터페이스를 만듭니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p file_lens --bin 22_ui
```

## 핵심 개념

Bevy UI의 각 요소는 Entity입니다. Node Component가 크기, 방향, 간격, 정렬을 정하고 Text와 BackgroundColor 같은 Component가 내용을 표현합니다.

최상위 Node는 세로 방향이며, 가운데 작업 영역은 가로 방향입니다. 왼쪽과 오른쪽 패널 폭을 38%와 62%로 나눕니다.

## 샘플 코드

```rust
commands.spawn((
    Node {
        width: percent(100),
        height: percent(100),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(px(20)),
        row_gap: px(14),
        ..default()
    },
    children![
        (Text::new("FILE LENS"),),
        (
            Node {
                width: percent(100),
                flex_grow: 1.0,
                column_gap: px(14),
                ..default()
            },
        )
    ],
));
```

## 코드 설명

- `percent(100)`은 부모의 사용 가능한 영역 전체를 차지합니다.
- `flex_direction`은 자식이 쌓이는 주축을 정합니다.
- `flex_grow: 1.0`은 고정 영역을 제외한 남은 공간을 사용합니다.
- `row_gap`과 `column_gap`은 자식 사이의 일정한 간격을 만듭니다.
- `Overflow::clip()`은 긴 미리보기가 패널 밖으로 그려지는 것을 막습니다.
- `ViewText` Component는 FileList, Preview, Status Text를 같은 Query로 안전하게 갱신하게 합니다.

## 실습 과제

1. 패널 비율을 50%씩 바꾸세요.
2. 창 폭이 작을 때 세로 배치가 되도록 별도 레이아웃을 실험하세요.
3. 패널 배경과 텍스트 색상 팔레트를 변경하세요.

## 심화 과제

파일 목록에 자식 Entity를 하나씩 생성하는 구조로 바꾸고, 목록이 길어질 때 ScrollPosition과 overflow를 적용하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part3/22_gui_layout.md)

## 다음 챕터

Clear와 Save Report 버튼의 Interaction을 읽고 마우스 상태에 따라 색을 바꿉니다.
