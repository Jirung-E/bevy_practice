# 39. Asset Browser 과제 해설

[본문으로 돌아가기](../../39_AssetBrowser.md#실습-과제)

## P6-C39-P1 · Cylinder

버튼마다 spawn 코드를 복사하지 말고 AssetKind 또는 프리팹 ID를 담은 Action을 만듭니다. 생성 System이 Mesh·Material과 이름 규칙을 한 곳에서 적용합니다.

## P6-C39-P2 · 생성 offset

생성 순번에서 grid 좌표를 계산해 겹치지 않게 배치합니다. 삭제 후 순번 재사용 여부와 월드 경계를 정책으로 정합니다.

## P6-C39-P3 · Material 변형

같은 Mesh 프리팹에 Material preset ID를 조합합니다. 색 이름만이 아니라 실제 material Handle과 미리보기 색을 데이터에 둡니다.

## P6-C39-A1 · 비동기 카드 브라우저

IO TaskPool에서 `assets`를 순회하고 확장자로 Scene/Image/Audio/Unknown을 분류합니다. 메인 스레드는 발견 결과를 batch로 받아 카드 Entity를 만들고 AssetServer의 load state를 표시합니다.

- 폴더 스캔과 GPU 썸네일 생성은 서로 다른 단계입니다.
- 파일 변경 감시는 전체 재스캔보다 추가·수정·삭제 delta를 반영합니다.
- 알 수 없는 확장자는 버리지 말고 Unknown으로 보여 문제를 조사할 수 있게 합니다.
- 경로는 AssetServer root 기준 논리 경로와 OS 절대 경로를 혼동하지 않습니다.

## 전체 코드 실행

```bash
cargo test -p world_editor --bin editor_model_solution
```

전체 코드: `examples/part6/world_editor/src/bin/editor_model_solution.rs`
