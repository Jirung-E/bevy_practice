# 34. Avian 3D 물리

## 학습 목표

- Bevy 0.19용 Avian PhysicsPlugins를 등록할 수 있다.
- RigidBody와 Collider의 역할을 구분할 수 있다.
- Transform 직접 이동을 LinearVelocity 제어로 바꿀 수 있다.

## 이번에 만들 결과물

플레이어가 중력의 영향을 받고 바닥과 장애물에 충돌하며 Space로 점프하는 물리 기반 TPS 훈련장을 만듭니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p tps_training --bin 34_physics
```

## 핵심 개념

Bevy 코어는 물리 엔진을 내장하지 않습니다. 이 교재는 Bevy 0.19과 호환되는 Avian 0.7을 사용합니다.

- Static: 움직이지 않는 바닥과 벽
- Dynamic: 힘, 중력, 충돌 해결의 영향을 받는 플레이어
- Kinematic: 게임 로직이 속도를 정하지만 다른 Collider와 상호작용하는 AI

Collider는 렌더링 Mesh와 별도입니다. 단순 Capsule과 Cuboid Collider가 복잡한 시각 Mesh보다 빠르고 안정적입니다.

## 샘플 코드

```rust
App::new().add_plugins((DefaultPlugins, PhysicsPlugins::default()));
```

```rust
commands.entity(player).insert((
    RigidBody::Dynamic,
    Collider::capsule(0.45, 1.0),
    LinearVelocity::ZERO,
    LockedAxes::ROTATION_LOCKED,
    Friction::new(0.0),
));
```

```rust
velocity.x = direction.x * PLAYER_SPEED;
velocity.z = direction.z * PLAYER_SPEED;
if jump_requested && is_near_ground {
    velocity.y = 6.5;
}
```

## 코드 설명

- PhysicsPlugins는 고정 시간 단계의 충돌 탐지와 해결 System을 추가합니다.
- 회전을 잠가 캡슐이 충돌로 넘어지지 않게 합니다.
- XZ 속도만 덮어써 중력과 점프가 만든 Y 속도는 보존합니다.
- Friction 0은 이동 입력과 지면 마찰이 싸워 생기는 조작 지연을 줄입니다.
- 현재 지면 검사는 높이 기반 입문 구현이며 실전에서는 shape cast 또는 접촉 법선을 사용해야 합니다.

## 실습 과제

1. 중력과 점프 속도를 조절하세요.
2. 장애물 크기와 Collider 크기를 다르게 해 보세요.
3. 물리 디버그 렌더링으로 Collider를 표시하세요.

## 심화 과제

Avian SpatialQuery의 shape cast로 지면을 감지하고 경사면 최대 각도, 계단 오르기, 공중 제어를 갖춘 캐릭터 컨트롤러로 발전시키세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part5/34_physics.md)

## 다음 챕터

bevy_landmass NavMesh 위에 적 에이전트를 만들고 플레이어를 경로 목표로 지정합니다.
