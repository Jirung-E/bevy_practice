# 40A. World Editor Scene 저장과 불러오기

## 학습 목표

- Part 1의 `Reflect`와 `DynamicWorld`를 에디터 문서 저장에 적용할 수 있다.
- Hierarchy, Inspector 값, 안정적인 Asset 식별자를 Scene 파일로 왕복할 수 있다.
- 에디터·런타임 전용 Component를 저장 대상에서 제외할 수 있다.
- 새 Scene, 열기, 저장, 변경 여부와 오류 상태를 UI에 표시할 수 있다.

## 이 내용으로 만들 수 있는 것

- 편집한 오브젝트와 계층 구조를 다시 여는 Scene 문서
- 게임 레벨·2D 스테이지·UI 배치를 저장하는 제작 파이프라인
- 런타임 전용 Component를 제외한 안정적인 저장 형식

## 이번에 만들 결과물

Part 6에서 만든 미니 World Editor에 Scene 문서 흐름을 추가합니다.

- `Ctrl+N` / macOS `Command+N`: 새 빈 Scene
- `Ctrl+O` / macOS `Command+O`: `target/world_editor_scene.scn.ron` 열기
- `Ctrl+S` / macOS `Command+S`: 현재 Scene 저장
- `Tab`: 다음 Entity 선택
- 방향키·`PageUp`·`PageDown`: Inspector의 Transform 편집
- `1`·`2`: Cube·Sphere 생성
- `Delete`: 선택 Entity 삭제

아래 명령은 저장소의 완성 샘플을 실행합니다.

```bash
cargo run -p world_editor --bin 40a_scene_io
```

## 핵심 개념

이 장은 Scene 직렬화를 처음 설명하는 장이 아닙니다. [12B. Reflect와 DynamicWorld](12B_ReflectDynamicWorld.md)에서 배운 등록·직렬화·복원 흐름을 실제 에디터 문서에 적용합니다. Scene과 SaveGame의 목적 차이가 헷갈리면 [12C. Scene과 SaveGame 설계](12C_SceneAndSaveGame.md)를 먼저 복습하세요.

### 편집 데이터와 실행 데이터를 나눈다

Scene에 보존하는 값은 다음과 같습니다.

- `SceneId`: 실행을 다시 해도 유지되는 안정적인 ID
- `SceneName`: Hierarchy에 표시할 이름
- `SceneAssetKind`: Cube·Sphere 같은 에셋 키
- `SceneParent`: 부모의 안정적인 SceneId
- `Transform`: Inspector에서 편집한 위치·회전·크기

다음 값은 저장하지 않습니다.

- `Editable`: 현재 에디터에서만 쓰는 marker
- `Mesh3d`, `MeshMaterial3d`: 이번 실행의 Asset Handle
- 선택 상태와 카메라
- 버튼, Text, Console 같은 에디터 UI

### Asset Handle 대신 안정적인 키

`Handle<Mesh>`의 내부 ID와 Entity 번호는 실행마다 달라질 수 있습니다. 파일에는 `SceneAssetKind::Cube`처럼 안정적인 키를 기록하고, 불러올 때 `EditorAssets`가 현재 실행의 Handle로 변환합니다.

```text
SceneAssetKind::Cube → EditorAssets.cube → 현재 실행의 Handle<Mesh>
```

실제 프로젝트에서는 enum 대신 `AssetPath`, UUID 또는 자체 Asset 데이터베이스 키를 사용할 수 있습니다.

### Entity 관계 복원

부모를 런타임 Entity 번호로 저장하지 않고 부모의 `SceneId`를 저장합니다. 불러올 때 모든 Entity를 먼저 생성해 `SceneId → Entity` 표를 만든 다음 `ChildOf`를 연결합니다. 부모가 없는 ID를 가리키거나 자기 자신을 부모로 가리키면 손상된 Scene으로 처리합니다.

## 샘플 코드

전체 코드: `examples/part6/world_editor/src/bin/40a_scene_io.rs`

### 등록된 편집 Component만 staging World에 복사

```rust
let mut staging = registered_app();
staging.world_mut().spawn((
    *id,
    name.clone(),
    *kind,
    SceneParent(parent),
    *transform,
));

let registry = staging.world().resource::<AppTypeRegistry>().read();
let ron = DynamicWorld::from_world_with(staging.world(), &registry)
    .serialize(&registry)?;
```

