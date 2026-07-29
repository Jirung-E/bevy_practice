# 36. Hierarchy 과제 해설

[본문으로 돌아가기](../../36_Hierarchy.md#실습-과제)

## P6-C36-P1 · 세 번째 Entity

`Editable`과 `Name`을 가진 Entity를 하나 더 만들고 Hierarchy Query에 자동으로 나타나는지 확인합니다. 목록 코드를 새 종류마다 수정해야 한다면 데이터 기반 구조가 아닙니다.

## P6-C36-P2 · 자동 선택

선택이 없고 목록이 비어 있지 않을 때만 첫 Entity를 선택합니다. 사용자가 선택 해제한 상태를 허용할지 여부는 별도 UX 규칙입니다.

## P6-C36-P3 · 정렬

표시용 목록을 수집한 뒤 이름 또는 Entity ID 키로 정렬합니다. World의 spawn 순서를 바꾸지 말고 View 순서만 바꿉니다.

## P6-C36-A1 · 재귀 트리

루트부터 `ChildOf`의 역관계를 따라 재귀 순회하며 깊이만큼 들여씁니다. 수행 예제는 Root → Child → Grandchild가 0, 1, 2단계 들여쓰기로 표시되는지 검사합니다.

Bevy 관계 API는 관계 target과 관계 hook을 통해 부모·자식 일관성을 유지합니다. 에디터 입력에서도 자기 자신이나 자손을 부모로 지정하는 요청을 사전에 거부해야 합니다. 방어적인 visited 집합은 손상된 외부 데이터가 들어와도 무한 재귀를 막습니다.

## 전체 코드 실행

```bash
cargo test -p world_editor --bin editor_model_solution
```

전체 코드: `examples/part6/world_editor/src/bin/editor_model_solution.rs`
