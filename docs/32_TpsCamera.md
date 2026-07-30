# 32. TPS 추적 카메라

## 학습 목표

- 플레이어를 추적하는 3인칭 카메라를 구현할 수 있다.
- 카메라 회전과 캐릭터 이동 방향을 연결할 수 있다.
- Update와 PostUpdate의 역할을 구분할 수 있다.

## 이 내용으로 만들 수 있는 것

- 마우스로 공전하고 휠로 확대하는 3인칭 카메라
- 플레이어 방향과 카메라 방향이 자연스럽게 연결된 이동
- 벽과 캐릭터 사이 거리를 조절할 수 있는 추적 카메라 기반

## 이번에 만들 결과물

오른쪽 마우스 드래그로 플레이어 주위를 회전하고 휠로 거리를 조절하는 TPS 카메라를 만듭니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p tps_training --bin 32_tps_camera
```

## 핵심 개념

TPS 이동은 월드 고정 방향보다 카메라가 바라보는 방향을 기준으로 하는 편이 자연스럽습니다. 입력 방향에 카메라 yaw 회전을 곱해 XZ 월드 방향을 얻습니다.

카메라 추적은 플레이어 이동과 물리 갱신 뒤의 Transform을 사용해야 합니다. 예제는 PostUpdate에서 카메라를 배치합니다. 실제 물리 보간 프로젝트에서는 Avian 스케줄 및 Transform 전파 순서도 함께 고려합니다.

## 샘플 코드

```rust
let camera_rotation = Quat::from_rotation_y(rig.yaw);
let direction =
    camera_rotation * Vec3::new(input.x, 0.0, -input.y).normalize_or_zero();
```

```rust
fn follow_player(
    rig: Res<CameraRig>,
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, With<FollowCamera>>,
) {
    let target = player.translation + Vec3::Y * 1.4;
    let rotation = Quat::from_euler(EulerRot::YXZ, rig.yaw, rig.pitch, 0.0);
    let position = target + rotation * Vec3::new(0.0, 0.0, rig.distance);
    **camera = Transform::from_translation(position).looking_at(target, Vec3::Y);
}
```

## 코드 설명

- CameraRig Resource는 yaw, pitch, distance를 보관합니다.
- pitch와 거리를 clamp해 뒤집힘과 과도한 줌을 막습니다.
- target을 캐릭터 중심보다 높게 두어 화면 구도를 맞춥니다.
- `Without<Player>` Filter는 서로 다른 Transform 가변 접근을 명확히 구분합니다.
- 카메라 yaw는 렌더링뿐 아니라 이동 기준 좌표계가 됩니다.

## 실습 과제

1. 초기 거리와 pitch를 바꾸세요.
2. 카메라 회전 감도를 Resource로 옮기세요.
3. Q/E 키로 카메라 어깨 방향을 전환하세요.

## 심화 과제

카메라와 플레이어 사이에 장애물이 있으면 SpatialQuery raycast로 거리를 줄여 벽 관통을 막으세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part5/32_tps_camera.md)

## 다음 챕터

이동량을 사용해 시각 모델에 걷기 흔들림을 적용하고 애니메이션 상태 설계를 배웁니다.
