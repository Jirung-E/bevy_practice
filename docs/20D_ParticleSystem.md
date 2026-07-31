# 20D. ECS로 만드는 2D 파티클 시스템

## 학습 목표

- emitter와 particle의 역할을 구분할 수 있다.
- burst 방출과 continuous 방출을 각각 구현할 수 있다.
- 속도·가속도·수명으로 입자의 움직임을 갱신할 수 있다.
- 정규화된 수명으로 색상·투명도·크기를 보간할 수 있다.
- 수명이 끝난 Entity를 제거하고 대량 효과의 비용을 판단할 수 있다.

## 이 내용으로 만들 수 있는 것

- 총알 충돌 지점에서 한 번에 퍼지는 폭발과 불꽃
- 우주선 뒤에서 일정한 간격으로 생성되는 추진 궤적
- 먼지·눈·비처럼 일정 영역에서 계속 생성되는 환경 효과
- 회복·레벨업·아이템 획득을 알리는 짧은 시각 피드백

## 이번에 만들 결과물

완성된 Space Survivor의 시각 효과를 별도 실습 장면에서 구현합니다. 청록색 기체를 움직이면 추진 파티클이 이어지고, `Space`를 누르면 기체 앞에서 원형 폭발이 발생합니다. 왼쪽 아래의 `PARTICLES` 값으로 살아 있는 입자 수를 확인할 수 있습니다.

- `WASD` 또는 방향키: 기체 이동과 continuous 추진 파티클
- `Space`: 36개 입자를 한 번에 만드는 burst

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다.

```bash
cargo run -p space_survivor --bin particle_system
```

이 예제에는 이미지 에셋이나 외부 파티클 플러그인이 필요하지 않습니다. 각 입자는 색이 있는 `Sprite` Entity이므로 생성부터 제거까지 ECS 흐름을 그대로 관찰할 수 있습니다.

## 핵심 개념

### emitter와 particle

파티클 시스템은 하나의 거대한 효과가 아니라 **방출 규칙**과 **수명이 짧은 입자 여러 개**의 조합입니다.

```text
입력·Timer·게임 이벤트
        ↓
Emitter가 Particle Entity 생성
        ↓
속도와 가속도로 위치 갱신
        ↓
수명 비율로 색·알파·크기 변경
        ↓
수명이 끝나면 despawn
```

emitter는 “언제, 어디서, 몇 개를, 어떤 초기값으로 만들 것인가”를 결정합니다. particle은 생성된 뒤 자신의 속도·가속도·나이·수명·색·크기를 가집니다. 둘을 분리하면 같은 particle 갱신 System을 폭발, 추진기, 먼지에 재사용할 수 있습니다.

### burst와 continuous 방출

burst는 충돌이나 폭발처럼 한 사건에서 여러 입자를 동시에 만듭니다. 이번 예제는 `Space`를 누를 때 36개 방향을 원 둘레에 균등하게 배치합니다.

```rust
for index in 0..PARTICLE_COUNT {
    let angle = TAU * index as f32 / PARTICLE_COUNT as f32 + phase;
    let direction = Vec2::from_angle(angle);
    let speed = 150.0 + (index % 6) as f32 * 22.0;

    // direction * speed를 초기 velocity로 가진 Particle을 spawn한다.
}
```

무작위 값 없이도 각도·속도·수명을 조금씩 다르게 하면 반복 무늬가 덜 보입니다. `phase`를 burst마다 바꾸면 다음 폭발은 방향이 약간 회전합니다. 결과가 재현되므로 테스트와 녹화에도 유리합니다.

continuous 방출은 추진기나 연기처럼 일정한 시간 간격으로 입자를 만듭니다. 프레임마다 한 개씩 생성하면 60 FPS와 144 FPS에서 방출량이 달라집니다. `Timer`를 사용하면 초당 생성량을 프레임률과 분리할 수 있습니다.

