# 39. Asset Browser

## 학습 목표

- 재사용 Mesh와 Material Handle을 에디터 Resource로 관리할 수 있다.
- Asset 항목에서 새 Entity를 생성할 수 있다.
- 생성 직후 선택과 로그를 함께 갱신할 수 있다.

## 이번에 만들 결과물

하단 Asset Browser에서 Cube 또는 Sphere 버튼을 눌러 새 Editable Entity를 생성하고 즉시 선택합니다.

```bash
cargo run -p world_editor --bin 39_asset_browser
```

## 핵심 개념

Asset Browser는 파일 목록만 보여 주는 패널이 아니라 에셋을 World 인스턴스로 만드는 작업의 시작점입니다. 예제는 외부 파일 없이 Mesh와 Material Handle을 `EditorAssets` Resource에 캐시합니다.

같은 버튼을 여러 번 눌러도 Mesh 데이터는 Assets에 한 번만 있고 새 Entity는 Handle만 복제합니다.

## 샘플 코드

```rust
#[derive(Resource)]
struct EditorAssets {
    cube: Handle<Mesh>,
    sphere: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
    sphere_material: Handle<StandardMaterial>,
}
```

```rust
let entity = spawn_editable(
    &mut commands,
    "New Cube",
    assets.cube.clone(),
    assets.cube_material.clone(),
    Vec3::new(0.0, 0.75, 0.0),
);
selection.0 = Some(entity);
log.lines.push(format!("Created cube {entity:?}"));
```

## 코드 설명

- Resource는 브라우저가 사용할 에셋 카탈로그 역할을 합니다.
- Handle 복제는 실제 Mesh 복제가 아닙니다.
- spawn_editable 함수가 이름, 표식, 렌더 Component, Transform 규칙을 한곳에 둡니다.
- 생성된 Entity를 선택해 Inspector에서 바로 편집할 수 있습니다.
- 실제 파일 브라우저는 AssetServer의 로드 상태와 타입, 썸네일 캐시를 관리해야 합니다.

## 실습 과제

1. Cylinder Asset 버튼을 추가하세요.
2. 생성 위치가 서로 겹치지 않도록 순번별 offset을 적용하세요.
3. Material 색상 변형을 여러 항목으로 표시하세요.

## 심화 과제

assets 폴더를 비동기로 스캔하고 확장자별 Asset 타입, 로드 상태, 썸네일을 가진 카드형 브라우저를 설계하세요.

## 다음 챕터

선택, 이동, 생성, 삭제 작업을 최근 로그 Console에 표시합니다.

