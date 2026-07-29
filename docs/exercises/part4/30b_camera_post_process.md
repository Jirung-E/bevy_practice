# 30B. 카메라 후처리 과제 해설

## 실습 과제 힌트

1. `P`는 카메라의 `PostProcessSettings` 컴포넌트를 제거하거나 다시 삽입합니다. `Rotates` 시스템은 이 컴포넌트를 조회하지 않습니다.
2. intensity가 0이면 WGSL의 `mix` 결과가 원본 색이 됩니다. 컴포넌트 제거와 달리 화면 전체 패스 자체는 계속 실행됩니다.
3. UV는 고정 해상도 상수가 아니라 `textureDimensions` 결과로 계산되어야 합니다.

## 심화 과제 수행 방향

설정 구조체에 `color_grade`와 `vignette`를 별도 `f32`로 두고 16바이트 정렬을 유지하세요.

```rust
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessSettings {
    color_grade: f32,
    vignette: f32,
    time: f32,
    _padding: f32,
}
```

한 패스에 결합하면 화면 텍스처를 한 번 읽고 한 번 기록할 수 있습니다. 두 패스로 나누면 효과를 독립적으로 조립하기는 쉽지만, 중간 텍스처와 추가 fullscreen draw가 필요합니다. 실제 선택은 효과 재사용성, 순서 의존성, GPU 측정 결과를 함께 고려합니다.
