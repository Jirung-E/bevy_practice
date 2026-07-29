# 36. Hierarchy

## 학습 목표

- 편집 가능한 런타임 Entity를 표시할 수 있다.
- Entity ID와 사용자용 이름을 구분할 수 있다.
- 선택 상태를 Resource로 관리할 수 있다.

## 이번에 만들 결과물

왼쪽 Hierarchy 패널에 Blue Cube와 Orange Sphere를 표시하고 Select Next 버튼으로 선택 대상을 순환하며 Delete로 제거합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p world_editor --bin 36_hierarchy
```

## 핵심 개념

에디터가 모든 Bevy Entity를 보여 주면 카메라, UI, 렌더링 내부 Entity까지 섞입니다. 이 프로젝트는 `Editable` 표식이 있는 Entity만 편집 대상으로 노출합니다.

`EditorName(String)`은 사용자에게 보여 줄 이름이고 Entity는 실제 대상을 가리키는 안정적인 런타임 ID입니다. 선택은 `Selection(Option<Entity>)` Resource 하나로 관리해 Hierarchy, Inspector, Viewport가 공유합니다.

## 샘플 코드

```rust
#[derive(Component)]
struct Editable;

#[derive(Component)]
struct EditorName(String);

#[derive(Resource, Default)]
struct Selection(Option<Entity>);
```

```rust
fn update_hierarchy(
    selection: Res<Selection>,
    editables: Query<(Entity, &EditorName), With<Editable>>,
    mut text: Single<&mut Text, With<HierarchyText>>,
) {
    let rows = editables.iter().map(|(entity, name)| {
        let marker = if selection.0 == Some(entity) { ">" } else { " " };
        format!("{marker} {} [{entity:?}]", name.0)
    });
    text.0 = rows.collect::<Vec<_>>().join("\n");
}
```

## 코드 설명

- Editable은 에디터 공개 정책을 명시하는 표식입니다.
- 선택된 Entity는 `>`로 표시합니다.
- 표시 순서를 안정화하기 위해 문자열 행을 정렬합니다.
- 삭제 후 Selection을 None으로 바꿔 유효하지 않은 ID 사용을 막습니다.
- 실제 계층 트리에서는 ChildOf/Children 관계와 접기 상태를 함께 순회해야 합니다.

## 실습 과제

1. 세 번째 Editable Entity를 추가하세요.
2. 선택이 없을 때 첫 Entity를 자동 선택하세요.
3. Entity 이름순과 ID순 정렬 버튼을 만드세요.

## 심화 과제

ChildOf 관계를 재귀적으로 읽어 들여 들여쓰기된 트리를 만들고, 순환 관계가 생길 수 없는 이유를 Bevy 관계 API 관점에서 설명하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part6/36_hierarchy.md)

## 다음 챕터

선택된 Entity의 Transform 값을 Inspector에 표시하고 버튼으로 수정합니다.
