# 26. GUI 상태 과제 해설

[본문으로 돌아가기](../../26_GuiState.md#실습-과제)

## P3-C26-P1 · Reading 상태

드롭을 수락하면 `Reading`, 작업 성공이면 `Ready`, 실패면 `Error`로 전환합니다. 작업 시작 전부터 Ready로 표시하면 사용자가 완료 시점을 오해합니다.

## P3-C26-P2 · Error 색상

상태 Text 갱신 System에서 현재 State에 따라 `TextColor`도 함께 바꿉니다. 오류가 해소되어 다른 State로 갈 때 기본색을 반드시 복구합니다.

## P3-C26-P3 · 마지막 선택

벡터 인덱스는 정렬·삭제 뒤 다른 파일을 가리킬 수 있습니다. 파일 항목을 Entity로 관리한다면 `Option<Entity>`, Resource 벡터라면 안정적인 ID나 경로를 선택 키로 쓰는 편이 안전합니다.

## P3-C26-A1 · Resource 벡터와 ECS 항목 비교

| 요구사항 | Resource 벡터 | Entity + Component |
|---|---|---|
| 전체 정렬 | `sort_by` 한 번으로 단순 | 정렬된 별도 View 모델 필요 |
| 단일 선택·삭제 | 인덱스 이동 주의 | Entity로 직접 지정 |
| 조건 검색 | iterator로 충분 | Query 필터와 조합 가능 |
| 항목별 UI 연결 | 매핑을 직접 유지 | 관계/표식 Component 사용 |
| 대량 일괄 저장 | 연속 데이터라 편함 | 수집 단계가 필요 |

수행 예시는 선택 표식을 가진 항목에서 안정적인 ID를 찾습니다. 어느 방식이 항상 우월한 것이 아니라 변경 빈도와 조회 형태에 맞춰 선택합니다.

## 전체 코드 실행

```bash
cargo run -p file_lens --bin gui_workflow_solution
cargo test -p file_lens --bin gui_workflow_solution
```

전체 코드: `examples/part3/file_lens/src/bin/gui_workflow_solution.rs`
