# 14. 총알 발사

## 학습 목표

- `just_pressed`와 `pressed`의 차이를 구분할 수 있다.
- 실행 중 Commands로 게임 Entity를 생성할 수 있다.
- 속도와 수명을 Component로 분리할 수 있다.

## 이번에 만들 결과물

Space 키를 누를 때마다 우주선 위에서 노란 총알이 발사되고, 일정 시간이 지나면 자동으로 제거됩니다.

```bash
cargo run -p space_survivor --bin 14_bullets
```

## 핵심 개념

총알은 `Bullet`, `Velocity`, `Lifetime`, `HitBox`, `Sprite`, `Transform`의 조합입니다. 이동 System은 Velocity가 있는 모든 Entity를 함께 처리합니다. 총알 전용 이동 함수를 계속 늘리지 않는 것이 ECS 조합의 장점입니다.

수명 제한이 없으면 화면 밖 총알도 World에 계속 남습니다. 보이지 않는 Entity 역시 Query와 스케줄 비용을 만들기 때문에 명확한 제거 정책이 필요합니다.

## 샘플 코드

```rust
fn shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    commands.spawn((
        Bullet,
        Velocity(Vec2::Y * BULLET_SPEED),
        Lifetime(Timer::from_seconds(1.4, TimerMode::Once)),
        Sprite::from_color(Color::srgb(1.0, 0.9, 0.3), Vec2::new(8.0, 20.0)),
        Transform::from_translation(player.translation + Vec3::Y * 34.0),
    ));
}
```

## 코드 설명

- `just_pressed`는 키를 누르기 시작한 프레임에만 참이므로 한 번 누를 때 한 발이 생성됩니다.
- 플레이어 Transform을 읽어 발사 위치를 계산합니다.
- `Velocity(Vec2::Y * BULLET_SPEED)`는 위쪽 속도를 데이터로 저장합니다.
- `TimerMode::Once` Timer는 반복하지 않는 수명을 표현합니다.
- 수명 System은 Timer를 tick하고 끝난 Entity에 `despawn()`을 예약합니다.

연사 무기를 만들 때는 `pressed`와 발사 간격 Timer를 함께 사용해야 합니다. 프레임마다 한 발씩 생성하면 프레임 속도에 따라 발사량이 달라집니다.

## 실습 과제

1. 총알 속도와 크기를 바꾸세요.
2. 총알 수명을 0.5초로 줄이세요.
3. Q 키로 왼쪽 위, E 키로 오른쪽 위 총알을 발사하세요.

## 심화 과제

`FireCooldown(Timer)` Resource를 추가해 Space를 누르고 있는 동안 초당 5발이 발사되도록 구현하세요.

## 다음 챕터

화면 위에서 적을 주기적으로 생성하고 아래쪽으로 이동시킵니다.

