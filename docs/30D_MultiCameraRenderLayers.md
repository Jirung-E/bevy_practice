# 30D. 멀티 카메라와 RenderLayers

## 학습 목표

- 하나의 창에 여러 카메라 결과를 합성합니다.
- `Camera::order`, `Viewport`, `ClearColorConfig`의 역할을 구분합니다.
- `RenderLayers`로 카메라마다 보이는 Entity를 선택합니다.

## 이 내용으로 만들 수 있는 것

- 미니맵, 후방 카메라와 화면 속 화면
- 1인칭 무기만 별도 FOV로 그리는 View Model
- 편집기의 장면 Viewport와 오브젝트 미리보기 카메라

## 이번에 만들 결과물

전체 화면의 원근 카메라 위에 탑다운 직교 카메라를 겹칩니다. 미니맵 전용 원형 마커는 오른쪽 위 Viewport에서만 보입니다. 아래 명령은 완성 샘플을 실행합니다.

```bash
cargo run -p product_showcase --bin multi_camera_layers
```

## 핵심 개념

`Camera::order`가 큰 카메라는 나중에 렌더링됩니다. 두 번째 카메라의 `Viewport`를 지정하면 그 영역에만 결과가 기록됩니다. 전체 화면 위에 투명하게 덧그릴 때는 `ClearColorConfig::None`, 독립된 미니맵 배경이 필요하면 `Custom` 색을 사용합니다.

`RenderLayers`는 물리 충돌 Layer나 UI Z 순서가 아니라 렌더링 가시성 비트마스크입니다. Component가 없는 Entity와 Camera는 기본 Layer 0에 속합니다. Camera와 Entity의 Layer가 하나라도 겹칠 때만 그 Camera에 보입니다.

| 대상 | Layer 0 | Layer 1 | 결과 |
|---|---:|---:|---|
| 메인 카메라 | ✓ |  | 일반 월드만 표시 |
| 미니맵 카메라 | ✓ | ✓ | 월드와 마커 표시 |
| 큐브·바닥 | ✓ |  | 두 카메라에 표시 |
| 미니맵 마커 |  | ✓ | 미니맵에만 표시 |

## 샘플 코드

```rust
commands.spawn((
    Camera3d::default(),
    Camera {
        order: 1,
        viewport: minimap_viewport(&window),
        ..default()
    },
    RenderLayers::from_layers(&[0, 1]),
));

commands.spawn((marker_mesh, marker_material, RenderLayers::layer(1)));
```

전체 코드는 [30d_multi_camera_layers.rs](source/part4.md#30d--멀티-카메라와-renderlayers)에서 확인할 수 있습니다.

## 코드 설명

- 메인 Camera는 Component 기본값인 Layer 0만 렌더링합니다.
- 미니맵 Camera는 Layer 0과 1을 함께 보므로 월드 위에 전용 마커가 나타납니다.
- 조명도 렌더 Layer의 영향을 받습니다. 두 Layer의 PBR 물체를 비추려면 Light에도 두 Layer를 지정해야 합니다.
- UI 루트에 `UiTargetCamera(main_camera)`를 붙여 여러 Camera가 있을 때 UI 대상을 명확히 합니다.
- Viewport 크기와 위치는 논리 픽셀이 아닌 물리 픽셀이므로 창 크기가 바뀔 때 다시 계산합니다.

## 실습 과제

1. 미니맵 전용 마커 하나를 메인 화면에서도 보이게 Layer 구성을 바꾸세요.
2. 미니맵 크기를 창의 짧은 변의 30%로 계산하세요.
3. M 키로 미니맵 Camera의 `is_active`를 전환하세요.

## 심화 과제

Render Texture에 별도 카메라 결과를 그린 뒤 3D 모니터 Mesh의 Material에 연결하세요. 화면 Viewport 합성과 Render-to-Texture가 각각 적합한 사용처를 비교하세요.

[선택한 과제 해설과 수행 예시 보기](exercises/part4/30d_multi_camera_layers.md)

## 다음 챕터

Part 5에서는 플레이어, 추적 카메라, 애니메이션, 물리와 NavMesh를 결합해 3D 게임을 만듭니다.
