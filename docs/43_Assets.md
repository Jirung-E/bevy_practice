# 43. Assets 관리

## 학습 목표

- Assets 저장소와 Handle의 역할을 구분할 수 있다.
- FromWorld로 에셋 카탈로그 Resource를 초기화할 수 있다.
- 에셋 로딩과 게임 Entity 생성을 분리할 수 있다.
- 전역·State·Entity 수명에 맞춰 Handle 소유권과 언로드 정책을 설계할 수 있다.

## 이 내용으로 만들 수 있는 것

- 자주 쓰는 Mesh·Material·Image Handle 카탈로그
- 로딩 완료 후 게임 화면으로 전환하는 프리로드 단계
- 누락된 에셋에 대체 리소스를 제공하는 안전한 로더

## 이번에 만들 결과물

플레이어와 적이 공유할 Mesh와 Material을 ArenaAssets에 등록합니다. 아직 Gameplay를 활성화하지 않고 에셋 계층만 준비합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p production_structure --bin 43_assets
```

## 핵심 개념

Assets는 실제 데이터를 타입별로 저장하고 Handle은 그 데이터를 가리키는 값싼 참조입니다. 각 System에서 동일 경로를 반복 로드하거나 Mesh를 다시 만들지 말고 의미 있는 카탈로그 Resource에 Handle을 모읍니다.

예제는 외부 파일 없이 실행되도록 절차적 Mesh를 만들지만, AssetServer로 GLTF·Image·Audio를 로드할 때도 같은 Handle 카탈로그 구조를 사용합니다.

### 하나의 전역 카탈로그가 만드는 함정

`ArenaAssets`가 앱 시작부터 종료까지 존재하면 그 안의 모든 Handle도 계속 살아 있습니다. 항상 쓰는 UI 글꼴과 플레이어 Mesh에는 알맞지만, 한 스테이지에서만 쓰는 4K Texture까지 전역 카탈로그에 넣으면 State를 떠나도 메모리에서 내려가지 않습니다.

Asset 그룹은 실제 사용 수명에 맞춰 나눕니다.

| 소유 위치 | 적합한 Asset | 해제 시점 |
|---|---|---|
| 앱 전역 Resource | 공통 글꼴, fallback, 공용 UI | 앱 종료 |
| State 전용 Resource | 메뉴 배경, 스테이지 Scene·Texture | `OnExit(State)`에서 Resource 제거 후 마지막 Entity 정리 |
| Entity의 Handle Component | 개별 캐릭터·이펙트 | Entity despawn과 함께 |
| 명시적 캐시 Resource | 곧 다시 쓸 고비용 Asset | 메모리 예산·LRU 정책에 따라 |

언로드는 파일을 지우는 작업이 아니라 마지막 강한 Handle을 놓는 작업입니다. 같은 Asset을 사용하는 Entity가 남아 있다면 State Resource를 제거해도 안전하게 유지됩니다. 반대로 Handle을 모두 놓은 직후 같은 경로를 다시 요청하면 디스크 읽기와 GPU 업로드 비용이 다시 발생할 수 있으므로, 화면 전환마다 무조건 비우는 정책도 좋지 않습니다.

### 실제 메모리와 관찰

`AssetEvent::Unused`는 마지막 강한 Handle이 사라졌다는 신호이고 `Removed`는 `Assets<T>`에서 본문이 제거됐다는 신호입니다. GPU에 제출된 자원이 정확히 같은 순간 운영체제 메모리 통계에서 사라진다고 가정하면 안 됩니다. 렌더 프레임 지연, 드라이버 캐시와 allocator 동작이 있기 때문입니다. 이벤트, `Assets<T>::len()`, 프로파일러의 CPU/GPU 메모리를 함께 관찰합니다.

Hot Reload를 위해 Source Asset을 감시하는 것과 Asset을 메모리에 유지하는 것도 별개입니다. 감시 기능을 켰다고 모든 Asset이 자동으로 상주하지 않습니다.

## 샘플 코드

```rust
#[derive(Resource)]
pub struct ArenaAssets {
    pub player_mesh: Handle<Mesh>,
    pub enemy_mesh: Handle<Mesh>,
    pub player_material: Handle<StandardMaterial>,
    pub enemy_material: Handle<StandardMaterial>,
}

impl FromWorld for ArenaAssets {
    fn from_world(world: &mut World) -> Self {
        let player_mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Capsule3d::new(0.42, 0.9));
        // 나머지 에셋도 같은 방식으로 등록
        Self { /* handles */ }
    }
}
```

## 코드 설명

- AssetCatalogPlugin은 `init_resource::<ArenaAssets>()`만 소유합니다.
- FromWorld는 초기화 시 기존 Assets Resource에 접근할 수 있습니다.
- GameplayPlugin은 파일 경로나 Mesh 생성 방법을 모르고 의미 있는 Handle만 사용합니다.
- Handle clone은 에셋 본문을 복제하지 않습니다.
- 외부 파일은 비동기 로드되므로 Loading State와 로드 실패 정책을 추가해야 합니다.
- 전역 `ArenaAssets`가 가진 Handle은 의도적으로 앱 전체 수명을 가지며, 스테이지 전용 카탈로그에는 같은 방식을 그대로 쓰지 않습니다.
- State 전용 Resource를 제거하기 전에 그 Handle을 사용하는 Entity 정리 순서를 설계합니다.

## 실습 과제

1. 바닥 Mesh와 Material도 ArenaAssets로 옮기세요.
2. Material 색상 변형 Handle을 배열로 관리하세요.
3. AssetServer로 이미지 하나를 로드하고 로드 상태를 로그로 확인하세요.
4. State 전용 Handle Resource를 만들고 State 이탈 전후의 `AssetEvent::Unused`와 `Removed`를 기록하세요.

## 심화 과제

에셋 그룹별 Loading State, 진행률, 실패 목록, fallback 에셋을 관리하는 AssetLoadingPlugin을 설계하세요. 전역·메뉴·스테이지 캐시별 메모리 예산과 “즉시 해제/다음 State까지 유지/LRU 제거” 정책도 명시하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part7/43_assets.md)

## 다음 챕터

입력, 시뮬레이션, 피드백 SystemSet과 Message를 사용해 기능 결합도를 낮춘 Gameplay를 구성합니다.
