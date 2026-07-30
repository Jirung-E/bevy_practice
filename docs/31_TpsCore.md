# 31. TPS 플레이어 기초

## 학습 목표

- 3D 입력을 월드 이동 방향으로 변환할 수 있다.
- 시각 모델과 게임 플레이 루트를 분리할 수 있다.
- TPS 훈련장 프로젝트의 구조를 이해한다.

## 이 내용으로 만들 수 있는 것

- 카메라 기준으로 움직이는 3인칭 캐릭터
- 이동 로직과 시각 모델을 분리한 플레이어 구조
- TPS 전투·탐험 게임의 기본 훈련장

## 이번에 만들 결과물

WASD로 캡슐 캐릭터를 움직이는 3D 훈련장을 만듭니다. 바닥, 장애물, 조명, 고정 카메라가 함께 배치됩니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p tps_training --bin 31_tps_core
```

## 핵심 개념

플레이어 Entity는 위치와 게임 플레이 Component를 가진 루트이며, 렌더링 Mesh는 자식 Entity입니다. 이 구조는 물리 Collider 크기와 모델 Transform을 독립적으로 조절하고 애니메이션을 시각 모델에만 적용하게 합니다.

입력 Vec2를 XZ 평면의 Vec3로 바꾸고 정규화해 대각선 속도를 맞춥니다. 이동 방향이 있으면 캐릭터의 Y축 회전을 바꿔 진행 방향을 바라보게 합니다.

## 샘플 코드

```rust
let direction = Vec3::new(input.x, 0.0, -input.y).normalize_or_zero();

if direction.length_squared() > 0.0 {
    transform.rotation =
        Quat::from_rotation_y(direction.x.atan2(direction.z));
}
transform.translation += direction * PLAYER_SPEED * time.delta_secs();
```

플레이어 생성:

```rust
commands.spawn((
    Player,
    MotionAmount::default(),
    Transform::from_xyz(0.0, 1.0, 4.0),
    Visibility::default(),
    children![(PlayerVisual, Mesh3d(mesh), MeshMaterial3d(material))],
));
```

## 코드 설명

- 게임 규칙은 Player 루트에, 그림은 PlayerVisual 자식에 둡니다.
- `atan2(direction.x, direction.z)`는 이동 벡터를 Y축 각도로 바꿉니다.
- MotionAmount는 애니메이션 System과 이동 System 사이의 작은 인터페이스입니다.
- 앞 단계에서는 Transform을 직접 이동하며 34장에서 물리 속도로 교체합니다.
- 훈련장 경계는 학습용 clamp로 제한합니다.

## 실습 과제

1. 이동 속도와 시작 위치를 바꾸세요.
2. Shift를 누르면 달리도록 만드세요.
3. 캐릭터 모델을 Cuboid와 Sphere 조합으로 바꾸세요.

## 심화 과제

걷기, 달리기, 정지 속도를 담은 MovementSettings Resource를 만들고 입력 System에서 상수를 제거하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part5/31_tps_core.md)

## 다음 챕터

카메라 yaw를 기준으로 입력 방향을 회전시켜 화면 기준 TPS 조작을 구현합니다.
