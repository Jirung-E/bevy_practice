# 25. 파일 입출력 과제 해설

[본문으로 돌아가기](../../25_FileIO.md#실습-과제)

## P3-C25-P1 · 텍스트와 이미지 비교

UTF-8 검증에 성공하면 제한 길이만큼 문자열을, 실패하면 바이트를 16진수로 표시합니다. 바이너리를 억지로 문자열로 바꾸면 대체 문자가 원본 정보를 가립니다.

## P3-C25-P2 · 1,024바이트 제한

파일 전체를 읽은 뒤 화면만 자르는 것과 처음부터 제한만큼 읽는 것은 메모리 사용이 다릅니다. 큰 파일 대응이 목표라면 `Read::take(1_024)`처럼 읽기 자체를 제한합니다.

## P3-C25-P3 · 확장자와 수정 시각

`Path::extension`과 metadata의 `modified()`를 보고서 필드로 추가합니다. 수정 시각 조회는 파일시스템에 따라 실패할 수 있으므로 Optional 값으로 다룹니다.

## P3-C25-A1 · 백그라운드 읽기

Bevy에서는 경로를 소유한 작업을 IO 풀에 보내고, 메인 스레드는 Task 완료 여부만 짧게 확인합니다.

```rust
let task = IoTaskPool::get().spawn(async move { std::fs::read(path) });
commands.spawn(PendingRead(task));
```

결과를 기다리려고 메인 System에서 block하면 TaskPool을 사용한 의미가 없습니다. 수행 예제는 같은 경계를 표준 스레드와 채널로 자동 테스트 가능한 형태로 보여 줍니다.

- 파일 읽기는 `IoTaskPool`이 목적에 맞습니다.
- CPU 집약적인 파싱은 `AsyncComputeTaskPool`을 검토합니다.
- Entity가 삭제돼도 작업이 남을 수 있으므로 취소 또는 결과 무시 정책이 필요합니다.

## 전체 코드 실행

```bash
cargo test -p file_lens --bin gui_workflow_solution
```

전체 코드: `examples/part3/file_lens/src/bin/gui_workflow_solution.rs`
