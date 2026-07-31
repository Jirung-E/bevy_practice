# 20B. 절차적 우주 배경과 UV 애니메이션

## 학습 목표

- 이미지 없이 fragment shader가 화면 무늬를 생성하는 원리를 설명할 수 있다.
- UV를 격자 cell 좌표와 cell 내부 좌표로 나눌 수 있다.
- hash 함수로 cell마다 재현 가능한 값을 만들 수 있다.
- `time` uniform으로 배경을 프레임률과 독립적으로 움직일 수 있다.
- 서로 다른 크기와 속도의 레이어로 시차 효과를 구현할 수 있다.

## 이 내용으로 만들 수 있는 것

- 파일 크기가 거의 늘지 않는 별·먼지·격자 배경
- 시간에 따라 흐르는 물·용암·구름 무늬
- 여러 속도의 레이어가 깊이감을 만드는 무한 스크롤 배경
- seed와 설정값만 저장하면 다시 만들 수 있는 절차적 장면

## 이번에 만들 결과물

Space Survivor 뒤에 사용할 별 배경을 PNG 없이 WGSL만으로 만듭니다. 멀리 있는 작은 별과 가까운 큰 별이 서로 다른 속도로 흐르고, 별마다 밝기가 다르게 반짝입니다. 게임 오브젝트는 별도의 Entity로 배경 앞에 표시됩니다.

```bash
cargo run -p space_survivor --bin procedural_background
```

조작:

- `Space`: shader 시간 정지·재개
- `↑`·`↓`: 스크롤 속도 변경
- `←`·`→`: 별 밀도 변경

별이 멈췄을 때도 무늬가 사라지지 않고, 밀도를 바꿀 때 이미지가 확대되는 것이 아니라 cell 중 별이 나타나는 비율이 달라져야 합니다.

## 핵심 개념

### 전체 화면 Mesh와 fragment shader

배경 Entity는 화면과 같은 크기의 `Rectangle` Mesh 하나입니다. vertex shader는 Bevy 기본 구현을 사용하고, 커스텀 fragment shader가 각 fragment의 UV로 색을 계산합니다.

```text
화면 크기 Rectangle
        ↓ 기본 vertex shader
0..1 UV가 보간된 fragment
        ↓ 20b_starfield.wgsl
배경색 + 먼 별 + 가까운 별
```

CPU는 별 Entity를 만들지 않습니다. 별 하나마다 `Sprite`, `Transform`, `Velocity`가 있는 것이 아니라 같은 fragment 함수가 화면의 모든 위치에서 병렬로 실행됩니다.

### UV를 격자로 나누기

UV에 큰 값을 곱하면 화면을 여러 cell로 나눌 수 있습니다.

```wgsl
let position = uv * grid_size;
let cell_id = floor(position);
let local = fract(position) - vec2<f32>(0.5);
```

- `cell_id`: 현재 fragment가 속한 cell의 정수 좌표
- `local`: cell 중심을 원점으로 한 내부 위치

같은 cell의 fragment는 같은 `cell_id`를 가지지만 `local`은 서로 다릅니다. 따라서 `cell_id`로 별의 존재 여부와 크기를 결정하고, `local`의 중심 거리를 이용해 별의 둥근 밝기를 만들 수 있습니다.

### hash는 난수가 아니다

WGSL 예제의 `hash21`은 같은 `cell_id`를 넣으면 항상 같은 값을 반환합니다.

```wgsl
fn hash21(value: vec2<f32>) -> f32 {
    let mixed = dot(value, vec2<f32>(127.1, 311.7));
    return fract(sin(mixed) * 43758.5453);
}
```

이 함수는 암호학적 난수가 아니라 시각적 패턴에 쓸 의사 난수입니다. 매 프레임 값이 바뀌지 않으므로 별이 위치를 이동할 때 갑자기 다른 모양으로 깜빡이지 않습니다.

### cell 안에 별 그리기

먼저 hash 값이 밀도 기준을 넘는 cell만 선택합니다.

```wgsl
let seed = hash21(cell_id);
let exists = step(1.0 - density, seed);
```

그다음 cell 중심으로부터 거리를 계산하고 `smoothstep`으로 밝기를 줄입니다.

```wgsl
let core = 1.0 - smoothstep(
    radius * 0.2,
    radius,
    length(local),
);
```

