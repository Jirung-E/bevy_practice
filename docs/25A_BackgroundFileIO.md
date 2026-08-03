# 25A. UI를 멈추지 않는 비동기 파일 입출력

## 학습 목표

- 파일 입출력이 메인 스레드를 멈추게 만드는 이유를 이해합니다.
- `IoTaskPool`에서 파일 읽기 작업을 실행합니다.
- `Task`를 `check_ready`로 확인하고 완료 결과를 ECS World에 반영합니다.

## 이 내용으로 만들 수 있는 것

- 큰 파일을 여는 동안에도 움직이는 로딩 UI
- 여러 에셋과 문서를 병렬로 검사하는 Asset Browser
- 저장·내보내기 작업의 성공과 실패를 나중 프레임에 표시하는 편집기

## 이번에 만들 결과물

창에 파일을 드롭하면 최대 64 KiB를 백그라운드에서 읽고, 완료되면 텍스트 또는 16진수 미리보기를 표시합니다. 아래 명령은 교재 저장소의 완성 샘플을 실행합니다.

```bash
cargo run -p file_lens --bin background_file_io
```

## 핵심 개념

Bevy의 `Update` System은 매 프레임 실행됩니다. 그 안에서 큰 파일을 동기적으로 끝까지 읽으면 읽기가 끝날 때까지 입력, UI 배치와 렌더링도 진행하지 못합니다. 파일·네트워크 같은 대기 중심 작업은 `IoTaskPool`, 무거운 계산은 `AsyncComputeTaskPool`에 맡기는 것이 목적에 맞습니다.

작업 결과를 기다린다고 `block_on`하면 백그라운드 작업을 만든 의미가 사라집니다. `Task<Result<T, E>>`를 Component로 보관하고 다음 프레임에 `check_ready`를 호출하면, 준비되지 않은 프레임에는 즉시 반환하고 완료된 프레임에만 결과를 꺼낼 수 있습니다.

```text
파일 드롭 → Task Entity 생성 → IoTaskPool에서 읽기
                 ↓ 매 프레임 check_ready
              완료 결과 → UI 갱신 → Task Entity despawn
```

## 샘플 코드

```rust
let task = IoTaskPool::get().spawn(async move { read_preview(path) });
commands.spawn(ReadFileTask(task));

for (entity, mut task) in &mut tasks {
    let Some(result) = check_ready(&mut task.0) else {
        continue;
    };
    // result를 UI 모델에 반영
    commands.entity(entity).despawn();
}
```

전체 실행 코드는 [25a_background_file_io.rs](source/part3.md#25a--백그라운드-파일-입출력)에서 확인할 수 있습니다.

## 코드 설명

- `ReadFileTask` Component가 진행 중인 `Task`를 World 안에서 추적합니다.
- 백그라운드 closure에는 `PathBuf`처럼 소유권을 넘길 수 있는 값만 전달합니다. 일반 ECS 참조를 작업 스레드에서 직접 사용하지 않습니다.
- 작업은 `Result<FilePreview, String>`을 반환하므로 성공과 실패가 같은 완료 경로로 돌아옵니다.
- `File::take`로 읽기 상한을 두어 미리보기 하나가 메모리를 무제한 점유하지 않게 합니다.
- 완료 결과를 한 번 처리한 뒤 Task Entity를 제거해야 같은 작업을 다시 poll하지 않습니다.

## 실습 과제

1. 동시에 여러 파일을 드롭하고 완료 순서가 드롭 순서와 다를 수 있는지 확인하세요.
2. 진행 중인 작업 수를 화면에 표시하세요.
3. 미리보기 제한을 상수 대신 Resource 설정값으로 옮기세요.

## 심화 과제

각 Task에 요청 번호를 부여하고, 사용자가 새 파일을 선택하면 이전 요청의 늦은 결과가 현재 미리보기를 덮어쓰지 못하도록 만드세요. 작업 취소와 결과 무시가 어떻게 다른지도 기록하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part3/25a_background_file_io.md)

## 다음 챕터

Empty, Ready, Error 상태로 애플리케이션 흐름을 명시하고 처리 결과에 따라 전환합니다.
