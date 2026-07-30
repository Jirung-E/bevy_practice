# 28A. UV와 PBR 텍스처 매핑

## 학습 목표

- 정점, 인덱스, 법선, UV가 하나의 Mesh에서 맡는 역할을 구분할 수 있다.
- `StandardMaterial`에 Base Color, Normal, Emissive 텍스처를 연결할 수 있다.
- Clamp와 Repeat 주소 모드가 UV 범위 밖의 좌표를 처리하는 방식을 비교할 수 있다.
- 색상 텍스처와 데이터 텍스처의 색 공간을 올바르게 설정할 수 있다.

## 이 내용으로 만들 수 있는 것

- 반복 무늬가 자연스럽게 이어지는 바닥과 벽
- 색상·노멀·발광 지도를 함께 쓰는 SF 패널
- UV 범위를 조절해 텍스처 밀도를 통일한 3D 소품

## 이번에 만들 결과물

같은 PBR 텍스처 세트를 사용하지만 UV 범위가 다른 두 패널을 나란히 표시합니다. 왼쪽은 UV 0..1로 이미지를 한 번 사용하고, 오른쪽은 UV 0..3으로 이미지를 세 번 반복합니다.

- `1`: Base Color만 표시
- `2`: Base Color + Normal
- `3`: Base Color + Emissive
- `4`: 세 맵 모두 표시
- `A`: 오른쪽 패널의 Clamp / Repeat 전환

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 Rust 파일뿐 아니라 `assets/textures/sci_fi_panel`의 이미지도 함께 복사하고 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p product_showcase --bin 28a_pbr_textures
```

## 핵심 개념

### 정점 속성과 인덱스

GPU가 삼각형을 그리려면 최소한 정점 위치와 삼각형을 구성할 순서가 필요합니다. PBR 조명과 텍스처까지 사용하면 다음 데이터가 함께 필요합니다.

| 데이터 | 이 예제에서 하는 일 |
|---|---|
| Position | 패널의 네 모서리 위치 |
| Index | 네 정점을 두 삼각형으로 연결 |
| Normal | 평평한 패널이 카메라 방향인 +Z를 향한다고 지정 |
| UV | 각 정점이 이미지의 어느 위치를 읽을지 지정 |
| Tangent | Normal 맵의 접선 공간 방향을 정의 |

Index는 정점 번호를 재사용해 `[0, 1, 2]`, `[0, 2, 3]` 두 삼각형을 만듭니다. UV는 이미지 픽셀을 직접 저장하지 않고 일반적으로 0..1 범위의 비율을 저장합니다. Bevy의 이미지 UV 원점은 왼쪽 위이며 `(0, 0)`이 왼쪽 위, `(1, 1)`이 오른쪽 아래입니다.

### UV 0..1과 0..3

`panel_mesh(1.0)`은 이미지 전체를 패널에 한 번 배치합니다. `panel_mesh(3.0)`은 같은 표면에서 UV가 3까지 증가하므로 주소 모드에 따라 결과가 달라집니다.

- **Clamp**: 1을 넘은 좌표가 이미지 가장자리의 픽셀에 고정됩니다.
- **Repeat**: UV의 소수 부분을 다시 사용해 이미지가 반복됩니다.

텍스처의 내용과 Mesh의 UV는 독립된 데이터입니다. 같은 PNG를 사용해도 UV가 다르면 화면 결과가 달라집니다.

### PBR 텍스처 세 종류

**Base Color**는 조명을 받기 전 표면의 기본색입니다. 이 예제에서는 비대칭 청록색 L자와 주황색 대각선을 넣어 UV 방향과 반복 횟수를 쉽게 구분합니다.

**Normal**은 실제 정점 위치를 늘리지 않고 픽셀마다 표면 방향을 바꿉니다. 홈과 볼트 주변의 빛 반응이 달라지지만 패널의 외곽선은 변하지 않습니다. Normal 맵을 쓰려면 Mesh에 UV, Normal뿐 아니라 Tangent도 있어야 합니다.

**Emissive**는 조명과 별도로 카메라에 더해지는 색입니다. 검은 픽셀은 빛나지 않고 청록색과 주황색 부분만 보입니다. Emissive는 주변 물체를 비추는 PointLight가 아니므로, 주변을 실제로 밝히려면 별도 Light가 필요합니다.

### 색 공간

Base Color와 컬러 Emissive 이미지는 사람이 보는 색으로 제작되므로 sRGB로 로드합니다. Normal은 RGB 채널을 방향 벡터 데이터로 사용하므로 선형 데이터로 읽어야 합니다.

```rust
settings.is_srgb = false;
```

Normal 맵을 sRGB로 읽으면 채널 값이 감마 변환되어 의도한 방향이 틀어집니다. DirectX 방식으로 제작되어 녹색 채널 방향이 반대인 Normal 맵은 `StandardMaterial::flip_normal_map_y`를 사용하지만, 이 교재의 맵은 기본값에 맞춘 오른손 좌표 방식입니다.

### 필터링과 mipmap

`mag_filter`는 텍스처가 원본보다 크게 보일 때, `min_filter`는 작게 보일 때 인접 texel을 섞는 방식을 정합니다. `Nearest`는 픽셀 경계가 선명하고 `Linear`는 부드럽습니다.

`mipmap_filter`는 여러 mip level 사이를 선택하는 방법입니다. Sampler에 필드를 지정하는 것만으로 원본 PNG에 새 mip level이 생기지는 않습니다. 멀리 있는 대형 텍스처의 깜빡임과 캐시 효율까지 다루는 실전 에셋은 KTX2 같은 형식이나 에셋 파이프라인에서 mipmap을 미리 준비하는 방식을 사용합니다.

### 에셋과 라이선스

이 장의 1254×1254 PBR 텍스처 세트는 OpenAI 이미지 생성을 이용해 이 교재 전용으로 제작했습니다. 외부 로고나 제3자 에셋은 사용하지 않았으며, 생성 프롬프트와 사용 조건은 [Part 4 전체 소스 페이지의 텍스처 라이선스](source/part4.md#텍스처-출처와-라이선스)에 기록했습니다.

Clamp와 Repeat는 같은 이미지 내용에 서로 다른 로더 설정을 사용합니다. Bevy는 하나의 asset path에 하나의 로딩 설정을 연결하므로 예제에서는 내용이 같은 `*_clamp.png` 복사본을 별도 경로로 둡니다.

## 샘플 코드

전체 코드: `examples/part4/product_showcase/src/bin/28a_pbr_textures.rs`

### UV가 있는 Mesh 만들기

```rust
let mut mesh = Mesh::new(
    PrimitiveTopology::TriangleList,
    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
)
.with_inserted_attribute(
    Mesh::ATTRIBUTE_POSITION,
    vec![
        [-1.4, -1.4, 0.0],
        [1.4, -1.4, 0.0],
        [1.4, 1.4, 0.0],
        [-1.4, 1.4, 0.0],
    ],
)
.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
.with_inserted_attribute(
    Mesh::ATTRIBUTE_UV_0,
    vec![
        [0.0, uv_scale],
        [uv_scale, uv_scale],
        [uv_scale, 0.0],
        [0.0, 0.0],
    ],
)
.with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));

