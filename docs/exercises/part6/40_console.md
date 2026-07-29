# 40. Console 과제 해설

[본문으로 돌아가기](../../40_Console.md#실습-과제)

## P6-C40-P1 · 경과 시간

로그 생성 시점에 앱 시작 이후 초를 저장합니다. 화면을 그릴 때 현재 시간을 붙이면 과거 로그의 시간이 계속 바뀌는 오류가 생깁니다.

## P6-C40-P2 · 로그 수준

Info, Warning, Error를 enum으로 두고 View에서 색을 선택합니다. 문자열 접두사만 파싱하는 방식보다 필터링과 통계가 안전합니다.

## P6-C40-P3 · 최대 1,000개

`VecDeque`에 새 로그를 뒤로 넣고 capacity 도달 시 앞의 가장 오래된 로그를 제거합니다. 수행 예제는 capacity 2에 세 로그를 넣었을 때 1, 2만 남는지 검사합니다.

## P6-C40-A1 · 명령 파서

`spawn cube`, `select next`, `move x 1.0`을 토큰화해 `EditorAction`으로 변환하고 기존 실행 경로에 전달합니다. 파서가 World를 직접 수정하면 Inspector 검증과 로그, undo를 우회합니다.

- 알 수 없는 명령, 누락 인수, 잘못된 숫자는 오류 로그로 돌려줍니다.
- 임의 코드 실행이 아니라 명시적인 명령 allowlist만 제공합니다.
- undo/redo를 위해 Action 적용 전 값을 역명령 또는 snapshot으로 기록합니다.

수행 예제는 `move x 1.5`가 선택 Entity를 포함한 MoveX Action과 정확히 같은지 검사합니다.

## 전체 코드 실행

```bash
cargo run -p world_editor --bin editor_model_solution
cargo test -p world_editor --bin editor_model_solution
```

전체 코드: `examples/part6/world_editor/src/bin/editor_model_solution.rs`
