# 12C. Scene과 Save Game 설계

## 학습 목표

- `DynamicWorld` 기반 Scene과 전용 `SaveGame` 구조체가 해결하는 문제를 구분할 수 있다.
- serde와 RON으로 버전 있는 저장 모델을 직렬화할 수 있다.
- 이전 버전 저장 데이터를 현재 구조로 명시적으로 마이그레이션할 수 있다.
- 손상되거나 지원하지 않는 버전에서 기본값으로 안전하게 복구할 수 있다.

## 이 내용으로 만들 수 있는 것

- 반복 배치되는 레벨·프리팹은 Scene으로, 플레이 진행은 안정적인 SaveGame 구조로 저장할 수 있습니다.
- 런타임 Handle과 임시 상태를 제외하고 버전 변경에 대응하는 저장 형식을 설계할 수 있습니다.

## 이번에 만들 결과물

버전 2 SaveGame을 RON으로 왕복하고, 필드가 적은 버전 1 데이터를 현재 구조로 마이그레이션하는 콘솔 예제를 만듭니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin save_game_model
cargo test -p ecs_basics --bin save_game_model
```

이 챕터는 저장 형식의 경계를 먼저 설계합니다. 실제 2D 창에서 플레이어 위치·체력·진행 상태를 저장하는 적용은 [19A](19A_SaveGameRoundTrip.md)에서 완성합니다.

## 핵심 개념

### Scene과 SaveGame은 저장 대상이 다르다

| 구분 | DynamicWorld(Scene) | 전용 SaveGame |
|---|---|---|
| 주 대상 | Entity, Component, Resource 구성 | 플레이 진행과 사용자 설정 |
| 구조 기준 | 현재 ECS 타입과 반사 등록 | 장기 유지할 명시적 데이터 계약 |
| 적합한 예 | 레벨, 프리팹, 에디터 월드 | 점수, 체력, 위치, 스테이지 |
| Entity 참조 | 복원 시 Entity 매핑 필요 | 안정 ID나 도메인 ID 사용 |
| 변경 대응 | 타입 경로·필드 변화에 민감 | 버전별 migration 함수 |
| 저장 범위 | allowlist/denylist로 선택 | 구조체 필드 자체가 allowlist |

Scene 전체를 SaveGame으로 쓰면 렌더링·캐시·타이머 같은 런타임 Component가 진행 데이터와 결합되기 쉽습니다. 반대로 레벨의 수백 Entity를 전용 구조체에 일일이 옮기면 ECS 구성을 중복 정의하게 됩니다.

두 형식은 경쟁 관계가 아닙니다.

```text
level.scn.ron      → 레벨 Entity와 Component 구성
save_game.ron      → 현재 레벨 ID, 플레이어 상태, 진행도
settings.ron       → 음량, 입력, 접근성 설정
```

### 저장 버전은 읽기 전에 검사한다

버전 1은 점수와 위치만 저장한다고 가정합니다.

```rust
struct SaveGameV1 {
    version: u32,
    score: u32,
    player_x: f32,
    player_y: f32,
}
```

버전 2는 체력, 최고 점수, 진행 상태를 추가합니다.

```rust
struct SaveGame {
    version: u32,
    score: u32,
    high_score: u32,
    player: SavedPlayer,
    progress: SavedProgress,
}
```

구버전 파일을 현재 구조체로 바로 역직렬화하며 serde 기본값에만 의존하지 않습니다. 먼저 V2를 시도하고 버전이 맞는지 확인한 뒤, V1이면 `From<SaveGameV1>` 마이그레이션에서 새 필드의 의미 있는 기본값을 결정합니다.

### Entity ID를 직접 저장하지 않는다

`Entity`는 현재 World에서만 유효한 세대가 포함된 런타임 식별자입니다. 앱을 다시 실행하면 같은 숫자가 같은 대상을 뜻한다고 보장되지 않습니다.

SaveGame에는 다음 중 하나를 저장하세요.

- `"forest_entrance"` 같은 안정적인 레벨·spawn point ID
- 게임이 생성한 UUID
- 인벤토리 정의를 가리키는 `"potion_small"` 같은 데이터 ID

Scene 내부 Entity 관계는 [12B](12B_ReflectDynamicWorld.md)에서 설명한 Entity 매핑을 사용합니다.

## 샘플 코드

```rust
fn decode(source: &str) -> Result<(SaveGame, LoadOrigin), String> {
    if let Ok(current) = ron::from_str::<SaveGame>(source)
        && current.version == SAVE_VERSION
    {
        return Ok((current, LoadOrigin::Current));
    }

    if let Ok(old) = ron::from_str::<SaveGameV1>(source)
        && old.version == 1
    {
        return Ok((old.into(), LoadOrigin::MigratedV1));
    }

    Err("unsupported or damaged save data".to_owned())
}
```

손상 데이터는 앱을 종료시키지 않고 오류와 함께 기본값을 반환합니다.

```rust
fn load_or_default(source: &str) -> (SaveGame, Option<String>) {
    match decode(source) {
        Ok((save, _)) => (save, None),
        Err(error) => (SaveGame::default(), Some(error)),
    }
}
```

전체 코드: [Part 1 전체 코드의 12C](source/part1.md#12c--scene과-save-game-설계)

## 코드 설명

- `#[serde(deny_unknown_fields)]`는 예상하지 않은 구조를 조용히 받아들이지 않습니다.
- `version`이 현재 값과 일치해야 Current로 인정합니다.
- V1 마이그레이션은 기존 score를 high score로 옮기고 새 health와 stage를 명시적으로 초기화합니다.
- 지원하지 않는 미래 버전을 현재 코드가 임의로 읽지 않습니다.
- 손상 fallback은 오류를 호출자에게 남겨 UI와 로그에 표시할 수 있게 합니다.
- 기본값으로 복구하더라도 손상된 원본 파일을 즉시 덮어쓰면 안 됩니다.

