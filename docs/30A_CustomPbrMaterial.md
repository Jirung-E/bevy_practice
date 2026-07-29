# 30A. 커스텀 Material과 PBR 셰이더

## 학습 목표

- `ExtendedMaterial`과 `MaterialExtension`으로 `StandardMaterial`을 확장할 수 있다.
- Rust의 `AsBindGroup` 필드와 WGSL의 `group`·`binding`을 연결할 수 있다.
- 정점 셰이더와 프래그먼트 셰이더를 교체하면서도 Bevy의 PBR 조명을 유지할 수 있다.
- 시간, 색상, 텍스처 파라미터를 런타임에 변경하고 셰이더 오류를 진단할 수 있다.

## 이번에 만들 결과물

왼쪽에는 기본 `StandardMaterial`, 오른쪽에는 같은 PBR 재질을 확장한 커스텀 Material 구체를 배치합니다. 오른쪽 구체만 표면이 움직이고 마스크 텍스처 영역의 색과 발광이 맥동합니다.

- `V`: 정점 변형 켜기/끄기
- `F`: 색상·발광 효과 켜기/끄기
- `C`: Cyan, Orange, Magenta 색상 순환

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 보며 별도 프로젝트를 만드는 경우에는 Rust 파일과 `assets/shaders`, `assets/textures`를 현재 프로젝트 구조에 맞게 복사하세요.

```bash
cargo run -p product_showcase --bin 30a_custom_pbr_material
```

## 핵심 개념

### Part 2를 읽지 않았다면

셰이더 연결 과정은 다음 한 줄로 요약할 수 있습니다.

```text
Rust Material 데이터 → AsBindGroup의 binding 번호 → WGSL의 같은 binding → GPU 처리
```

2D에서 다룬 [렌더링 파이프라인](13B_RenderingPipeline.md), [WGSL과 셰이더 연결](13C_WgslShader.md), [Material2d 이펙트](13D_Material2dEffects.md)를 먼저 읽으면 셰이더 단계와 바인딩을 더 자세히 복습할 수 있습니다. 이 장에서도 필요한 내용을 다시 설명하므로 Part 2 전체를 먼저 읽을 필요는 없습니다.

### StandardMaterial을 버리지 않고 확장하기

`ExtendedMaterial<Base, Extension>`은 기존 재질을 `base`로 유지하고 사용자 데이터를 `extension`으로 덧붙입니다.

```rust
type CustomPbrMaterial = ExtendedMaterial<StandardMaterial, PulseExtension>;
```

따라서 Base Color, Normal Map, metallic, roughness 같은 PBR 입력은 `StandardMaterial`이 계속 담당합니다. 커스텀 프래그먼트 셰이더는 이 입력을 수정한 뒤 Bevy의 `apply_pbr_lighting`을 호출합니다. 직접 최종 색만 반환하는 셰이더와 달리 Directional·Point·Ambient Light와 그림자 반응이 유지됩니다.

### Rust와 WGSL의 binding 계약

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PulseExtension {
    #[uniform(100)]
    effect: Vec4,
    #[uniform(101)]
    tint: LinearRgba,
    #[texture(102)]
    #[sampler(103)]
    mask_texture: Handle<Image>,
}
```

WGSL도 같은 번호와 자료형을 사용해야 합니다.

```wgsl
@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> effect: vec4<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var<uniform> tint: vec4<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var mask_texture: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(103)
var mask_sampler: sampler;
```

번호, 자료형, texture와 sampler의 순서 중 하나라도 다르면 렌더 파이프라인 생성 오류가 발생합니다. `#{MATERIAL_BIND_GROUP}`은 Bevy가 렌더러 구성에 맞는 Material bind group 번호로 치환합니다.

## 샘플 코드

전체 Rust 코드: `examples/part4/product_showcase/src/bin/30a_custom_pbr_material.rs`

전체 WGSL 코드: `examples/part4/product_showcase/assets/shaders/30a_custom_pbr.wgsl`