이 계산은 원형 Mesh를 추가하지 않고도 fragment 색상만으로 작은 원형 별을 만듭니다. 별이 없는 cell은 `exists`가 0이므로 결과 밝기도 0입니다.

### time uniform과 스크롤

Rust System은 누적 시간을 Material uniform에 기록합니다.

```rust
if !settings.paused {
    settings.elapsed += time.delta_secs();
}
material.options = settings.options();
```

WGSL은 격자를 나누기 전에 UV에 시간 오프셋을 더합니다.

```wgsl
let position =
    aspect_uv * grid_size +
    vec2<f32>(0.0, options.x * options.y * drift);
```

`options.x`는 누적 시간, `options.y`는 사용자가 바꾼 속도, `drift`는 레이어 고유 속도입니다. 프레임 수를 전달하지 않으므로 60 FPS와 144 FPS에서 같은 시간 동안 같은 거리만큼 이동합니다.

### 두 레이어와 시차

같은 함수를 두 번 호출하되 격자 크기, 속도, UV 오프셋을 다르게 사용합니다.

```wgsl
let far_stars = star_layer(input.uv, 24.0, 0.45, options.z * 1.4);
let near_stars = star_layer(
    input.uv + vec2<f32>(options.w, 0.17),
    13.0,
    1.35,
    options.z,
);
```

먼 별은 작고 천천히, 가까운 별은 크고 빠르게 움직입니다. 실제 Z 좌표가 다른 것은 아니지만 화면에서 상대 속도가 달라 깊이감이 생깁니다.

## 샘플 코드

Rust Material은 네 개의 작은 설정값만 GPU로 보냅니다.

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StarfieldMaterial {
    // x: elapsed time, y: scroll speed,
    // z: star density, w: layer separation
    #[uniform(0)]
    options: Vec4,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/20b_starfield.wgsl".into()
    }
}
```

별의 위치 목록이나 이미지 픽셀은 CPU에서 GPU로 보내지 않습니다.

전체 코드:

- Rust: `examples/part2/space_survivor/src/bin/20b_procedural_background.rs`
- WGSL: `examples/part2/space_survivor/assets/shaders/20b_starfield.wgsl`

## 코드 설명

- 배경 Transform의 Z는 `-10`이므로 게임 오브젝트 뒤에 그려집니다.
- `StarfieldSettings`는 사용자가 바꾸는 속도·밀도와 누적 시간을 관리합니다.
- Material 하나를 배경 Entity가 참조하므로 uniform 변경이 그 배경 draw에 적용됩니다.
- `aspect_uv.x`를 1.5배 하여 960×640 화면에서도 별이 지나치게 가로로 늘어지지 않게 합니다.
- `twinkle`은 별마다 다른 seed를 위상과 속도로 사용합니다.
- 별 밝기는 1보다 커질 수 있으므로 밝은 중심이 강조됩니다. 최종 색은 렌더 타깃 형식에 맞춰 표시됩니다.

절차적 효과는 “텍스처를 전혀 사용하지 않는 것이 항상 빠르다”는 뜻이 아닙니다. fragment마다 hash, `sin`, 여러 레이어를 계산합니다. 이미지 메모리와 연산 비용 사이의 선택이므로 GPU profiler로 확인해야 합니다.

## 실습 과제

1. 가까운 별의 `drift`를 `-1.35`로 바꾸어 두 레이어가 반대 방향으로 움직이게 하세요.
2. `grid_size`를 바꾸고 별의 평균 크기와 cell 수가 어떻게 달라지는지 기록하세요.
3. `twinkle` 계산을 제거한 화면과 비교하세요.
4. 세 번째 별 레이어를 추가하되 기존 두 레이어와 크기·속도·색을 다르게 구성하세요.

## 심화 과제

마우스 위치를 `Vec2` uniform으로 전달하고, 커서 주변에서만 별이 바깥쪽으로 밀려나는 왜곡을 구현하세요. CPU가 별 위치를 수정해서는 안 되며 WGSL에서 UV를 변형한 뒤 기존 `star_layer`를 호출해야 합니다.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/20b_procedural_background.md)를 확인하세요.

## 다음 챕터

다음 20C에서는 이 절차적 배경을 실제 이동·사격 장면에 넣고, 적 사망 dissolve와 플레이어 실드 파동을 vertex·fragment shader로 구현합니다.
