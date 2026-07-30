# 30C. 3D 오브젝트 마우스 선택

## 학습 목표

- 화면의 마우스 위치가 3D 공간의 광선으로 변환되는 과정을 설명할 수 있습니다.
- `MeshPickingPlugin`과 Pointer observer로 3D Entity를 선택할 수 있습니다.
- 선택 가능 대상, 배경, UI 입력을 구분할 수 있습니다.

## 이 내용으로 만들 수 있는 것

- 월드 에디터에서 Viewport의 오브젝트를 클릭해 Inspector에 표시할 수 있습니다.
- 전략 게임에서 유닛을 선택하거나 어드벤처 게임에서 상호작용 대상을 가리킬 수 있습니다.
- 배치 게임에서 건물·아이템을 클릭해 이동, 회전, 삭제 도구를 연결할 수 있습니다.

## 이번에 만들 결과물

큐브와 구에 마우스를 올리면 청록색으로 바뀌고, 왼쪽 버튼으로 클릭하면 노란색 선택 상태가 유지됩니다. 바닥을 클릭하면 선택이 해제됩니다.

```bash
cargo run -p product_showcase --bin 30c_object_picking
```

## 핵심 개념

화면의 한 픽셀만으로는 3D 위치를 정할 수 없습니다. Camera는 마우스 화면 좌표를 Camera 원점에서 장면 안쪽으로 나아가는 **ray**로 바꿉니다. Picking backend는 이 ray와 Mesh 삼각형의 교차점을 찾고 거리가 가장 가까운 hit를 Pointer 이벤트의 대상으로 정합니다.

```text
마우스 화면 좌표
    ↓ Camera projection 역변환
3D ray
    ↓ Mesh 교차 검사
가까운 hit Entity
    ↓ Pointer<Over> / Pointer<Click>
게임의 Selection 상태
```

`MeshPickingPlugin`은 입문과 디버깅에 편리한 CPU Mesh raycast backend입니다. 물리 Collider를 이미 사용하는 게임은 물리 엔진의 raycast를 선택 판정과 공유할 수 있고, 매우 큰 장면은 공간 분할이나 GPU Picking을 검토합니다.

## 샘플 코드

```rust
App::new()
    .insert_resource(MeshPickingSettings {
        require_markers: true,
        ..default()
    })
    .add_plugins((DefaultPlugins, MeshPickingPlugin))
    .add_observer(handle_click);
```

```rust
commands.spawn((
    SelectableObject { name: "Blue Cube" },
    Pickable::default(),
    Mesh3d(cube),
    MeshMaterial3d(material),
));
```

```rust
fn handle_click(
    click: On<Pointer<Click>>,
    mut selection: ResMut<Selection>,
    objects: Query<(), With<SelectableObject>>,
) {
    if click.button == PointerButton::Primary && objects.contains(click.entity) {
        selection.0 = Some(click.entity);
    }
}
```

선택하지 않을 바닥을 완전히 무시하려면 `Pickable::IGNORE`를 붙입니다. 이 예제는 바닥 클릭으로 선택을 해제해야 하므로 `SelectionBackground`와 `Pickable::default()`를 붙인 뒤 별도로 처리합니다.

## 코드 설명

- `require_markers: true`는 `Pickable`을 붙인 Mesh만 검사해 선택 대상 범위를 명시합니다.
- Pointer observer의 `click.entity`는 가장 가까운 hit Entity입니다. 뒤에 겹친 Mesh를 직접 정렬할 필요가 없습니다.
- `Pointer<Over>`와 `Pointer<Out>`은 hover 재질을 바꿉니다.
- 선택 상태는 `Selection` Resource에 저장하므로 Inspector나 gizmo System도 같은 Entity를 읽을 수 있습니다.
- 선택 재질과 hover 재질을 분리해 마우스가 빠져나가도 선택 표시가 사라지지 않게 합니다.
- 화면 전체 UI root에는 `Pickable::IGNORE`를 사용하되 실제 Button은 기본 Picking을 유지할 수 있습니다. UI가 클릭을 소비하면 뒤의 3D Mesh까지 이벤트가 전달되지 않습니다.

빈 공간 해제 정책은 프로젝트마다 다릅니다. 이 예제는 넓은 바닥을 배경 hit 대상으로 사용합니다. 바닥 밖의 진짜 빈 화면까지 처리해야 한다면 마우스 버튼 입력 시 `PointerInteraction::get_nearest_hit()`가 없는 경우를 검사하세요.

## 실습 과제

1. 세 번째 오브젝트를 추가하고 `Pickable`을 붙여 선택되는지 확인하세요.
2. 한 오브젝트에서 `Pickable`을 제거해 선택 대상에서 제외하세요.
3. 선택된 Entity의 이름과 위치를 화면 상태 문구에 함께 표시하세요.

## 심화 과제

`PointerInteraction`의 가장 가까운 hit 위치와 법선을 gizmo로 표시하고, Shift를 누른 상태에서는 여러 Entity를 선택하는 `HashSet<Entity>` 기반 다중 선택으로 확장하세요.

[선택한 과제 해설과 수행 예시 보기](exercises/part4/30c_object_picking.md)

## 다음 챕터

Part 5에서는 지금까지 만든 3D 장면에 플레이어, 카메라, 애니메이션, 물리와 NavMesh를 결합합니다. Part 6의 Viewport에서는 같은 Picking 흐름을 Hierarchy와 Inspector 선택에 연결합니다.
