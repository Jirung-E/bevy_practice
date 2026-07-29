# 30A. 커스텀 PBR Material 과제 해설

## 실습 과제 힌트

1. 색상 배열의 길이와 `% 3`을 함께 바꿔야 새 색까지 순환합니다.
2. 정점 변위는 Rust에서 `effect.y`로 전달됩니다. `V`를 눌러 0과 변경값을 비교하세요.
3. WGSL의 `sin(effect.x * 4.0 + ...)`에서 시간에 곱하는 값이 클수록 빠르게 반복됩니다.

## 수행 예시

### 네 번째 색 추가

```rust
fn tint_color(index: usize) -> LinearRgba {
    [
        LinearRgba::rgb(0.1, 1.3, 2.6),
        LinearRgba::rgb(2.8, 0.55, 0.08),
        LinearRgba::rgb(2.0, 0.12, 1.4),
        LinearRgba::rgb(0.35, 2.2, 0.35),
    ][index % 4]
}
```

입력 처리의 순환 범위와 상태 문자열의 이름 배열도 네 항목으로 맞춥니다.

### roughness uniform 설계

`PulseExtension`에 새 uniform binding을 추가하고 WGSL에서도 같은 번호와 `f32` 자료형으로 선언합니다. 프래그먼트 셰이더에서 PBR 조명 계산 전에 다음처럼 반영할 수 있습니다.

```wgsl
pbr_input.material.perceptual_roughness = roughness;
```

값은 0과 1 사이로 제한하세요. 0에 가까울수록 하이라이트가 날카롭고, 1에 가까울수록 넓고 흐릿해집니다.
