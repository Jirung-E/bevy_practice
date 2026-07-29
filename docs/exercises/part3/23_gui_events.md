# 23. GUI 이벤트 과제 해설

[본문으로 돌아가기](../../23_GuiEvents.md#실습-과제)

## P3-C23-P1 · 버튼 색 상태

`Changed<Interaction>` Query에서 `Pressed`, `Hovered`, `None`별 배경색을 지정합니다. 매 프레임 전체 버튼을 훑는 대신 Interaction이 바뀐 Entity만 처리합니다.

## P3-C23-P2 · Delete 단축키

버튼과 단축키가 각각 모델을 직접 초기화하지 않게 하세요. 두 입력 모두 같은 `ClearRequested` 의도를 만들면 실제 초기화 규칙은 한 곳에만 남습니다.

## P3-C23-P3 · 비활성 Save 표현

파일이 없을 때 색만 흐리게 하는 것으로 끝내지 말고 Save 의도도 거부해야 합니다. 수행 예시는 비어 있는 모델에서 `SaveRequested`가 `false`가 되는지 확인합니다.

## P3-C23-A1 · Event와 Observer

```rust
#[derive(Event)]
struct ClearRequested;

commands.trigger(ClearRequested);

fn clear_files(
    _trigger: On<ClearRequested>,
    mut model: ResMut<FileModel>,
) {
    model.files.clear();
}
```

Observer를 쓰면 버튼과 키보드 입력은 “요청 발생”까지만 담당하고, 모델 변경은 Observer 하나가 담당합니다. 요청을 기록·취소·검증해야 한다면 중간 정책 System을 두는 편이 낫습니다.

## 전체 코드 실행

```bash
cargo test -p file_lens --bin gui_workflow_solution
```

전체 코드: `examples/part3/file_lens/src/bin/gui_workflow_solution.rs`
