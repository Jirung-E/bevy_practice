# 27. Camera3d와 3D 좌표

## 학습 목표

- Bevy의 오른손 3D 좌표계를 이해한다.
- Camera3d와 원근 투영 장면을 구성할 수 있다.
- `looking_at`으로 카메라 방향을 지정할 수 있다.

## 이번에 만들 결과물

어두운 3D 창에 Camera3d를 배치하고, 오른쪽 마우스 드래그와 휠로 원점을 공전하는 카메라를 만듭니다.

```bash
cargo run -p product_showcase --bin 27_camera3d
```

## 핵심 개념

Bevy의 3D 공간은 X가 오른쪽, Y가 위쪽이며 카메라는 로컬 -Z 방향을 바라봅니다. Transform은 위치, 회전, 크기를 함께 나타냅니다.

공전 카메라는 초점, yaw, pitch, 반지름을 Resource로 저장합니다. 구면 방향 회전으로 위치를 계산하고 `looking_at(focus, Vec3::Y)`로 초점을 향하게 합니다.

## 샘플 코드

```rust
commands.spawn((
    OrbitCamera,
    Camera3d { ..default() },
    Transform::from_xyz(0.0, 4.0, 9.0)
        .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
));
```

```rust
fn orbit_transform(orbit: &Orbit) -> Transform {
    let rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
    let position = orbit.focus + rotation * Vec3::new(0.0, 0.0, orbit.radius);
    Transform::from_translation(position).looking_at(orbit.focus, Vec3::Y)
}
```

## 코드 설명

- Camera3d는 Bevy 0.19에서 설정 필드를 가진 Component이므로 구조체 기본값 문법을 사용합니다.
- yaw는 Y축 공전, pitch는 위아래 각도입니다.
- pitch를 제한하면 카메라가 극점을 넘어 뒤집히는 현상을 막습니다.
- MouseMotion과 MouseWheel은 MessageReader로 처리합니다.
- 카메라 거리 테스트는 계산된 Transform과 focus 사이 거리를 검사합니다.

## 실습 과제

1. 초기 카메라 반지름을 5와 15로 바꾸세요.
2. 왼쪽 마우스로도 공전하게 하세요.
3. pitch 제한을 제거하고 극점에서 동작을 관찰하세요.

## 심화 과제

마우스 가운데 버튼으로 focus를 카메라의 right/up 방향으로 평행 이동하는 pan 기능을 구현하세요.

## 다음 챕터

Cuboid, Torus, Sphere, Plane 기본 도형을 조합해 제품 모델과 바닥을 만듭니다.

