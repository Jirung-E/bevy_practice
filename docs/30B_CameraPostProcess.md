# 30B. 카메라 후처리 셰이더

## 학습 목표

- 3D 장면을 모두 그린 뒤 화면 전체에 적용하는 후처리 패스를 설명할 수 있다.
- 카메라 출력 텍스처를 샘플링하고 uniform으로 효과 강도를 제어할 수 있다.
- 메인 월드의 설정을 렌더 월드로 추출하는 과정을 이해할 수 있다.
- 독립 Plugin으로 후처리를 추가·제거하고 GPU 오류와 성능을 점검할 수 있다.

## 이번에 만들 결과물

회전하는 세 구체의 최종 카메라 화면에 색상 조정과 비네트를 적용합니다.

- `P`: 후처리 켜기/끄기
- `↑`·`↓`: 효과 강도 조절

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 별도 프로젝트에서 따라 하는 경우 Rust 파일과 WGSL 파일을 함께 복사해야 합니다.

```bash
cargo run -p product_showcase --bin 30b_camera_post_process
```

## 핵심 개념

### 화면 전체 패스

Material 셰이더는 특정 Mesh의 픽셀을 처리합니다. 후처리 셰이더는 장면 렌더링이 끝난 카메라 출력 전체를 입력 텍스처로 읽고 새 출력 텍스처에 기록합니다.

```text
3D 장면 렌더링 → source 화면 텍스처 → 후처리 셰이더 → destination 화면 텍스처
```

이 예제는 화면을 덮는 Mesh를 만들지 않습니다. Bevy의 `FullscreenShader`가 정점 세 개로 화면 전체를 덮는 삼각형을 만듭니다.

### 메인 월드와 렌더 월드

게임 로직이 실행되는 메인 월드와 GPU 명령을 준비하는 렌더 월드는 분리되어 있습니다. 카메라의 `PostProcessSettings`는 다음 두 Plugin을 통해 매 프레임 렌더 월드로 전달됩니다.

- `ExtractComponentPlugin`: 카메라 컴포넌트를 렌더 월드로 추출
- `UniformComponentPlugin`: 추출된 데이터를 GPU uniform buffer에 기록

`P`를 눌러 카메라에서 설정 컴포넌트를 제거하면 렌더 시스템의 `ViewQuery` 대상에서 빠집니다. 따라서 후처리 패스가 실행되지 않으며 회전이나 게임 로직은 그대로 유지됩니다.

### RenderGraph와 실행 위치

RenderGraph는 카메라 렌더링의 여러 단계를 의존 순서대로 실행하는 구조입니다. Bevy 0.19의 이 예제는 직접 그래프 노드 타입을 만들지 않고, `Core3d` 스케줄의 `Core3dSystems::PostProcess` 집합에 렌더 시스템을 등록합니다. 이 집합은 메인 3D 패스 뒤의 확장 지점입니다.

## 샘플 코드

전체 Rust 코드: `examples/part4/product_showcase/src/bin/30b_camera_post_process.rs`

전체 WGSL 코드: `examples/part4/product_showcase/assets/shaders/30b_camera_post_process.wgsl`

### 독립 Plugin 등록

```rust
impl Plugin for CameraPostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<PostProcessSettings>::default(),
            UniformComponentPlugin::<PostProcessSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(RenderStartup, init_post_process_pipeline)
            .add_systems(
                Core3d,
                post_process_system.in_set(Core3dSystems::PostProcess),
            );
    }
}
```

### 카메라 출력 읽기와 쓰기

```rust
let post_process = view_target.post_process_write();

let bind_group = ctx.render_device().create_bind_group(
    "camera_post_process_bind_group",
    &pipeline_cache.get_bind_group_layout(&pipeline.layout),
    &BindGroupEntries::sequential((
        post_process.source,
        &pipeline.sampler,
        settings_binding.clone(),
    )),
);
```

### 화면 좌표를 UV로 변환

```wgsl
let size = vec2<f32>(textureDimensions(screen_texture));
let uv = position.xy / size;
let source = textureSample(screen_texture, screen_sampler, uv);
```

## 코드 설명

`ViewTarget::post_process_write()`는 현재 화면인 `source`와 다음 결과를 기록할 `destination`을 반환합니다. 반드시 destination에 그려야 Bevy가 다음 렌더 단계에서 새 결과를 사용합니다.

소스 텍스처는 프레임마다 두 내부 텍스처 사이를 번갈아 사용합니다. 예제의 캐시는 texture view ID가 같을 때만 bind group을 재사용합니다.

WGSL은 `@builtin(position)`을 현재 픽셀 좌표로 받고 텍스처 크기로 나누어 0~1 UV를 만듭니다. 이 방식은 창 크기가 바뀌어도 현재 해상도를 사용합니다. 비네트는 화면 중심에서 멀수록 색을 어둡게 하고, 색상 조정은 RGB 채널의 비율을 변경합니다.

후처리는 일반적으로 해상도에 비례해 비용이 증가합니다. 1920×1080에서 한 번 실행하던 패스를 3840×2160에서 실행하면 처리할 픽셀은 약 네 배가 됩니다. 효과마다 source를 다시 읽고 destination에 쓰므로 패스 수와 텍스처 샘플 수도 중요합니다.

문제가 생기면 다음을 확인하세요.

1. WGSL binding 순서가 texture, sampler, uniform layout과 같은지 확인합니다.
2. `ColorTargetState` 형식이 카메라 출력 형식과 맞는지 확인합니다. HDR 카메라를 사용하면 대상 형식도 함께 바꿔야 합니다.
3. 셰이더 컴파일 오류는 터미널의 첫 WGSL 위치부터 수정합니다.
4. GPU 프레임 캡처 도구나 Bevy 진단 Plugin으로 효과 ON/OFF의 프레임 시간을 비교합니다.
5. 창 크기를 키워 프레임 시간이 급격히 증가한다면 샘플 수와 후처리 패스 수를 줄입니다.

## 실습 과제

1. `P`를 반복해서 눌러 물체 회전은 유지되고 화면 효과만 사라지는지 확인하세요.
2. 강도를 0.0, 0.5, 1.0으로 바꿔 화면 가장자리와 중앙의 차이를 비교하세요.
3. 창 크기를 바꾸고 비네트 중심과 모양이 계속 올바른지 확인하세요.

## 심화 과제

비네트와 색상 조정의 강도를 서로 다른 uniform으로 분리하고 각각 다른 키로 조절하세요. 그다음 두 효과를 하나의 패스에서 처리할 때와 두 개의 독립 패스로 나눌 때 필요한 텍스처 읽기·쓰기 횟수를 비교해 설명하세요.

과제를 먼저 직접 수행한 뒤 필요하면 [힌트와 수행 예시](exercises/part4/30b_camera_post_process.md)를 확인하세요.

## 다음 챕터

Part 5에서는 렌더링이 준비된 3D 월드를 플레이 가능한 TPS 예제로 발전시킵니다.
