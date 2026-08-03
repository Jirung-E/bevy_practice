# 38. Viewport

## 학습 목표

- 에디터 UI와 3D Camera를 같은 창에 구성할 수 있다.
- 선택 Entity를 Gizmo로 강조할 수 있다.
- Viewport 카메라 상태를 편집 대상과 분리할 수 있다.
- 논리 픽셀, 물리 픽셀과 World 좌표를 변환할 수 있다.

## 이 내용으로 만들 수 있는 것

- Scene과 편집 UI가 함께 보이는 3D 작업 공간
- 선택 테두리와 이동 축 Gizmo가 있는 레벨 편집 화면
- 편집 대상과 독립적으로 움직이는 뷰포트 카메라

## 이번에 만들 결과물

Hierarchy와 Inspector가 차지하지 않는 중앙 영역에만 3D World를 렌더링하고, 선택한 대상에 노란 경계 상자와 축 Gizmo를 표시합니다. 커서를 중앙에 올리면 논리 화면 좌표, World 바닥 좌표와 DPI 배율을 함께 표시합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p world_editor --bin 38_viewport
```

조작:

- 오른쪽 마우스 드래그: Viewport 공전
- 휠: 줌

## 핵심 개념

에디터 카메라는 게임 World를 보기 위한 도구이지 편집 대상이 아닙니다. `EditorCamera` 표식을 사용하고 Editable을 붙이지 않습니다.

Gizmos는 디버그·도구 시각화에 적합한 즉시 모드 선 그리기 API입니다. 매 프레임 선택 Transform을 읽어 축과 경계 상자를 다시 그립니다.

### UI Camera와 World Camera 분리

World Camera의 Viewport를 중앙으로 줄이면 그 Camera가 담당하는 UI도 중앙에 잘릴 수 있습니다. 예제는 다음처럼 역할을 나눕니다.

- `Camera3d`: 중앙 물리 픽셀 Viewport에 World를 렌더링
- `Camera2d`: `order: 1`, 투명 배경으로 전체 창 UI를 렌더링
- UI 루트의 `UiTargetCamera`: UI를 담당할 Camera2d를 명시

여러 Camera가 있을 때 UI 대상을 암묵적으로 두면 어떤 Camera가 UI를 그릴지 모호해질 수 있습니다.

### 논리 픽셀과 물리 픽셀

Bevy UI의 `px(250)`과 `Window::cursor_position()`은 논리 픽셀입니다. 반면 `Camera::viewport`의 `physical_position`과 `physical_size`는 물리 픽셀입니다. DPI 150% 화면에서 논리 폭 250은 물리 폭 375가 됩니다.

```rust
let scale = window.scale_factor();
let left = (HIERARCHY_WIDTH * scale).round() as u32;
let size = UVec2::new(
    window.physical_width().saturating_sub(left + right),
    window.physical_height().saturating_sub(bottom),
);
camera.viewport = Some(Viewport {
    physical_position: UVec2::new(left, 0),
    physical_size: size.max(UVec2::ONE),
    ..default()
});
```

창 크기나 모니터 DPI가 달라지면 물리 크기와 `scale_factor`가 변합니다. 예제는 기대 Viewport를 매 프레임 계산하되 값이 달라졌을 때만 Camera에 반영하므로 창 이동과 리사이즈를 모두 처리합니다.

### 화면 좌표에서 World 좌표로

마우스 위치 자체는 3D 점이 아닙니다. `viewport_to_world`는 Camera 원점에서 커서 방향으로 나가는 ray를 만들고, 이 ray와 편집 바닥 평면의 교점을 구해야 World 좌표가 됩니다.

```rust
let ray = camera.viewport_to_world(camera_transform, cursor)?;
let point = ray.plane_intersection_point(
    Vec3::ZERO,
    InfinitePlane3d::new(Vec3::Y),
)?;
```

커서가 좌우 패널이나 Console 위에 있으면 World 입력을 처리하지 않습니다. Orbit과 Picking도 같은 중앙 논리 rect 판정을 공유해야 UI 조작이 Camera 조작으로 새지 않습니다.

## 샘플 코드

```rust
fn draw_selection_gizmo(
    selection: Res<Selection>,
    selected: Query<&Transform, With<Editable>>,
    mut gizmos: Gizmos,
) {
    let Some(transform) =
        selection.0.and_then(|entity| selected.get(entity).ok())
    else {
        return;
    };

    gizmos.axes(*transform, 1.4);
    gizmos.cube(
        Transform::from_translation(transform.translation)
            .with_scale(Vec3::splat(1.9)),
        Color::srgb(1.0, 0.85, 0.1),
    );
}
```

## 코드 설명

- UI는 3D 렌더 결과 위에 겹쳐 그려져 패널을 구성합니다.
- World와 UI를 서로 다른 Camera가 렌더링하므로 중앙 Viewport를 줄여도 패널은 잘리지 않습니다.
- Orbit Resource는 에디터 카메라만의 yaw, pitch, radius를 보관합니다.
- 선택 Gizmo는 저장되는 게임 데이터가 아닙니다.
- Bevy 0.19에서는 상자 Gizmo 메서드가 `cube`입니다.
- Viewport는 DPI가 반영된 물리 rect이고, 입력 경계는 UI와 같은 논리 rect입니다.
- `viewport_to_world`가 Camera의 Viewport를 반영하므로 수동으로 정규화 좌표를 다시 계산하지 않습니다.

## 실습 과제

1. 선택 상자 색과 크기를 대상 Mesh에 맞게 바꾸세요.
2. F 키로 선택 대상에 카메라 focus를 맞추세요.
3. XZ 바닥 Grid Gizmo를 추가하세요.
4. 창 크기와 Windows 배율을 바꿔 World가 패널 아래로 그려지지 않는지 확인하세요.

## 심화 과제

중앙 패널의 경계를 드래그해 폭을 바꾸는 splitter를 추가하고, 바뀐 논리 rect 하나를 UI Layout, Camera Viewport, Picking 입력 경계가 함께 사용하도록 구성하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part6/38_viewport.md)

## 다음 챕터

Asset Browser의 Cube와 Sphere 버튼으로 새 World Entity를 생성합니다.
