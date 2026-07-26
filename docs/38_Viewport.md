# 38. Viewport

## 학습 목표

- 에디터 UI와 3D Camera를 같은 창에 구성할 수 있다.
- 선택 Entity를 Gizmo로 강조할 수 있다.
- Viewport 카메라 상태를 편집 대상과 분리할 수 있다.

## 이번에 만들 결과물

Hierarchy와 Inspector 뒤에 3D World를 렌더링하고, 선택한 대상에 노란 경계 상자와 축 Gizmo를 표시합니다.

```bash
cargo run -p world_editor --bin 38_viewport
```

조작:

- 오른쪽 마우스 드래그: Viewport 공전
- 휠: 줌

## 핵심 개념

에디터 카메라는 게임 World를 보기 위한 도구이지 편집 대상이 아닙니다. `EditorCamera` 표식을 사용하고 Editable을 붙이지 않습니다.

Gizmos는 디버그·도구 시각화에 적합한 즉시 모드 선 그리기 API입니다. 매 프레임 선택 Transform을 읽어 축과 경계 상자를 다시 그립니다.

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
- Orbit Resource는 에디터 카메라만의 yaw, pitch, radius를 보관합니다.
- 선택 Gizmo는 저장되는 게임 데이터가 아닙니다.
- Bevy 0.19에서는 상자 Gizmo 메서드가 `cube`입니다.
- 실제 에디터는 Camera viewport 영역과 UI 패널 크기를 연동해 3D 렌더 픽셀을 줄입니다.

## 실습 과제

1. 선택 상자 색과 크기를 대상 Mesh에 맞게 바꾸세요.
2. F 키로 선택 대상에 카메라 focus를 맞추세요.
3. XZ 바닥 Grid Gizmo를 추가하세요.

## 심화 과제

카메라 viewport를 중앙 패널의 물리 픽셀 영역으로 제한하고 창 리사이즈·DPI 변경 때 갱신하세요.

## 다음 챕터

Asset Browser의 Cube와 Sphere 버튼으로 새 World Entity를 생성합니다.