### 정점·프래그먼트 셰이더 지정

```rust
impl MaterialExtension for PulseExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/30a_custom_pbr.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/30a_custom_pbr.wgsl".into()
    }
}
```

### PBR 입력을 수정한 뒤 조명 계산

```wgsl
var pbr_input = pbr_input_from_standard_material(in, is_front);

// 마스크 영역의 기본 색과 발광을 변경합니다.
pbr_input.material.base_color = vec4<f32>(
    tinted_base_color,
    pbr_input.material.base_color.a,
);
pbr_input.material.emissive = vec4<f32>(
    pbr_input.material.emissive.rgb + tint.rgb * mask * pulse * effect.z,
    pbr_input.material.emissive.a,
);

var out: FragmentOutput;
out.color = apply_pbr_lighting(pbr_input);
out.color = main_pass_post_lighting_processing(pbr_input, out.color);
```

## 코드 설명

`effect`의 네 성분은 각각 시간, 정점 변위 크기, 발광 강도, 색상 혼합 비율입니다. 매 프레임 Rust 시스템이 경과 시간과 키 입력 상태를 Material asset에 기록하고, Bevy가 uniform buffer로 GPU에 전달합니다.

정점 셰이더는 로컬 정점을 법선 방향으로 움직인 뒤 월드 좌표와 클립 좌표를 다시 계산합니다. 이 예제는 작은 스타일 효과를 보여 주기 위해 변형 후 법선을 다시 계산하지 않습니다. 변위가 커지면 조명 방향이 표면 모양과 어긋날 수 있으므로 실제 지형이나 큰 변형에는 법선 재계산 또는 별도 normal 처리가 필요합니다.

프래그먼트 셰이더는 마스크 텍스처를 읽어 지정 영역에만 tint와 emissive pulse를 적용합니다. emissive는 물체 자체가 밝게 보이게 하지만 주변 물체를 비추는 Light는 아닙니다.

`MaterialPlugin::<CustomPbrMaterial>`을 등록해야 해당 Material의 GPU 파이프라인이 준비됩니다. Rust 구조를 바꾸면 프로그램을 다시 빌드해야 하지만, WGSL만 수정할 때는 `file_watcher` 기능과 `AssetPlugin`의 감시 설정으로 실행 중 다시 로드할 수 있습니다.

오류가 나면 다음 순서로 확인하세요.

1. 터미널의 첫 WGSL 오류 위치를 확인합니다.
2. Rust와 WGSL의 binding 번호와 자료형을 비교합니다.
3. 셰이더 경로가 crate의 `assets` 디렉터리를 기준으로 맞는지 확인합니다.
4. 정점 출력 필드가 프래그먼트 입력에서 사용 가능한지 확인합니다.
5. PBR 함수를 호출했다면 필요한 `bevy_pbr` import가 있는지 확인합니다.

## 실습 과제

1. `C`를 눌러 세 색이 순환하는지 확인하고, 네 번째 색을 직접 추가하세요.
2. 정점 변위 크기를 절반으로 줄여 왼쪽 기본 구체와 윤곽을 비교하세요.
3. 발광 pulse 속도를 조절하는 식의 상수를 바꾸고 한 주기가 얼마나 달라지는지 관찰하세요.

## 심화 과제

별도의 `roughness` uniform을 추가하세요. 키 입력으로 값을 변경하고 `pbr_input.material.perceptual_roughness`에 반영하여, 동일한 Light 아래에서 하이라이트의 크기가 어떻게 변하는지 기록하세요.

과제를 먼저 직접 수행한 뒤 필요하면 [힌트와 수행 예시](exercises/part4/30a_custom_pbr_material.md)를 확인하세요.

## 다음 챕터

다음 장에서는 개별 Material이 아니라 카메라가 완성한 화면 전체를 입력으로 받아 색상 조정과 비네트를 적용합니다.
