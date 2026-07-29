# 22. GUI 레이아웃 과제 해설

[본문으로 돌아가기](../../22_GuiLayout.md#실습-과제)

## P3-C22-P1 · 50% 패널

두 패널의 `width`를 각각 `percent(50)`로 바꿉니다. 패딩과 gap이 더해질 수 있으므로 부모의 가용 폭을 기준으로 실제 결과를 확인하세요.

## P3-C22-P2 · 작은 창의 세로 배치

창 폭을 읽어 임계값 아래에서는 `FlexDirection::Column`, 그 이상에서는 `Row`를 선택합니다. 수행 예시의 `responsive_orientation`은 720픽셀을 경계로 이 결정을 테스트합니다.

## P3-C22-P3 · 팔레트

배경·본문·강조·오류 색을 상수 또는 Resource에 모읍니다. 각 Entity에 숫자 색값을 흩어 놓으면 테마 변경과 대비 검사가 어렵습니다.

## P3-C22-A1 · 자식 목록과 스크롤

각 파일을 별도 자식 Entity로 만들고 목록 컨테이너에 세로 overflow와 `ScrollPosition`을 둡니다. 텍스트 하나를 매번 다시 만드는 방식보다 선택·삭제·강조 대상이 명확해집니다.

- 항목 수가 작으면 전체 자식 재구성도 충분히 단순합니다.
- 수천 항목이면 보이는 범위만 Entity로 만드는 가상화가 필요합니다.
- 스크롤 위치는 목록 갱신 전후에 보존할지 초기화할지 UX 규칙을 정합니다.

## 전체 코드 실행

```bash
cargo test -p file_lens --bin gui_workflow_solution
```

전체 코드: `examples/part3/file_lens/src/bin/gui_workflow_solution.rs`