### 불러온 키를 런타임 Asset Handle로 변환

```rust
let (mesh, material) = match record.kind {
    SceneAssetKind::Cube => (
        assets.cube.clone(),
        assets.cube_material.clone(),
    ),
    SceneAssetKind::Sphere => (
        assets.sphere.clone(),
        assets.sphere_material.clone(),
    ),
};
```

### 버전과 관계 검증

```rust
if header != SCENE_HEADER {
    return Err(format!(
        "지원하지 않는 Scene 버전입니다. 현재 버전: {SCENE_VERSION}"
    ));
}

if let Some(parent) = record.parent
    && !ids.contains(&parent)
{
    return Err(format!("부모 #{parent}를 찾을 수 없습니다"));
}
```

## 코드 설명

실행 중인 전체 World를 바로 직렬화하면 카메라, 조명, UI, 선택 gizmo까지 섞입니다. 예제는 저장 대상만 staging World에 복사한 뒤 `DynamicWorld`로 직렬화합니다. 이 allowlist 방식은 Component가 추가될 때 의도치 않게 문서 형식에 포함되는 문제를 줄입니다.

파일 첫 줄에는 Scene 버전을 기록합니다. 오래된 버전을 무조건 읽어 잘못된 월드를 만들기보다 오류 메시지를 표시합니다. 실제 제품에서는 버전별 마이그레이션 함수를 추가할 수 있습니다.

문서를 편집하면 `MODIFIED *`, 저장하거나 성공적으로 열면 `SAVED`가 표시됩니다. 열기 실패 시 현재 World를 지우지 않고 `OPEN ERROR`만 표시하므로 손상 파일 때문에 작업 중인 Scene을 잃지 않습니다.

저장 위치는 저장소의 `target` 아래입니다. 따라서 빌드 결과와 함께 Git에 포함되지 않으며, 프로그램을 종료한 뒤 다시 실행하면 파일이 존재할 때 자동으로 복원합니다.

### UI/UX 검증 계획

| 시나리오 | 조작 | 기대 결과 |
|---|---|---|
| 새 문서 | `Ctrl+N` / `Command+N` | 빈 Hierarchy, `MODIFIED *` |
| 편집 | Entity 생성·이동 | Inspector 값 변경, dirty 표시 |
| 저장 | `Ctrl+S` / `Command+S` | `SAVED`, 경로와 성공 메시지 |
| 재실행 | 창을 닫고 다시 실행 | 이름·Transform·부모 관계 복원 |
| 열기 실패 | 파일 내용을 손상 후 `Ctrl+O` / `Command+O` | 현재 World 유지, `OPEN ERROR` |
| 오래된 버전 | 헤더 버전을 0으로 변경 | 지원하지 않는 버전 메시지 |
| 누락 부모 | 존재하지 않는 parent ID 입력 | 관계 오류 메시지 |

자동 테스트는 값 왕복, 제외 Component, 손상·구버전·누락 부모를 검사합니다. 키보드 포커스, 메시지 가독성, 재실행 동작은 위 표대로 수동 확인합니다.

## 실습 과제

1. Cube를 생성해 이동한 뒤 저장하고 프로그램을 다시 실행하여 Transform이 복원되는지 확인하세요.
2. 저장 후 Entity를 이동해 `MODIFIED *`가 다시 표시되는지 확인하세요.
3. Scene 파일의 버전 헤더를 0으로 바꾸고 열기 오류가 기존 World를 지우지 않는지 확인하세요.

## 심화 과제

저장 직전에 임시 파일을 만들고 쓰기가 성공한 경우에만 기존 파일과 교체하는 원자적 저장을 구현하세요. 또한 저장된 마지막 내용의 해시를 보관해 편집 값을 원래대로 되돌렸을 때 dirty 상태가 해제되도록 개선하세요.

과제를 먼저 직접 수행한 뒤 필요하면 [힌트와 수행 예시](exercises/part6/40a_world_editor_scene_io.md)를 확인하세요.

## 다음 챕터

Entity에 Script Asset을 연결하고 실행 중 파일 변경을 Hot Reload합니다.
