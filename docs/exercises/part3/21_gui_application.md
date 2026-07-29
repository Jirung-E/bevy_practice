# 21. GUI 애플리케이션 과제 해설

[본문으로 돌아가기](../../21_GuiApplication.md#실습-과제)

## P3-C21-P1 · 창 제목과 크기

`WindowPlugin`의 `primary_window`에 제목과 `WindowResolution`을 지정합니다. 콘텐츠가 잘리지 않는 최소 크기도 같은 Window 설정에서 지정하세요.

## P3-C21-P2 · 최소 창 크기

최소 폭·높이는 단순한 권장값이 아니라 레이아웃이 기능을 유지하는 경계입니다. 실제로 경계 크기까지 창을 줄여 버튼과 텍스트가 겹치지 않는지 확인합니다.

## P3-C21-P3 · 시작 문구 스타일

글자 크기는 `TextFont`, 색은 `TextColor`가 담당합니다. 레이아웃 Node와 표현 Component를 분리하면 스타일을 바꿔도 중앙 정렬은 유지됩니다.

## P3-C21-A1 · 중앙 정렬이 유지되는 이유

루트 Node가 카메라의 UI viewport 전체를 차지하고, 자식의 폭·높이와 정렬이 percent 단위 및 `JustifyContent::Center`, `AlignItems::Center`로 계산되기 때문입니다. 카메라가 창 크기 변경 뒤 viewport를 갱신하면 같은 비율 규칙이 새 크기에 다시 적용됩니다.

고정 픽셀 좌표는 창 크기가 달라져도 그대로지만, percent와 Flexbox는 부모의 새 크기를 기준으로 재계산된다는 차이가 있습니다.

## 전체 코드 실행

```bash
cargo run -p file_lens --bin gui_workflow_solution
cargo test -p file_lens --bin gui_workflow_solution
```

전체 코드: `examples/part3/file_lens/src/bin/gui_workflow_solution.rs`