mesh.generate_tangents()
    .expect("UV와 법선이 있으므로 tangent 생성에 성공해야 합니다");
```

### Repeat Sampler로 이미지 로드하기

```rust
let image = asset_server
    .load_builder()
    .with_settings(move |settings: &mut ImageLoaderSettings| {
        settings.is_srgb = is_srgb;
        settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        });
    })
    .load(path);
```

### StandardMaterial에 세 맵 연결하기

```rust
let material = materials.add(StandardMaterial {
    base_color_texture: Some(base_color),
    normal_map_texture: Some(normal),
    emissive: LinearRgba::WHITE,
    emissive_texture: Some(emissive),
    metallic: 0.35,
    perceptual_roughness: 0.38,
    ..default()
});
```

## 코드 설명

- `RenderAssetUsages::MAIN_WORLD`는 테스트와 런타임에서 CPU 쪽 Mesh 속성을 계속 조회할 수 있게 합니다.
- `Mesh::ATTRIBUTE_UV_0`은 `StandardMaterial`이 기본으로 읽는 첫 번째 UV 채널입니다.
- `generate_tangents()`는 Position, Normal, UV, Index를 이용해 Normal 맵의 접선 공간을 계산합니다.
- `load_builder().with_settings(...)`는 이미지별 sRGB와 Sampler 설정을 로드 전에 지정합니다.
- Base, Base+Normal, Base+Emissive, All 네 Material Handle을 미리 만들고 키 입력에는 Handle만 교체합니다.
- UV 0..1인 왼쪽 패널은 Clamp와 Repeat가 같은 결과를 냅니다. 주소 모드의 차이는 UV가 범위를 벗어나는 오른쪽 패널에서 확인합니다.

## 실습 과제

1. `1`과 `2`를 번갈아 눌러 Normal 맵이 패널의 외곽선이 아니라 조명 반응만 바꾸는지 확인하세요.
2. 오른쪽 패널을 Repeat로 둔 뒤 `panel_mesh(3.0)`을 `panel_mesh(5.0)`으로 바꾸고 가로·세로 반복 횟수를 확인하세요.
3. `load_repeat`에서 Normal 맵의 `is_srgb`를 잠시 `true`로 바꾸어 조명 결과를 비교한 뒤 반드시 `false`로 복원하세요.

## 심화 과제

Repeat 이미지 전용 Sampler에 `mag_filter`, `min_filter`, `mipmap_filter`를 `Nearest`와 `Linear`로 각각 설정한 두 버전을 만드세요. 카메라 거리를 바꾸며 패널의 선과 볼트가 확대·축소될 때 차이를 기록하고, 단일 mip PNG에서 `mipmap_filter`만 바꿨을 때 차이가 제한적인 이유를 설명하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part4/28a_uv_pbr_textures.md)를 확인하세요.

## 다음 챕터

이미지로 표면의 색과 방향을 표현했으므로, 다음에는 `StandardMaterial`의 금속성·거칠기와 조명의 관계를 집중적으로 다룹니다.
