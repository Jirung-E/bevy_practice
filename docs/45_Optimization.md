# 45. 측정 기반 최적화

## 학습 목표

- 성능 문제를 측정하고 병목을 분류할 수 있다.
- 변경 감지와 Query Filter로 불필요한 작업을 줄일 수 있다.
- 개발 빌드와 배포 빌드의 최적화 전략을 구분할 수 있다.

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

## 핵심 개념

최적화 순서:

1. 재현 가능한 장면과 목표 프레임 시간을 정합니다.
2. CPU, GPU, 메모리, 로딩 중 어느 영역인지 측정합니다.
3. 가장 큰 병목 하나를 줄입니다.
4. 같은 장면에서 다시 측정합니다.
5. 회귀 방지 테스트나 진단을 남깁니다.

FPS 숫자만 보고 ECS를 무작정 바꾸면 복잡성만 늘 수 있습니다.

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

개발 중 빠른 반복에는 동적 링크와 빠른 링커가 유용하지만 배포 성능은 `--release`로 측정해야 합니다. LTO와 codegen-units 같은 프로필 옵션은 빌드 시간과 런타임 성능을 비교한 뒤 선택하세요.

## 실습 과제

1. 적 Entity를 1,000개로 늘리고 진단값을 기록하세요.
2. 이동 Query에 필요한 Component만 포함되어 있는지 검토하세요.
3. 개발 빌드와 release 빌드의 프레임 시간을 비교하세요.

## 심화 과제

프레임 시간, Entity 수, 적 수의 기준값을 기록하는 성능 시나리오를 만들고 변경 전후 결과를 자동 비교하는 벤치마크 절차를 작성하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part7/45_optimization.md)

## 다음 챕터

전체 커리큘럼을 완료했습니다. 관심 있는 Part의 심화 과제를 실제 프로젝트 요구사항에 맞게 확장하고, Bevy 버전을 올릴 때는 공식 마이그레이션 가이드와 전체 워크스페이스 검사를 다시 실행하세요.
