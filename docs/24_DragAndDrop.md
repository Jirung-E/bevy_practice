# 24. 파일 Drag & Drop

## 학습 목표

- 운영체제 파일 Drag & Drop Message를 읽을 수 있다.
- 드롭, hover, 취소 이벤트를 구분할 수 있다.
- 신뢰할 수 없는 외부 경로를 검증할 수 있다.

## 이번에 만들 결과물

탐색기나 Finder에서 File Lens 창으로 파일을 끌어 놓으면 파일명과 크기가 목록에 추가됩니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p file_lens --bin 24_drag_drop
```

## 핵심 개념

WindowPlugin은 운영체제 Drag & Drop을 `FileDragAndDrop` Message로 변환합니다. MessageReader는 아직 읽지 않은 이벤트만 처리합니다.

외부에서 받은 경로는 파일이라고 가정하면 안 됩니다. 폴더, 사라진 파일, 권한이 없는 경로일 수 있으므로 `fs::metadata` 결과와 `is_file()`을 확인합니다.

## 샘플 코드

```rust
fn handle_file_drop(
    mut dropped: MessageReader<FileDragAndDrop>,
    mut model: ResMut<FileModel>,
) {
    for event in dropped.read() {
        let FileDragAndDrop::DroppedFile { path_buf, .. } = event else {
            continue;
        };

        match inspect_file(path_buf, false) {
            Ok(entry) => model.files.push(entry),
            Err(error) => model.status = error,
        }
    }
}
```

## 코드 설명

- 패턴 매칭으로 DroppedFile만 처리하고 HoveredFile과 HoveredFileCanceled는 건너뜁니다.
- path_buf는 소유 경로 PathBuf이며 검사 함수에는 `&Path`로 빌려줍니다.
- 메타데이터 읽기 오류를 String으로 변환해 UI 상태에 표시합니다.
- 같은 파일 중복을 허용할지 여부는 제품 정책입니다. 현재 예제는 드롭 기록을 그대로 보존합니다.

## 실습 과제

1. 폴더를 드롭해 오류 문구를 확인하세요.
2. 같은 경로가 이미 있으면 추가하지 않도록 만드세요.
3. HoveredFile 동안 드롭 패널 배경색을 바꾸세요.

## 심화 과제

여러 파일을 한 번에 드롭할 때 확장자, 크기, 이름 기준으로 정렬하고 허용 최대 개수와 최대 파일 크기를 적용하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part3/24_drag_and_drop.md)

## 다음 챕터

UTF-8 텍스트는 내용 일부를 표시하고 바이너리는 안전한 16진수 미리보기를 만듭니다.
