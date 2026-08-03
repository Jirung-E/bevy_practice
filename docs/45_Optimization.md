# 45. 측정 기반 최적화

## 학습 목표

- 성능 문제를 측정하고 병목을 분류할 수 있다.
- 변경 감지와 Query Filter로 불필요한 작업을 줄일 수 있다.
- 개발 빌드와 배포 빌드의 최적화 전략을 구분할 수 있다.

## 이 내용으로 만들 수 있는 것

- 프레임 시간과 Entity 수를 관찰하는 성능 HUD
- 변경된 데이터만 다시 계산하는 반응형 System
- 프로파일링 결과를 근거로 Query와 스케줄을 개선한 게임

## 이번에 만들 결과물

교재의 최종 Production Arena입니다. 프레임 시간 진단을 활성화하고 Score가 바뀔 때만 HUD 문자열을 다시 만듭니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p production_structure --bin 45_optimization
```

릴리스 성능 확인:

```bash
cargo run --release -p production_structure --bin 45_optimization
```

같은 임시 효과를 `spawn/despawn`하는 경로와 미리 만든 Entity를 재사용하는 경로는 별도 비교 샘플로 확인합니다.

```bash
cargo run --release -p production_structure --bin pooling_comparison
```

## 핵심 개념

최적화 순서:

1. 재현 가능한 장면과 목표 프레임 시간을 정합니다.
2. CPU, GPU, 메모리, 로딩 중 어느 영역인지 측정합니다.
3. 가장 큰 병목 하나를 줄입니다.
4. 같은 장면에서 다시 측정합니다.
5. 회귀 방지 테스트나 진단을 남깁니다.

FPS 숫자만 보고 ECS를 무작정 바꾸면 복잡성만 늘 수 있습니다.

## 오브젝트 풀링은 조건부 최적화다

ECS Entity는 객체지향 엔진의 무거운 GameObject와 비용 구조가 같지 않습니다. Bevy의 `spawn/despawn`이 실제 병목이라는 측정 없이 모든 총알과 적을 풀링하면 다음 비용이 생깁니다.

- 비활성 Entity도 World와 Query에 남아 생기는 메모리·필터 비용
- `active` 상태를 모든 관련 System이 빠뜨리지 않고 검사해야 하는 복잡성
- 재사용할 때 Timer, Velocity, 관계와 이벤트 상태를 완전히 초기화해야 하는 위험
- 최대 pool 크기, 고갈과 장면 전환 정책 관리

다음 조건이 함께 보일 때 풀링을 검토합니다.

| 질문 | 풀링을 검토할 신호 |
|---|---|
| 무엇이 반복되는가? | 같은 Component 구성의 짧은 수명 효과가 대량 생성·제거됨 |
| 측정된 병목은? | spawn/despawn과 archetype 이동이 CPU profile 상 유의미함 |
| 재사용이 단순한가? | 초기화할 상태가 작고 명확함 |
| 비활성 비용은? | pool이 메모리 예산 안에 있고 Query에서 쉽게 제외됨 |

`pooling_comparison`은 같은 수의 임시 효과를 반복합니다. 기준 경로는 매 burst마다 Entity를 만들고 제거하며, pool 경로는 처음 `BURST_SIZE`만 만들고 `active`와 데이터를 재설정합니다. Entity 할당 횟수와 최종 Entity 수는 결정적으로 검사하고, 실행 시간은 기계·빌드·백그라운드 부하에 영향을 받으므로 참고값으로만 출력합니다.

```rust
for entity in pool.iter().copied() {
    let mut effect = world.get_mut::<TemporaryEffect>(entity).unwrap();
    effect.active = true;
    effect.position = spawn_position;
}
// 수명이 끝나면 despawn 대신 모든 상태를 초기화하고 active = false
```

실제 렌더 Entity라면 비활성 시 `Visibility::Hidden`을 사용하고, 실행 System Query에는 `ActiveEffect` 표식이나 `active` 조건을 일관되게 적용합니다. Component 표식을 삽입·제거하면 archetype 이동은 여전히 발생하므로 그것까지 줄이려는 pool인지 구분해야 합니다.

## 샘플 코드

```rust
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ));
    }
}
```

```rust
fn update_hud(
    score: Res<Score>,
    mut hud: Single<&mut Text, With<Hud>>,
) {
    if !score.is_changed() {
        return;
    }
    hud.0 = format!("SCORE {:05}", score.0);
}
```

## 코드 설명

- FrameTimeDiagnosticsPlugin은 프레임 시간과 FPS 측정값을 수집합니다.
- LogDiagnosticsPlugin은 주기적으로 진단 결과를 로그에 기록합니다.
- 변경 감지는 값 비교가 아니라 ECS 변경 tick을 사용합니다.
- `Changed<T>`, `Added<T>`, `Without<T>` Filter로 Query 대상을 줄일 수 있습니다.
- 실제 병목에서는 tracing span, GPU profiler, entity 수, draw call, 에셋 메모리도 함께 봅니다.
- 풀링 전후에는 평균뿐 아니라 최악 프레임, Entity 수, 메모리와 초기화 누락도 비교합니다.

풀링 비교 전체 코드는 [45a_pooling_comparison.rs](source/part7.md#45a--조건부-오브젝트-풀링-비교)에서 확인할 수 있습니다.

개발 중 빠른 반복에는 동적 링크와 빠른 링커가 유용하지만 배포 성능은 `--release`로 측정해야 합니다. LTO와 codegen-units 같은 프로필 옵션은 빌드 시간과 런타임 성능을 비교한 뒤 선택하세요.

## 실습 과제

1. 적 Entity를 1,000개로 늘리고 진단값을 기록하세요.
2. 이동 Query에 필요한 Component만 포함되어 있는지 검토하세요.
3. 개발 빌드와 release 빌드의 프레임 시간을 비교하세요.
4. `pooling_comparison`의 pool 크기를 burst의 절반으로 줄이고 고갈 시 확장·요청 거부 중 하나의 정책을 구현하세요.

## 심화 과제

프레임 시간, Entity 수, 적 수의 기준값을 기록하는 성능 시나리오를 만들고 변경 전후 결과를 자동 비교하는 벤치마크 절차를 작성하세요. 풀링 적용 전후의 p95/p99 프레임 시간, 메모리, 비활성 Entity 수와 재사용 초기화 테스트도 포함하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part7/45_optimization.md)

## 다음 챕터

전체 커리큘럼을 완료했습니다. 관심 있는 Part의 심화 과제를 실제 프로젝트 요구사항에 맞게 확장하고, Bevy 버전을 올릴 때는 공식 마이그레이션 가이드와 전체 워크스페이스 검사를 다시 실행하세요.
