# 24. Drag & Drop 과제 해설

[본문으로 돌아가기](../../24_DragAndDrop.md#실습-과제)

## P3-C24-P1 · 폴더 거부

드롭 경로의 metadata를 확인해 `is_file()`이 아니면 모델에 넣지 않고 사용자가 행동을 수정할 수 있는 오류 문구를 표시합니다.

## P3-C24-P2 · 중복 경로

현재 목록의 `PathBuf`를 `HashSet`으로 만든 뒤 새 경로를 삽입합니다. 문자열 비교 전에 경로 정규화와 심볼릭 링크 처리 정책도 정해야 합니다.

## P3-C24-P3 · hover 피드백

`HoveredFile`에서 드롭 영역을 강조하고 `HoveredFileCanceled` 또는 실제 drop 뒤 원래 색으로 되돌립니다. 전역 배경이 아니라 드롭 대상 패널만 바꾸는 편이 의미가 분명합니다.

## P3-C24-A1 · 다중 드롭 정책

수행 예시는 다음 순서를 적용합니다.

1. 기존 경로와 중복 제거
2. 최대 파일 크기 거부
3. 전체 최대 개수 제한
4. 확장자, 크기, 이름 순 정렬

제한을 먼저 적용하고 정렬하면 입력이 허용 개수를 넘었을 때 운영체제가 전달한 순서가 선택 결과에 영향을 줍니다. “전체 후보를 정렬한 뒤 상위 N개”가 요구사항이라면 정렬과 truncate 순서를 바꾸세요.

## 전체 코드 실행

```bash
cargo test -p file_lens --bin gui_workflow_solution
```

전체 코드: `examples/part3/file_lens/src/bin/gui_workflow_solution.rs`