자동 저장은 플레이어가 잊지 않아 편하지만 잘못된 상태나 손상 데이터를 덮어쓸 수 있습니다. 명시적 저장은 통제하기 쉽지만 진행 손실 위험이 있습니다. 실전에서는 체크포인트 자동 저장과 사용자 슬롯 저장을 함께 쓰고, 임시 파일·백업·롤백을 적용합니다.

저장 위치도 플랫폼마다 다릅니다. 현재 작업 디렉터리 상대 경로는 실습에는 간단하지만 설치된 앱에는 적합하지 않습니다.

| 플랫폼 | 일반적인 사용자 데이터 위치 |
|---|---|
| Windows | `%APPDATA%/제작사/게임` 또는 Saved Games |
| Linux | `$XDG_DATA_HOME` 또는 `~/.local/share` |
| macOS | `~/Library/Application Support` |
| WASM | 브라우저 저장소 API 또는 서버 저장 |

## 실습 과제

1. SaveGame에 음악·효과음 볼륨 설정을 추가하고 버전 2 왕복 테스트를 보강하세요.
2. 버전 1 마이그레이션에서 체력 기본값을 5로 바꾸고 기존 테스트가 실패한 뒤 새 정책에 맞게 수정하세요.
3. 지원하지 않는 버전과 문법이 손상된 RON을 서로 다른 오류 메시지로 구분하세요.

## 심화 과제

V1 → V2 → V3처럼 한 단계씩 이동하는 migration pipeline을 설계하세요. 마이그레이션 전 원본 백업, 현재 버전으로 다시 저장하는 시점, migration 실패 시 사용자에게 제공할 복구 선택지를 포함하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/12c_scene_save_game.md)를 확인하세요.

## 다음 챕터

[12D. ECS 동작 추상화와 스킬 시스템](12D_BehaviorAbstraction.md)에서 Component·System·Query·Message를 하나의 확장 가능한 실행 구조로 결합합니다. Part 2에서 플레이 가능한 2D 게임을 만든 뒤 [19A. 게임 상태 저장과 불러오기](19A_SaveGameRoundTrip.md)에서 저장 모델을 실제 Session과 연결합니다.
