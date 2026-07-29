# 15. 적 생성과 이동

## 학습 목표

- 반복 Timer로 생성 주기를 제어할 수 있다.
- 같은 이동 System을 여러 Entity 종류에 재사용할 수 있다.
- 결정적으로 재현 가능한 생성 위치를 만들 수 있다.

## 이번에 만들 결과물

0.9초마다 화면 위쪽에서 빨간 적이 등장해 아래로 내려옵니다. 이동과 총알 발사가 함께 작동합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 15_enemies
```

## 핵심 개념

적 생성기는 `EnemySpawnTimer` Resource로 게임 전체의 생성 간격을 관리합니다. 예제는 외부 난수 의존성 없이 `SpawnSequence`와 사인 함수를 사용해 넓게 퍼진 X 좌표를 만듭니다. 같은 실행은 같은 패턴을 만들므로 디버깅과 실습 결과 재현이 쉽습니다.

## 샘플 코드

```rust
fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut sequence: ResMut<SpawnSequence>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    sequence.0 += 1;
    let x = ops::sin(sequence.0 as f32 * 2.17) * 445.0;
    commands.spawn((
        Enemy,
        Velocity(Vec2::NEG_Y * ENEMY_SPEED),
        Lifetime(Timer::from_seconds(6.0, TimerMode::Once)),
        Sprite::from_color(Color::srgb(1.0, 0.25, 0.35), Vec2::splat(38.0)),
        Transform::from_xyz(x, 345.0, 1.0),
    ));
}
```

## 코드 설명

- 반복 Timer는 완료 뒤 자동으로 다음 주기를 시작합니다.
- `just_finished()`는 이번 tick에서 완료 경계를 넘었는지 알려 줍니다.
- 순번 기반 좌표는 진짜 난수는 아니지만 다양한 위치와 재현성을 함께 제공합니다.
- 적과 총알 모두 Velocity가 있으므로 `move_dynamic_entities` 하나가 이동시킵니다.
- Lifetime은 창 아래로 지나간 적을 정리하는 안전장치입니다.

## 실습 과제

1. 생성 간격을 0.4초로 줄이세요.
2. 적 속도가 순번에 따라 달라지게 하세요.
3. 크기가 큰 적을 매 다섯 번째 순서에 생성하세요.

## 심화 과제

시간이 지날수록 생성 간격과 이동 속도가 단계적으로 증가하는 `Difficulty` Resource를 설계하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/15_enemies.md)를 확인하세요.

## 다음 챕터

총알과 적, 플레이어와 적의 사각형이 겹치는지 검사해 점수와 체력을 변경합니다.
