# 34A. SpatialQuery로 TPS 카메라 충돌 처리하기

## 학습 목표

- 추적 카메라가 벽을 통과하는 원인을 설명합니다.
- Avian의 `SpatialQuery::cast_ray`로 시점과 카메라 사이 장애물을 찾습니다.
- 자기 Collider 제외, 안전 여백과 최소 거리를 적용합니다.

## 이 내용으로 만들 수 있는 것

- 좁은 실내에서도 벽 뒤로 넘어가지 않는 TPS 카메라
- 조준선과 총구 사이 가림을 검사하는 슈팅 게임
- 캐릭터 주변의 시야 확보와 카메라 페이드 시스템

## 이번에 만들 결과물

WASD로 플레이어를 움직이면 원하는 추적 위치까지 ray cast하고 벽이 있으면 카메라를 충돌 지점 앞으로 당깁니다. C 키로 처리를 꺼 벽 관통 상태와 비교할 수 있습니다.

```bash
cargo run -p tps_training --bin camera_collision
```

## 핵심 개념

Camera는 기본적으로 렌더 관찰점일 뿐 물리 Collider가 아닙니다. Camera Entity에 동적 Collider를 붙이면 물리 반동과 떨림이 생길 수 있으므로, 먼저 “원하는 카메라 위치”를 계산하고 그 선분이 막혔는지만 질의하는 방식이 일반적입니다.

```text
focus(플레이어 머리) ───────── desired camera
          │             █ 벽
          └─ ray hit ───┘
               ↑ 실제 camera = hit distance - 안전 여백
```

Ray의 시작점은 플레이어 발이 아니라 카메라가 바라보는 focus 지점이어야 합니다. `SpatialQueryFilter::from_excluded_entities`로 플레이어 자신을 제외하지 않으면 시작점 안의 Player Collider가 가장 먼저 검출될 수 있습니다.

ray는 선이므로 모서리 가까이에서 Camera의 near plane이 벽을 뚫어 보일 수 있습니다. 이 장에서는 작은 안전 여백을 빼고, 더 견고한 구현은 Camera 크기에 해당하는 sphere shape cast로 확장합니다.

## 샘플 코드

```rust
let filter = SpatialQueryFilter::from_excluded_entities([follow.target]);
if let Some(hit) = spatial_query.cast_ray(
    focus,
    direction,
    follow.distance,
    true,
    &filter,
) {
    distance = (hit.distance - CAMERA_RADIUS).max(0.5);
}
camera.translation = focus + backward * distance;
camera.look_at(focus, Vec3::Y);
```

전체 코드는 [34a_camera_collision.rs](source/part5.md#34a--tps-카메라-충돌)에서 확인할 수 있습니다.

## 코드 설명

- `focus`와 `desired camera` 사이만 검사하므로 카메라 뒤 장애물은 관계없습니다.
- 최대 ray 거리는 원래 Camera 거리로 제한해 먼 Collider가 결과에 들어오지 않게 합니다.
- `solid: true`는 시작점이 Collider 안일 때도 내부 충돌을 고려합니다. 그래서 자기 Entity 제외가 중요합니다.
- `hit.distance`에서 Camera 안전 반지름을 빼고 최소 거리를 적용해 focus와 Camera가 같은 위치가 되는 것을 막습니다.
- 충돌이 사라지면 원하는 거리로 즉시 돌아갑니다. 상용 카메라는 복귀 거리에 감쇠를 적용하되 장애물 안쪽으로 들어가는 방향은 즉시 반영합니다.

## 실습 과제

1. 안전 여백을 0과 0.5로 바꿔 벽 가장자리 화면을 비교하세요.
2. 카메라 복귀에만 `smooth_nudge` 또는 보간을 적용하세요.
3. 특정 Collision Layer만 카메라를 막도록 `SpatialQueryFilter`를 구성하세요.

## 심화 과제

Ray cast를 Camera 반지름의 sphere shape cast로 교체하고 모서리에서 near plane 관통이 줄어드는지 비교하세요. 반투명 장식물처럼 카메라를 막지 않아야 하는 Collider 분류도 함께 설계하세요.

[선택한 과제 해설과 수행 예시 보기](exercises/part5/34a_tps_camera_collision.md)

## 다음 챕터

Landmass NavMesh를 생성하고 적 에이전트가 장애물을 피해 플레이어를 추적하게 합니다.