```rust
if moving && thruster_timer.0.tick(time.delta()).just_finished() {
    // 기체 아래에서 세 개의 추진 입자를 생성한다.
}
```

### Particle Component

입자 하나에 필요한 시뮬레이션 값을 Component로 묶습니다.

```rust
#[derive(Component)]
struct Particle {
    velocity: Vec2,
    acceleration: Vec2,
    age: f32,
    lifetime: f32,
    start_color: Vec4,
    end_color: Vec4,
    start_size: f32,
    end_size: f32,
}
```

- `velocity`: 현재 이동 방향과 초당 거리
- `acceleration`: 중력·바람처럼 매초 velocity를 바꾸는 값
- `age`, `lifetime`: 생성 후 경과 시간과 제거 시점
- `start_color`, `end_color`: 태어날 때와 사라질 때의 RGBA
- `start_size`, `end_size`: 수명 동안 변할 크기

충돌 판정, 점수, 저장 데이터 같은 게임 규칙은 넣지 않습니다. 파티클은 결과를 보여 주는 presentation 데이터입니다. 효과가 사라져도 게임 상태가 달라지지 않아야 합니다.

### 프레임 독립적인 이동

`velocity`의 단위가 초당 픽셀이라면 프레임에서 이동할 거리는 `velocity * delta_secs`입니다. 가속도를 먼저 속도에 반영한 뒤 위치를 갱신합니다.

```rust
let delta = time.delta_secs();
particle.velocity += particle.acceleration * delta;
transform.translation += (particle.velocity * delta).extend(0.0);
```

`delta`를 곱하지 않으면 빠른 PC에서 입자가 더 빨리 이동합니다. 파티클은 시각 효과이므로 보통 `Update`의 가변 시간으로 충분합니다. 물리 판정과 정확히 일치해야 하는 입자는 게임 규칙 Entity로 처리하고, 그 결과만 파티클로 보여 주는 편이 안전합니다.

### 정규화된 수명과 보간

서로 수명이 다른 입자를 같은 식으로 제어하기 위해 현재 나이를 `0.0..=1.0`으로 바꿉니다.

```rust
let t = (particle.age / particle.lifetime).clamp(0.0, 1.0);
let color = particle.start_color.lerp(particle.end_color, t);
let size = particle.start_size.lerp(particle.end_size, t);
```

- `t = 0.0`: 방금 생성됨
- `t = 0.5`: 수명의 절반
- `t = 1.0`: 제거 시점

이번 폭발은 노란색에서 붉은색으로 변하면서 alpha가 0이 되고 크기가 작아집니다. 같은 `t`를 사용하므로 움직임과 시각 변화가 입자의 수명에 맞춰 함께 끝납니다.

### 제거와 Entity 수 확인

수명이 끝난 입자를 남겨 두면 화면에서는 투명해도 Query와 World에는 계속 존재합니다.

```rust
particle.age += delta;
if particle.age >= particle.lifetime {
    commands.entity(entity).despawn();
    continue;
}
```

예제의 `PARTICLES` 표시는 현재 `Particle` Entity 수를 보여 줍니다. 이동을 멈추고 burst를 누르지 않았을 때 잠시 후 0으로 돌아와야 제거가 정상적으로 동작한 것입니다.

### CPU ECS 파티클과 GPU 파티클

이번 방식은 입자 하나가 Entity 하나인 CPU 파티클입니다.

장점:

- Query와 Component만으로 동작해 원리를 이해하기 쉽습니다.
- 게임 이벤트, Transform, 색상과 연결하기 쉽습니다.
- 수십~수백 개의 중요한 효과를 세밀하게 제어하기 좋습니다.

한계:

- 입자마다 Entity 생성·제거와 Transform 갱신 비용이 발생합니다.
- 수만 개의 눈·연기·불꽃에는 CPU와 draw 준비 비용이 커질 수 있습니다.

