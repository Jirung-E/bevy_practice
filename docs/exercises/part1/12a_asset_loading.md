# 12A. Asset Loading 과제 해설

[본문으로 돌아가기](../../12A_AssetLoading.md#실습-과제)

## P1-C12A-P1 · Handle을 공유하는 두 번째 Sprite

`Handle<Image>`를 `clone()`해도 픽셀 데이터가 복제되지 않습니다. 두 Sprite가 같은 Asset ID를 참조하고 Transform만 다르게 설정하면 됩니다.

```rust
for x in [-190.0, 190.0] {
    commands.spawn((
        Sprite {
            image: assets.preview.clone(),
            custom_size: Some(Vec2::new(340.0, 238.0)),
            ..default()
        },
        Transform::from_xyz(x, -30.0, 0.0),
    ));
}
```

## P1-C12A-P2 · 실패 경로 확인

파일을 실제로 삭제하지 않아도 `PREVIEW_PATH`를 존재하지 않는 경로로 잠시 바꾸어 같은 흐름을 확인할 수 있습니다.

### 확인 기준

- panic으로 종료되지 않는다.
- 오류 로그에 실패한 경로가 나온다.
- 상태가 Failed로 전환되고 fallback 화면이 나타난다.
- 확인 후 원래 경로를 복구한다.

## P1-C12A-P3 · 상태 진입·이탈 로그

`OnEnter`와 `OnExit`에 작은 로그 System을 등록합니다. Loading에서 Ready 또는 Failed로 나갈 때 `OnExit(Loading)`이 한 번 실행되어야 합니다.

## P1-C12A-A1 · 여러 에셋 진행률

여러 Handle을 한 벡터에 억지로 같은 타입으로 넣는 대신 경로, 종류, 상태를 함께 추적하는 레코드를 둡니다. UI가 필요한 값은 다음과 같이 순수 계산으로 분리할 수 있습니다.

```rust
struct LoadProgress {
    total: usize,
    completed: usize,
    failed_paths: Vec<&'static str>,
}
```

각 typed Handle의 `LoadState`를 읽어 공통 `TrackedState`로 변환한 뒤 진행률을 계산하면 UI와 에셋 타입이 느슨하게 결합됩니다.

| 종류 | fallback 예시 |
|---|---|
| Image | 단색 Sprite 또는 누락 이미지 |
| Audio | 무음으로 계속 진행 |
| Scene | 대체 기본 Mesh 또는 해당 오브젝트 생략 |

실패를 모두 같은 정책으로 처리하지 않는 것이 핵심입니다. 필수 Scene 실패는 Failed 상태로 보내고, 선택적 효과음 실패는 경고만 남긴 채 Ready로 진행할 수 있습니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin asset_loading_solution
```

전체 코드는 같은 PNG Handle을 공유하는 두 Sprite와 상태 로그를 구현하며, 진행률 계산은 자동 테스트로 검증합니다.

전체 코드: `examples/part1/ecs_basics/src/bin/asset_loading_solution.rs`

