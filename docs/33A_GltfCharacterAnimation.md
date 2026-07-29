# 33A. glTF 캐릭터와 실제 애니메이션

## 학습 목표

- glTF/GLB가 Scene, Mesh, Material, Skin, Animation을 묶는 형식임을 이해할 수 있다.
- 로딩 완료를 확인한 뒤 glTF 내부 Scene과 이름 있는 Animation에 접근할 수 있다.
- `AnimationGraph`, `AnimationPlayer`, `AnimationTransitions`의 역할을 구분할 수 있다.
- 입력 상태와 애니메이션 Handle을 분리하고 Survey·Walk·Run을 혼합 전환할 수 있다.

## 이번에 만들 결과물

Khronos Fox GLB를 실제 3D Scene으로 생성하고 입력에 따라 세 애니메이션을 전환합니다. 이 모델에는 별도 Idle 클립이 없으므로 주위를 살피는 `Survey`를 정지 상태로 사용합니다.

- 입력 없음: Survey(Idle)
- `WASD`: Walk
- `Shift` + `WASD`: Run

아래 명령은 저장소에 포함된 GLB까지 사용하는 완성 샘플을 실행합니다.

```bash
cargo run -p tps_training --bin 33a_gltf_character
```

## 핵심 개념

### glTF 하나에는 여러 종류의 데이터가 있다

GLB는 glTF JSON, 바이너리 Mesh·Skin·Animation 데이터, 이미지 등을 하나의 파일에 담을 수 있습니다. Bevy에서 파일 전체를 `Handle<Gltf>`로 로드하면 다음 항목에 접근할 수 있습니다.

- `default_scene`, `scenes`: 생성할 Scene
- `named_animations`: 이름으로 찾는 AnimationClip
- `meshes`, `materials`: 개별 렌더 에셋

에셋 Handle을 얻은 직후에는 실제 데이터가 준비되지 않았을 수 있습니다. `is_loaded_with_dependencies`가 참이 된 뒤 `Assets<Gltf>`를 조회해야 Scene 내부 텍스처 등 의존 에셋까지 준비되었음을 보장할 수 있습니다.

### SceneRoot와 Bevy 0.19의 WorldAssetRoot

이전 Bevy 자료에서 glTF Scene을 생성할 때 `SceneRoot`라는 이름을 볼 수 있습니다. Bevy 0.19의 현재 API는 월드 에셋 생성 흐름을 통합한 `WorldAssetRoot`를 사용합니다.

```rust
commands.spawn(WorldAssetRoot(fox.default_scene.clone().unwrap()));
```

역할은 같습니다. 지정한 Scene asset을 루트 아래 Entity 계층으로 인스턴스화합니다. 이 장은 최신 Bevy 0.19 명칭을 사용합니다.

### AnimationGraph와 AnimationPlayer

`AnimationClip`은 키프레임 데이터입니다. `AnimationGraph`는 어떤 클립을 재생하고 혼합할지 정의하는 그래프이며, `AnimationPlayer`는 인스턴스마다 현재 재생 상태를 가집니다.

glTF Scene의 Entity 계층은 비동기로 생성됩니다. `WorldInstanceReady`가 발생했을 때 자동 생성된 `AnimationPlayer`에 `AnimationGraphHandle`과 `AnimationTransitions`를 연결합니다. 로딩 전에 자식 Entity를 찾으려 하지 않는 것이 중요합니다.

## 샘플 코드

전체 코드: `examples/part5/tps_training/src/bin/33a_gltf_character.rs`

### 로딩 완료 후 그래프 구성

```rust
if !asset_server.is_loaded_with_dependencies(&fox_asset.0) {
    return;
}

let fox = gltfs.get(&fox_asset.0).expect("loaded glTF");
let (graph, nodes) = AnimationGraph::from_clips([
    fox.named_animations["Survey"].clone(),
    fox.named_animations["Walk"].clone(),
    fox.named_animations["Run"].clone(),
]);
```

### Scene 인스턴스가 준비된 뒤 재생 시작

```rust
fn prepare_animation_player(
    _ready: On<WorldInstanceReady>,
    mut commands: Commands,
    animations: Res<FoxAnimations>,
    player: Single<(Entity, &mut AnimationPlayer)>,
) {
    let (entity, mut player) = player.into_inner();
    let mut transitions = AnimationTransitions::new();
    transitions
        .play(&mut player, animations.survey, Duration::ZERO)
        .repeat();

    commands.entity(entity).insert((
        AnimationGraphHandle(animations.graph.clone()),
        transitions,
        Motion::Survey,
    ));
}
```

### 상태가 바뀔 때만 혼합 전환

```rust
if *current != desired {
    transitions
        .play(&mut player, clip, Duration::from_millis(220))
        .repeat();
    *current = desired;
}
```

## 코드 설명

게임 로직은 `Motion`만 결정합니다. `FoxAnimations`는 Motion과 `AnimationNodeIndex`의 대응을 보관하고, 애니메이션 시스템이 실제 Player를 제어합니다. 이렇게 분리하면 캐릭터 모델이나 클립 이름을 바꿔도 이동 입력 로직을 다시 작성할 필요가 없습니다.

`AnimationTransitions`를 사용하는 동안에는 `AnimationPlayer::play`를 별도로 호출하지 않습니다. Transition 컴포넌트가 현재·이전 클립의 가중치를 관리하므로 모든 전환을 같은 경로로 요청해야 합니다.

Fox 원본 단위는 이 실습 월드보다 크므로 Scene 루트에 `0.025` scale을 적용합니다. Blender에서 직접 내보낼 때는 다음을 확인하세요.

- Bevy는 Y-up 좌표계를 사용합니다.
- 내보내기 전에 Rotation과 Scale을 적용합니다.
- 뼈대와 Mesh가 같은 기준 축을 사용하는지 확인합니다.
- 애니메이션 이름을 Idle, Walk, Run처럼 안정적으로 지정합니다.
- 모델이 너무 크거나 작다면 먼저 Blender 단위와 적용된 Transform을 확인하고, 런타임 scale은 마지막 보정으로 사용합니다.

### 에셋 출처와 라이선스

Fox 모델은 KhronosGroup의 `glTF-Sample-Assets`에서 가져왔습니다.

- 모델: PixelMannen, CC0 1.0
- 리깅·애니메이션: tomkranis, CC BY 4.0
- glTF 변환: Asobo Studio와 scurest, CC BY 4.0

저장소에 원문 출처와 링크를 함께 보관합니다: `examples/part5/tps_training/assets/models/fox/SOURCE.md`

## 실습 과제

1. 이동하지 않을 때 Survey, 이동할 때 Walk가 선택되는지 확인하세요.
2. 이동 중 Shift를 누르고 떼며 Walk와 Run 사이의 0.22초 혼합을 관찰하세요.
3. 혼합 시간을 0초와 0.5초로 바꿔 전환 인상을 비교하세요.

## 심화 과제

`CharacterAnimationSet` 구조를 만들어 모델 경로, 클립 이름, 이동 속도별 재생 속도를 한곳에 모으세요. 로딩한 glTF에 필요한 이름이 없을 때 panic 대신 사용자에게 누락된 이름 목록을 보여 주도록 오류 처리를 추가하세요.

과제를 먼저 직접 수행한 뒤 필요하면 [힌트와 수행 예시](exercises/part5/33a_gltf_character_animation.md)를 확인하세요.

## 다음 챕터

다음 장에서는 Avian 3D 물리를 적용해 애니메이션 루트와 실제 충돌·중력 이동을 분리합니다.