먼저 profiler로 비용을 측정하세요. 생성·제거가 병목이면 비활성 Entity를 재사용하는 pool을 고려하고, 훨씬 많은 입자가 필요하면 GPU compute 기반 파티클 플러그인이나 직접 작성한 GPU buffer 방식을 선택합니다. GPU 방식에서도 emitter, 초기값, 수명, 보간이라는 개념은 그대로 유지됩니다.

## 샘플 코드

입자를 만드는 함수는 시각 Entity와 시뮬레이션 Component를 함께 생성합니다.

```rust
fn spawn_particle(commands: &mut Commands, position: Vec2, particle: Particle) {
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::splat(PARTICLE_BASE_SIZE)),
        Transform::from_xyz(position.x, position.y, 0.0),
        particle,
    ));
}
```

갱신 System은 모든 입자에 같은 생명주기를 적용합니다.

```rust
fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    let delta = time.delta_secs();

    for (entity, mut particle, mut transform, mut sprite) in &mut particles {
        particle.age += delta;
        if particle.age >= particle.lifetime {
            commands.entity(entity).despawn();
            continue;
        }

        let acceleration = particle.acceleration;
        particle.velocity += acceleration * delta;
        transform.translation += (particle.velocity * delta).extend(0.0);

        let (color, size) = particle_visual(&particle);
        sprite.color = Color::srgba(color.x, color.y, color.z, color.w);
        transform.scale = Vec3::splat(size / PARTICLE_BASE_SIZE);
    }
}
```

전체 코드는 `examples/part2/space_survivor/src/bin/20d_particle_system.rs`에서 확인할 수 있습니다.

## 코드 설명

- `ThrusterTimer`는 continuous emitter의 방출 간격을 Resource로 관리합니다.
- `BurstSequence`는 폭발마다 방향 위상을 바꾸되 실행 결과를 재현할 수 있게 합니다.
- `Vec2::from_angle`은 각도를 단위 방향 벡터로 바꿉니다.
- `normalize_or_zero`는 대각선 이동 속도가 빨라지는 문제를 막습니다.
- `Sprite`의 기본 크기는 그대로 두고 `Transform::scale`로 수명에 따른 크기를 표현합니다.
- alpha가 0이 되기 전에 수명이 끝나며, 다음 Command 적용 시 Entity가 제거됩니다.
- 색상과 크기를 계산하는 `particle_visual`은 렌더러 없이 단위 테스트할 수 있는 순수 함수입니다.

입자끼리 순서가 중요한 반투명 효과라면 Z 좌표와 blending 결과도 확인해야 합니다. 이번 예제에서는 모든 파티클이 동일한 Z에 있고 서로 비슷한 작은 사각형이므로 생성 순서 차이가 결과에 큰 영향을 주지 않습니다.

## 실습 과제

1. burst의 `PARTICLE_COUNT`를 12, 72로 바꾸고 화면의 밀도와 `PARTICLES` 최댓값을 비교하세요.
2. `acceleration`의 Y를 양수로 바꾸어 불꽃이 위로 휘는지 확인하세요.
3. 추진 입자의 `Timer` 간격을 0.02초와 0.1초로 바꾸고 프레임률이 아니라 시간 기준으로 방출되는지 확인하세요.
4. 폭발 입자의 시작·끝 색과 크기를 바꾸어 얼음 폭발을 만드세요.

## 심화 과제

현재 입자를 제거하는 대신 `ActiveParticle` marker를 제거하고 숨긴 뒤, 다음 방출 때 재사용하는 간단한 pool을 설계하세요. 먼저 동시에 살아 있던 입자의 최댓값을 기록하고 그 수를 기준으로 pool 용량을 정하세요. pool이 가득 찼을 때 오래된 입자를 재사용할지 새 효과를 생략할지도 명시하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/20d_particle_system.md)를 확인하세요.

## 다음 챕터

Part 2의 게임과 2D 렌더링 심화 과정이 끝났습니다. 다음 Part 3에서는 Bevy ECS와 UI를 사용해 게임이 아닌 GUI 애플리케이션을 제작합니다.
