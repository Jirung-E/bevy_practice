# 20C. Space Survivor 실전 셰이더 효과

## 학습 목표

- 게임 사건을 shader uniform 변화로 연결할 수 있다.
- noise와 `discard`로 점진적인 dissolve를 구현할 수 있다.
- dissolve 경계만 별도의 발광색으로 표시할 수 있다.
- vertex shader와 fragment shader를 결합해 에너지 실드를 만들 수 있다.
- 게임 규칙 Entity와 렌더링 효과의 제거 시점을 분리할 수 있다.

## 이 내용으로 만들 수 있는 것

- 적이 불규칙한 조각으로 사라지는 사망 연출
- 피격 지점에서 파동이 퍼지는 보호막
- 얼어붙음·석화·소각처럼 경계가 진행되는 상태 효과
- 게임 사건은 CPU가 결정하고 픽셀 표현은 GPU가 담당하는 렌더링 구조

## 이번에 만들 결과물

완성한 Space Survivor의 이동·사격·충돌 흐름을 셰이더 효과가 포함된 장면으로 다시 구성합니다.

```bash
cargo run -p space_survivor --bin shader_effects
```

조작:

- `WASD` 또는 방향키: 이동
- `Space`: 총알 발사
- `H`: 충돌을 기다리지 않고 실드 impact 확인

총알이 적에게 맞으면 적이 즉시 사라지지 않습니다. noise 패턴을 따라 몸체가 없어지고 경계가 노란색으로 타오른 뒤 Entity가 제거됩니다. 적이 플레이어에 닿거나 `H`를 누르면 원형 실드 Mesh가 흔들리고 안쪽에서 충격파가 퍼집니다.

## 핵심 개념

### CPU가 사건을 결정하고 GPU가 표현한다

충돌 여부와 점수 같은 게임 규칙을 shader가 결정하게 만들면 안 됩니다. GPU 결과를 CPU가 즉시 읽는 것은 비싸고 비동기적입니다.

```text
CPU / ECS
총알-적 AABB 충돌 판정
        ↓
Dissolving(Timer) 추가
        ↓ 매 프레임 progress uniform
GPU / WGSL
noise → discard → 발광 경계
        ↓
Timer 종료 후 CPU가 Entity despawn
```

실드도 같은 원칙입니다. CPU는 적과 플레이어 충돌을 판정해 `ShieldPulse` Timer를 재시작하고, GPU는 전달받은 impact 강도로 Mesh와 픽셀을 변형합니다.

### 즉시 despawn하지 않는 사망 상태

기본 Space Survivor는 총알이 맞은 적을 즉시 제거했습니다. dissolve를 보여 주려면 게임 규칙상 살아 있는 적과 사망 연출 중인 Entity를 구분해야 합니다.

```rust
commands
    .entity(enemy_entity)
    .remove::<(Enemy, Velocity)>()
    .insert(Dissolving(Timer::new(
        Duration::from_secs_f32(0.9),
        TimerMode::Once,
    )));
```

`Enemy`를 제거했으므로 추가 충돌과 이동 Query에서 제외됩니다. 하지만 Mesh와 Material은 남아 GPU가 사망 연출을 계속 그립니다. Timer가 끝난 뒤에만 `despawn`합니다.

### Entity마다 다른 Material Handle

각 적은 서로 다른 시점에 죽으므로 dissolve 진행도도 달라야 합니다. 모든 적이 같은 Material Handle을 공유하면 한 적의 progress를 바꿀 때 살아 있는 적까지 함께 사라집니다.

```rust
let material = materials.add(DissolveMaterial {
    effect: Vec4::new(time.elapsed_secs(), 0.0, 0.075, 0.0),
});
```

예제는 적을 생성할 때마다 작은 uniform을 가진 Material 인스턴스를 만듭니다. 실제 프로젝트에서는 Material 인스턴스 수와 batch 감소를 측정하고, 많은 개별 상태가 필요하면 instance buffer나 storage buffer 구조를 고려합니다.

### value noise

dissolve shader는 두 단계의 value noise를 섞습니다.

```wgsl
let coarse = value_noise(input.uv * 8.0);
let detail = value_noise(
    input.uv * 23.0 +
    vec2<f32>(effect.x * 0.18, 0.0),
);
let noise = coarse * 0.72 + detail * 0.28;
```

낮은 주파수 noise는 큰 덩어리를, 높은 주파수 noise는 거친 가장자리를 만듭니다. `effect.y`는 0에서 1로 증가하는 dissolve 진행도입니다.

### discard와 발광 경계

현재 픽셀의 noise가 진행도보다 작으면 fragment를 버립니다.

```wgsl
let remaining = noise - effect.y;
if remaining < 0.0 {
    discard;
}
```

