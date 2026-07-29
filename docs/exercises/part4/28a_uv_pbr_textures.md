# 28A. UV와 PBR 텍스처 매핑 과제 해설

[본문으로 돌아가기](../../28A_UvPbrTextures.md#실습-과제)

## P4-C28A-P1 · Normal 맵 비교

### 확인 기준

- `1`과 `2`에서 패널의 사각형 외곽선과 UV 배치는 그대로다.
- `2`에서는 홈, 테두리, 볼트 주변의 명암이 조명 방향에 따라 달라진다.
- Normal 맵은 실루엣이나 충돌 Mesh를 바꾸지 않는다고 설명한다.

## P4-C28A-P2 · UV 반복 횟수

### 힌트

`setup`에서 오른쪽 Mesh를 만드는 한 줄만 변경합니다.

```rust
let uv_five = meshes.add(panel_mesh(5.0));
```

UV가 0에서 5까지 이동하고 Repeat Sampler를 사용하므로 가로와 세로에 각각 다섯 번 나타납니다. Clamp 상태에서는 오른쪽과 아래쪽 가장자리 texel이 넓게 늘어납니다.

## P4-C28A-P3 · Normal 맵과 sRGB

### 접근 방법

실험 전후에 반드시 같은 Map 모드, 주소 모드, 조명 상태를 사용합니다. `NORMAL_REPEAT`을 로드하는 호출의 마지막 인수만 잠시 바꿉니다.

```rust
load_repeat(&asset_server, NORMAL_REPEAT, true)
```

RGB가 색이 아니라 방향 데이터이므로 sRGB 디코딩이 적용되면 중립값과 기울기 값이 달라집니다. 관찰 후 `false`로 복원합니다.

## P4-C28A-A1 · 필터 비교

### 수행 예시

```rust
use bevy::image::ImageFilterMode;

let linear = ImageSamplerDescriptor {
    address_mode_u: ImageAddressMode::Repeat,
    address_mode_v: ImageAddressMode::Repeat,
    mag_filter: ImageFilterMode::Linear,
    min_filter: ImageFilterMode::Linear,
    mipmap_filter: ImageFilterMode::Linear,
    ..default()
};

let nearest = ImageSamplerDescriptor {
    address_mode_u: ImageAddressMode::Repeat,
    address_mode_v: ImageAddressMode::Repeat,
    mag_filter: ImageFilterMode::Nearest,
    min_filter: ImageFilterMode::Nearest,
    mipmap_filter: ImageFilterMode::Nearest,
    ..default()
};
```

### 판단 기준

- 확대 시 Nearest는 texel 경계가 계단처럼 보이고 Linear는 인접 texel을 섞는다.
- 축소 시 mipmap이 없는 고주파 무늬는 깜빡임이 남을 수 있다.
- `mipmap_filter`는 존재하는 mip level 사이의 선택 방식이며 mip level 자체를 생성하지 않는다.
- 사진·PBR 표면에는 보통 Linear를 사용하고, 의도적으로 픽셀 경계를 보존하는 아트에는 Nearest를 검토한다.

## 전체 코드 검증

본문 예제에는 UV 0..1/0..3 Mesh, Clamp/Repeat Sampler, 네 Material 조합이 모두 포함되어 있습니다.

```bash
cargo test -p product_showcase --bin 28a_pbr_textures
cargo clippy -p product_showcase --bin 28a_pbr_textures -- -D warnings
```

전체 코드: `examples/part4/product_showcase/src/bin/28a_pbr_textures.rs`
