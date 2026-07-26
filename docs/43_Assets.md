# 43. Assets 관리

## 학습 목표

- Assets 저장소와 Handle의 역할을 구분할 수 있다.
- FromWorld로 에셋 카탈로그 Resource를 초기화할 수 있다.
- 에셋 로딩과 게임 Entity 생성을 분리할 수 있다.

## 이번에 만들 결과물

플레이어와 적이 공유할 Mesh와 Material을 ArenaAssets에 등록합니다. 아직 Gameplay를 활성화하지 않고 에셋 계층만 준비합니다.

```bash
cargo run -p production_structure --bin 43_assets
```

## 핵심 개념

Assets는 실제 데이터를 타입별로 저장하고 Handle은 그 데이터를 가리키는 값싼 참조입니다. 각 System에서 동일 경로를 반복 로드하거나 Mesh를 다시 만들지 말고 의미 있는 카탈로그 Resource에 Handle을 모읍니다.

예제는 외부 파일 없이 실행되도록 절차적 Mesh를 만들지만, AssetServer로 GLTF·Image·Audio를 로드할 때도 같은 Handle 카탈로그 구조를 사용합니다.

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

## 실습 과제

1. 바닥 Mesh와 Material도 ArenaAssets로 옮기세요.
2. Material 색상 변형 Handle을 배열로 관리하세요.
3. AssetServer로 이미지 하나를 로드하고 로드 상태를 로그로 확인하세요.

## 심화 과제

에셋 그룹별 Loading State, 진행률, 실패 목록, fallback 에셋을 관리하는 AssetLoadingPlugin을 설계하세요.

## 다음 챕터

입력, 시뮬레이션, 피드백 SystemSet과 Message를 사용해 기능 결합도를 낮춘 Gameplay를 구성합니다.

