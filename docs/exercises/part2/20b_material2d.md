# 20B. Material2d 과제 해설

[본문으로 돌아가기](../../20B_Material2d.md#실습-과제)

## P2-C20B-P1 · 흔들림 배율

| 배율 | 관찰 결과 |
|---:|---|
| `0.0` | 시간은 변하지만 정점 이동 없음 |
| `10.0` | 작은 흔들림 |
| `80.0` | 큰 흔들림 |

Transform과 충돌 위치는 세 경우 모두 같아야 합니다. 화면 효과가 게임 좌표까지 바꾸면 렌더링과 게임 로직이 분리되지 않은 것입니다.

## P2-C20B-P2 · 피격 지속 시간

Timer의 duration만 `0.1`, `1.0`초로 바꿉니다. 점멸 강도는 남은 비율로 계산하므로 duration이 달라도 시작은 1, 끝은 0입니다.

```rust
let flash = 1.0 - timer.fraction();
```

짧은 효과는 즉각적이지만 놓치기 쉽고, 긴 효과는 상태 전달은 분명하지만 조작 피드백을 방해할 수 있습니다.

## P2-C20B-P3 · 피격색

WGSL 상수 예시:

```wgsl
const HIT_WHITE: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
const HIT_YELLOW: vec3<f32> = vec3<f32>(1.0, 0.9, 0.15);
```

원본 alpha를 유지해 투명 배경이 사각형으로 나타나지 않는지도 확인하세요.

## P2-C20B-P4 · atlas 프레임 선택

4열 atlas에서 한 프레임의 UV 폭은 `0.25`입니다.

| 열 | `uv_rect.x` |
|---:|---:|
| 0 | `0.0` |
| 1 | `0.25` |
| 2 | `0.5` |
| 3 | `0.75` |

`uv_rect.z`는 계속 `0.25`여야 한 프레임만 읽습니다.

## P2-C20B-A1 · 상태 Component와 렌더 uniform 분리

게임 로직은 상태 marker만 추가하거나 제거합니다.

```rust
#[derive(Component)]
struct Poisoned;

#[derive(Component)]
struct Shielded;
```

`SpriteEffectPlugin`의 변환 System이 marker 조합을 읽고 색과 강도를 uniform으로 변환합니다. 예제에서는 동시에 여러 상태가 있을 때 `HitFlash > Shielded > Poisoned > Normal` 우선순위를 명시합니다.

| 상태 | 색 | 강도 |
|---|---|---:|
| HitFlash | 흰색 | Timer 남은 비율 |
| Shielded | 청록색 | `0.65` |
| Poisoned | 녹색 | `0.55` |
| Normal | 원본 | `0.0` |

### 확인 기준

- 게임 System은 `Assets<SpriteEffectMaterial>`을 직접 수정하지 않는다.
- 렌더 Plugin이 상태를 uniform으로 변환한다.
- 여러 상태가 겹칠 때 결과가 System 실행 순서에 우연히 의존하지 않는다.
- 렌더 전용 Material Handle은 저장 데이터에서 제외한다.

색상 혼합을 누적하는 방식도 가능하지만, 그 경우 clamp와 조합 규칙을 별도로 정의해야 합니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin material_effects_solution
cargo test -p space_survivor --bin material_effects_solution
```

실제 Material2d와 WGSL 화면은 다음 명령으로 확인합니다.

```bash
cargo run -p space_survivor --bin material_2d
```

상태 변환 전체 코드: `examples/part2/space_survivor/src/bin/material_effects_solution.rs`