남아 있는 픽셀 중 `remaining`이 0에 가까운 부분은 방금 사라지기 직전의 경계입니다.

```wgsl
let edge = 1.0 - smoothstep(0.0, effect.z, remaining);
let color = mix(body, glowing_edge, edge);
```

이 결과는 단순 색상 보간과 다릅니다. 픽셀마다 서로 다른 noise 값을 사용해 실제 silhouette가 변하고, `discard`된 영역은 그려지지 않습니다.

### vertex shader 실드 변형

실드 vertex shader는 정점의 각도와 시간을 사용해 원 둘레를 흔듭니다.

```wgsl
let angle = atan2(local_position.y, local_position.x);
let idle_wave = sin(angle * 8.0 - effect.x * 2.5) * 0.012;
let impact_wave =
    sin(angle * 12.0 - effect.x * 9.0) *
    effect.y *
    0.09;
local_position.xy *= 1.0 + idle_wave + impact_wave;
```

`effect.y`가 0이면 작은 idle 파동만 있고, 충돌 직후에는 큰 파동이 원의 실제 vertex 위치를 바꿉니다. `Transform.scale`을 변경한 것이 아니므로 둘레의 각 위치가 서로 다른 양만큼 움직입니다.

### fragment shader 실드 패턴

fragment shader는 UV 중심 거리로 외곽 링을 만들고, 각도로 방사형 구획을 만듭니다. impact가 발생하면 반지름이 변하는 추가 링을 합성합니다.

```wgsl
let impact_ring =
    1.0 - smoothstep(
        0.025,
        0.09,
        abs(radius - (0.25 + (1.0 - effect.y) * 0.58)),
    );
```

실드의 geometry 변형과 내부 빛 패턴이 같은 `effect.y`를 공유하므로 충돌 연출이 하나의 효과처럼 보입니다.

## 샘플 코드

Material마다 필요한 shader와 uniform을 분리합니다.

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DissolveMaterial {
    // x: time, y: dissolve progress, z: edge width
    #[uniform(0)]
    effect: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ShieldMaterial {
    // x: time, y: impact strength
    #[uniform(0)]
    effect: Vec4,
}
```

dissolve 갱신 System은 Timer 비율만 Material에 전달합니다.

```rust
dissolve.0.tick(time.delta());
let progress = dissolve.0.fraction();

if let Some(material) = materials.get_mut(&material_handle.0) {
    material.effect.x = time.elapsed_secs();
    material.effect.y = progress;
}
```

전체 코드:

- Rust: `examples/part2/space_survivor/src/bin/20c_shader_effects.rs`
- 배경: `assets/shaders/20b_starfield.wgsl`
- dissolve: `assets/shaders/20c_dissolve.wgsl`
- 실드: `assets/shaders/20c_shield.wgsl`

## 코드 설명

- 배경은 20B의 같은 WGSL을 재사용합니다.
- 적은 `DissolveMaterial` Handle을 개별 소유합니다.
- `Dissolving` Entity는 `Enemy`와 `Velocity`가 제거되어 게임 규칙에서 제외됩니다.
- `ShieldVisual`은 Player의 위치를 따라가지만 별도의 Mesh와 Material입니다.
- `ShieldPulse`는 충돌 직후 1에서 0으로 감소하는 impact 값을 만듭니다.
- `H` 입력은 충돌 재현을 기다리지 않고 shader 결과를 반복 관찰하기 위한 디버그 입력입니다.
- UI는 일반 Bevy Text이므로 배경과 효과 Material의 영향을 받지 않습니다.

현재 예제는 교육을 위해 Material 인스턴스와 Mesh를 단순하게 생성합니다. 실제 게임에서는 적 Mesh Handle을 공유하고, 개별 효과값을 instance 데이터로 전달하는 최적화가 가능합니다.

## 실습 과제

1. dissolve 시간을 0.3초와 2초로 바꾸고 경계 이동 속도를 비교하세요.
2. `effect.z` 경계 폭을 바꾸어 얇은 불꽃과 넓은 불꽃을 만드세요.
3. 실드 vertex shader의 파동 횟수 8과 12를 각각 바꾸어 silhouette를 비교하세요.
4. `H`를 연속 입력했을 때 impact Timer가 처음부터 다시 시작하는지 확인하세요.

## 심화 과제

적 종류에 따라 소각·빙결 두 dissolve Material을 선택할 수 있도록 효과 설정을 데이터로 분리하세요. 충돌 System에는 색상이나 noise 상수를 넣지 말고, 사망 원인 Component 또는 Message만 기록해야 합니다.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/20c_shader_effects.md)를 확인하세요.

## 다음 챕터

다음 20D에서는 별 배경, dissolve, 실드 WGSL을 앱 실행 중 수정하고 shader asset의 재로딩과 pipeline 오류 복구를 실습합니다.
