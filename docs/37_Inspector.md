# 37. Inspector

## 학습 목표

- 선택된 Entity의 Component를 조회할 수 있다.
- Inspector 작업을 명령 데이터로 표현할 수 있다.
- 편집과 화면 표시를 분리할 수 있다.

## 이 내용으로 만들 수 있는 것

- 선택한 Entity의 Transform과 Component를 편집하는 패널
- 속성 변경을 명령으로 기록하는 Undo/Redo 기반
- 런타임 데이터를 살펴보고 조정하는 디버그 Inspector

## 이번에 만들 결과물

오른쪽 Inspector에 선택된 이름과 Transform 좌표를 표시하고 X/Y/Z 증감 버튼으로 위치를 편집합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p world_editor --bin 37_inspector
```

## 핵심 개념

Inspector는 Selection의 Entity ID로 필요한 Component를 조회합니다. 해당 Entity가 삭제되었거나 Transform이 없을 수 있으므로 `Query::get`의 Result를 처리합니다.

버튼마다 별도 System을 만드는 대신 `EditorAction` enum Component로 작업을 데이터화합니다. 공통 입력 System은 Pressed 버튼의 Action을 읽고 선택 Entity에 적용합니다.

## 샘플 코드

```rust
#[derive(Component, Clone, Copy)]
enum EditorAction {
    SelectNext,
    Delete,
    MoveX(f32),
    MoveY(f32),
    MoveZ(f32),
}
```

```rust
fn move_selected(
    selected: Option<Entity>,
    transforms: &mut Query<&mut Transform, With<Editable>>,
    delta: Vec3,
) {
    let Some(entity) = selected else { return };
    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.translation += delta;
    }
}
```

## 코드 설명

- EditorAction은 UI 문구가 아니라 도메인 명령을 표현합니다.
- Inspector Text는 World 데이터를 읽고 편집 System은 World를 씁니다.
- System chain으로 편집 후 같은 프레임에 표시 값이 갱신되게 합니다.
- 위치는 0.25 단위로 변경해 결과를 쉽게 확인합니다.
- 범용 Inspector에는 Reflect와 TypeRegistry를 사용해 Component별 코드를 줄일 수 있습니다.

## 실습 과제

1. Scale 조절 Action을 추가하세요.
2. Y축 회전을 15도씩 바꾸는 버튼을 추가하세요.
3. 위치를 소수점 한 자리와 세 자리로 표시하는 옵션을 만드세요.

## 심화 과제

ReflectComponent를 사용해 등록된 Component의 필드 목록을 읽고 f32, Vec3, bool을 공통 위젯으로 편집하는 Inspector를 설계하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part6/37_inspector.md)

## 다음 챕터

3D 카메라, 선택 Gizmo, 공전 조작을 추가해 World를 직접 보는 Viewport를 만듭니다.
