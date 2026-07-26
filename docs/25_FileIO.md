# 25. 파일 입출력

## 학습 목표

- 텍스트와 바이너리 파일을 안전하게 미리 볼 수 있다.
- 읽기 크기를 제한하는 이유를 이해한다.
- 검사 결과를 보고서 파일로 저장할 수 있다.

## 이번에 만들 결과물

드롭한 파일의 앞부분을 미리보기 패널에 표시하고 Save Report 버튼으로 `output/file_report.txt`를 생성합니다.

```bash
cargo run -p file_lens --bin 25_file_io
```

## 핵심 개념

파일 내용은 유효한 UTF-8이라고 가정할 수 없습니다. 예제는 앞 600바이트만 선택해 UTF-8이면 텍스트로, 아니면 앞 64바이트를 16진수로 표시합니다.

현재 예제는 학습을 위해 `fs::read`를 사용합니다. 매우 큰 파일은 전체를 메모리에 읽지 않고 `File::take` 또는 비동기 작업으로 필요한 부분만 읽어야 합니다.

## 샘플 코드

```rust
fn preview_bytes(bytes: &[u8], limit: usize) -> String {
    let slice = &bytes[..bytes.len().min(limit)];
    match std::str::from_utf8(slice) {
        Ok(text) => text.to_owned(),
        Err(_) => slice
            .iter()
            .take(64)
            .map(|byte| format!("{byte:02X}"))
            .collect::<Vec<_>>()
            .join(" "),
    }
}
```

보고서 저장은 `create_dir_all("output")` 후 파일별 경로와 바이트 크기를 작성합니다.

## 코드 설명

- `min(limit)`으로 짧은 파일을 범위 밖에서 자르지 않습니다.
- `from_utf8`은 데이터를 복사하지 않고 유효성부터 검사합니다.
- 바이너리 fallback 덕분에 잘못된 인코딩이 panic을 만들지 않습니다.
- 출력 폴더 생성과 쓰기 오류는 Result로 호출자에게 돌려 UI에 표시합니다.
- 실제 파일 경로는 민감한 정보일 수 있으므로 보고서를 공유하기 전에 정책을 정해야 합니다.

## 실습 과제

1. 텍스트 파일과 이미지 파일을 각각 드롭해 결과를 비교하세요.
2. 미리보기 제한을 1,024바이트로 바꾸세요.
3. 보고서에 확장자와 수정 시각을 추가하세요.

## 심화 과제

Bevy TaskPool을 사용해 파일 읽기를 백그라운드 작업으로 옮기고, 큰 파일을 드롭해도 UI 프레임이 멈추지 않도록 만드세요.

## 다음 챕터

Empty, Ready, Error 상태로 애플리케이션 흐름을 명시하고 처리 결과에 따라 전환합니다.

