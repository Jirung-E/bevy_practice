# 20A. 렌더링 파이프라인과 WGSL 과제 해설

[본문으로 돌아가기](../../20A_RenderingPipeline.md#실습-과제)

## P2-C20A-P1 · 정점 이동량 비교

정점 이동량의 부호는 기울어지는 방향을, 절댓값은 변형 크기를 바꿉니다.

| 값 | 예상 결과 |
|---:|---|
| `90.0` | 위쪽 정점이 오른쪽으로 이동 |
| `-120.0` | 위쪽 정점이 왼쪽으로 더 크게 이동 |
| `30.0` | 같은 방향으로 완만하게 이동 |

Transform은 바뀌지 않고 Mesh의 정점이 GPU에서 이동하는지 확인하세요.

## P2-C20A-P2 · UV의 R/G 교환

```wgsl
let uv_color = vec4<f32>(input.uv.y, input.uv.x, 1.0 - input.uv.x, 1.0);
```

R 채널이 세로 방향, G 채널이 가로 방향 변화로 바뀝니다. UV 자체나 정점 위치는 변경되지 않습니다.

## P2-C20A-P3 · V/F 네 가지 조합

| Vertex | Fragment | 윤곽 | 내부 색 |
|:---:|:---:|---|---|
| OFF | OFF | 직사각형 | base color |
| ON | OFF | 변형됨 | base color |
| OFF | ON | 직사각형 | UV gradient |
| ON | ON | 변형됨 | UV gradient |

Vertex 단계는 정점 위치와 윤곽을, Fragment 단계는 각 픽셀의 최종 색을 결정합니다. 한 단계가 다른 단계의 역할을 대신한다고 설명하면 안 됩니다.

## P2-C20A-A1 · 시간 uniform 정점 애니메이션

Rust는 Material의 `options.z`에 경과 시간을 기록합니다.

```rust
material.options.z = time.elapsed_secs();
```

WGSL은 이 값을 `sin` 입력으로 사용합니다.

```wgsl
local_position.x += sin(options.z * 3.0 + local_position.y * 0.02)
    * 12.0
    * options.x;
```

### 확인 기준

- Rust System은 Material uniform만 변경한다.
- Mesh asset과 Transform을 매 프레임 수정하지 않는다.
- Vertex 효과를 끄면 흔들림도 사라진다.
- 시간은 누적되지만 위치 판정과 충돌 좌표는 그대로다.

GPU 정점 애니메이션은 렌더링되는 모양만 바꿉니다. `Transform` 이동은 ECS 월드 위치를 바꾸므로 카메라, 충돌, Query 결과에도 영향을 줍니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin rendering_pipeline_solution
cargo test -p space_survivor --bin rendering_pipeline_solution
```

키 `V`, `F`로 단계를 켜고 끄며 `1`, `2`, `3`으로 정점 이동량 `90`, `-120`, `30`을 선택합니다.

전체 Rust 코드: `examples/part2/space_survivor/src/bin/rendering_pipeline_solution.rs`

전체 WGSL 코드: `examples/part2/space_survivor/assets/shaders/20a_pipeline_solution.wgsl`
