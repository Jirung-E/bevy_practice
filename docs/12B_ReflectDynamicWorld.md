# 12B. Reflect와 DynamicWorld(Scene)

## 학습 목표

- `Reflect`와 `App::register_type`이 런타임 타입 정보에 어떤 역할을 하는지 설명할 수 있다.
- Bevy 0.19의 `DynamicWorld`로 ECS Component를 RON에 직렬화하고 새 World에 복원할 수 있다.
- 저장할 Component와 실행할 때만 필요한 Component를 분리할 수 있다.
- 저장된 Entity ID가 새 실행의 Entity ID와 같다고 가정하면 안 되는 이유를 이해한다.

## 이번에 만들 결과물

카메라와 렌더링 Plugin 없이 `Position`과 `Health`를 가진 Entity 두 개를 만듭니다. 이 ECS 데이터를 `12b_dynamic_world.scn.ron` 파일에 저장한 뒤 새 World로 읽어 원래 값이 복원되는지 확인합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin dynamic_world
```

실행 결과는 RON 원문과 복원된 값을 터미널에 출력합니다.

```text
saved: .../target/12b_dynamic_world.scn.ron
restored position=(-2, 1.5), health=80, runtime_session=false
restored position=(4, -3), health=35, runtime_session=false
```

창을 만들지 않으므로 `Camera2d`, `Camera3d`, Mesh, Sprite는 전혀 필요하지 않습니다.

## 핵심 개념

### Bevy 0.19의 이름 변경

이전 Bevy 자료에서 `DynamicScene`과 `DynamicSceneBuilder`라는 이름을 볼 수 있습니다. Bevy 0.19에서는 ECS World 직렬화 기능이 `bevy_world_serialization`으로 분리되면서 핵심 타입 이름이 `DynamicWorld`와 `DynamicWorldBuilder`로 바뀌었습니다.

이 교재에서 Scene은 “2D 화면”이나 “3D 모델”을 뜻하지 않습니다. 직렬화 가능한 Entity, Component, Resource의 묶음을 뜻합니다.

```text
ECS World
  ├─ Entity A
  │   ├─ Position
  │   ├─ Health
  │   └─ RuntimeSession  ── 저장 대상 아님
  └─ Entity B
      ├─ Position
      └─ Health
             │
             ▼ Reflect + TypeRegistry
        DynamicWorld
             │
             ▼ RON
       .scn.ron 파일
```

### Reflect와 Component 등록은 다른 역할이다

```rust
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

- `Component`는 이 타입을 ECS Entity에 붙일 수 있게 합니다.
- `Reflect`는 필드 구조와 값을 런타임에 조사하고 복제할 수 있게 합니다.
- `#[reflect(Component)]`는 반사된 값을 Component로 다시 삽입하는 동작을 TypeRegistry에 등록합니다.
- `app.register_type::<Position>()`은 실제 앱의 TypeRegistry에 이 타입 정보를 넣습니다.

derive만 하고 `register_type`을 빼면 저장 파일을 읽을 때 타입 경로를 실제 Rust 타입으로 복원할 수 없습니다.

### Entity ID는 영구 식별자가 아니다

RON에는 Entity 참조를 다시 연결하기 위한 ID가 나타나지만, 이것은 저장 파일 바깥에서 영구 ID로 사용하라는 뜻이 아닙니다. `write_to_world`는 `EntityHashMap`을 사용해 저장 당시 Entity를 새 World의 Entity로 매핑합니다.

게임의 퀘스트 대상이나 아이템처럼 실행을 넘어 유지할 식별자는 별도 UUID, 문자열 ID, 데이터베이스 키처럼 명시적인 안정 ID를 저장하세요.

`ChildOf`나 사용자 정의 Entity 참조 Component도 복원 과정에서 매핑되어야 합니다. 사용자 정의 참조 타입은 `MapEntities`와 반사 등록이 필요합니다. 관계를 단순 숫자 Entity ID로 저장하면 새 World에서 엉뚱한 대상을 가리킬 수 있습니다.

## 샘플 코드

```rust
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Health(u32);

fn registered_app() -> App {
    let mut app = App::new();
    app.register_type::<Position>().register_type::<Health>();
    app
}

fn serialize_world(app: &App) -> Result<String, ron::Error> {
    let world = app.world();
    let registry = world.resource::<AppTypeRegistry>().read();
    DynamicWorld::from_world_with(world, &registry).serialize(&registry)
}
```

복원할 때는 같은 타입을 등록한 새 App과 `WorldDeserializer`를 사용합니다.

```rust
let dynamic_world = WorldDeserializer {
    type_registry: &registry,
    load_from_path: &mut NoAssetLoader,
}
.deserialize(&mut ron_deserializer)?;

dynamic_world.write_to_world(
    app.world_mut(),
    &mut EntityHashMap::default(),
)?;
```

전체 코드: [Part 1 전체 코드의 12B](source/part1.md#12b--reflect와-dynamicworldscene)

## 코드 설명

- `App::new()`만 사용하므로 창이나 렌더 그래프가 생성되지 않습니다.
- `Position`과 `Health`는 `Reflect`와 `#[reflect(Component)]`를 갖고 TypeRegistry에 등록됩니다.
- `RuntimeSession`은 평범한 Component지만 반사·등록하지 않아 RON에 들어가지 않습니다.
- `DynamicWorld::from_world_with`는 등록되어 있고 반사 가능한 Component를 World에서 추출합니다.
- `serialize`은 사람이 읽을 수 있는 Bevy World RON을 만듭니다.
- `WorldDeserializer`는 RON의 타입 경로를 TypeRegistry에서 찾아 동적 Component로 복원합니다.
- `write_to_world`는 새 Entity를 만들고 저장된 Component를 삽입합니다.
- 에셋 Handle이 없는 예제이므로 `NoAssetLoader`는 호출되지 않습니다. Handle을 저장하는 실제 Scene에서는 AssetServer 기반 loader가 필요합니다.
- 손상된 RON은 panic 대신 `Result::Err`로 전달합니다.

Component 타입이나 필드 이름을 바꾸면 기존 RON의 타입 경로 또는 필드가 더 이상 맞지 않을 수 있습니다. 장기간 유지할 파일 형식에는 버전과 마이그레이션 정책이 필요합니다. [12C. Scene과 Save Game 설계](12C_SceneAndSaveGame.md)에서 전용 저장 모델과 함께 다룹니다.

## 실습 과제

1. `Mana(u32)` Component를 반사·등록하고 두 Entity 중 하나에만 추가한 뒤 RON에서 타입과 값이 보이는지 확인하세요.
2. 저장된 RON의 Health 값을 직접 바꾸고 다시 실행해 수정한 값이 복원되는지 확인하세요.
3. RON 일부를 손상시켰을 때 앱이 panic하지 않고 오류 메시지와 빈 기본 World를 선택하도록 처리하세요.

## 심화 과제

`DynamicWorldBuilder`를 사용해 기본 자동 추출 대신 `Position`, `Health`, `Mana`만 허용하는 allowlist 저장 함수를 만드세요. 새 런타임 Component가 추가되어도 저장 형식에 우연히 포함되지 않는지 자동 테스트하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/12b_reflect_dynamic_world.md)를 확인하세요.

## 다음 챕터

[12C. Scene과 Save Game 설계](12C_SceneAndSaveGame.md)에서 여기서 배운 Scene/DynamicWorld와 플레이 진행용 SaveGame 구조체를 구분합니다.
