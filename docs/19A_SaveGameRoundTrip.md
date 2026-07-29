# 19A. 게임 상태 저장과 불러오기

## 선수 지식

- [12B. Reflect와 DynamicWorld(Scene)](12B_ReflectDynamicWorld.md)
- [12C. Scene과 Save Game 설계](12C_SceneAndSaveGame.md)
- [19. 최고 점수 저장](19_Save.md)

Scene 개념은 이 챕터에서 처음 배우지 않습니다. 여기서는 전용 SaveGame을 실제 2D 플레이 상태에 적용합니다.

## 학습 목표

- 플레이어 위치·체력·점수·진행 상태를 SaveGame으로 변환할 수 있다.
- 명시적 저장과 시작 시 자동 불러오기를 구현할 수 있다.
- 임시 파일, 백업, rename으로 기존 저장 파일을 보호할 수 있다.
- 구버전과 손상 파일을 재현하고 안전한 복구 흐름을 확인할 수 있다.

## 이번에 만들 결과물

파란 플레이어를 움직이고 상태를 바꾼 뒤 F5로 저장합니다. 앱을 다시 실행하거나 R로 Session을 초기화한 뒤 F9를 누르면 저장 당시 위치와 전체 진행 데이터가 복원됩니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 19a_save_game
```

조작법:

| 입력 | 동작 |
|---|---|
| WASD | 플레이어 이동 |
| H | 체력 1 감소 |
| K | 점수 100·처치 수 1 증가 |
| P | 스테이지 1 증가 |
| F5 | 현재 버전 원자적 저장 |
| F9 | 파일 불러오기 |
| F6 | 마이그레이션 실습용 V1 파일 생성 |
| F7 | fallback 실습용 손상 파일 생성 |
| R | 메모리 Session만 초기화 |

## 핵심 개념

게임의 ECS/Resource 구조를 그대로 serde에 노출하지 않고 저장 경계에서 변환합니다.

```text
Session Resource
  score, high_score
  player_position: Vec2
  health, stage, defeated
  status: UI 전용
            │
            │ Session::to_save()
            ▼
SaveGame V2
  score, high_score
  SavedPlayer { [f32; 2], health }
  SavedProgress { stage, defeated }
            │
            ▼ RON + 임시 파일 교체
         save.ron
```

UI 상태 문자열은 저장하지 않습니다. Bevy `Vec2`도 장기 형식에 직접 결합하지 않고 `[f32; 2]`로 변환합니다. 나중에 렌더링·ECS 구조가 바뀌어도 SaveGame 계약을 독립적으로 마이그레이션할 수 있습니다.

### 안전한 파일 교체

1. 같은 디렉터리의 `save.ron.tmp`에 전체 내용을 씁니다.
2. `sync_all`로 파일 쓰기를 운영체제에 요청합니다.
3. 기존 파일을 `save.ron.bak`으로 이동합니다.
4. 임시 파일을 `save.ron`으로 rename합니다.
5. 성공하면 backup을 지우고, 실패하면 기존 파일을 복구합니다.

표준 `rename`의 기존 대상 교체 동작은 운영체제마다 다릅니다. 예제는 Windows에서도 동작하도록 backup/rollback 단계를 명시합니다. 더 강한 원자성을 요구하면 플랫폼별 atomic replace API나 검증된 저장 라이브러리를 사용하세요.

## 샘플 코드

```rust
impl Session {
    fn to_save(&self) -> SaveGame {
        SaveGame {
            version: SAVE_VERSION,
            score: self.score,
            high_score: self.high_score.max(self.score),
            player: SavedPlayer {
                position: self.player_position.to_array(),
                health: self.health,
            },
            progress: SavedProgress {
                stage: self.stage,
                defeated_enemies: self.defeated_enemies,
            },
        }
    }
}
```

불러오기는 파일 없음, 현재 버전, 마이그레이션, fallback을 구분합니다.

```rust
fn load_path(path: &Path) -> (SaveGame, LoadOrigin, Option<String>) {
    if !path.exists() {
        return (SaveGame::default(), LoadOrigin::NewGame, None);
    }

    match fs::read_to_string(path).ok().and_then(|source| decode(&source).ok()) {
        Some((save, origin)) => (save, origin, None),
        None => (
            SaveGame::default(),
            LoadOrigin::Fallback,
            Some("load failed".to_owned()),
        ),
    }
}
```

실제 전체 코드는 오류 원인을 보존하므로 위 축약 코드보다 상세합니다.

전체 코드: [Part 2 전체 코드의 19A](source/part2.md#19a--게임-상태-저장과-불러오기)

## 코드 설명

- 앱 시작 시 파일이 없으면 NewGame, 있으면 자동으로 읽습니다.
- F5는 현재 Session snapshot을 명시적으로 저장합니다.
- F9는 성공했을 때만 저장 데이터를 Session에 반영합니다.
- F6는 실제 버전 1 RON을 만들어 V1 → V2 migration을 눈으로 확인하게 합니다.
- F7은 손상된 원본을 덮어쓰지 않은 채 fallback UI를 확인하게 합니다.
- `status`는 런타임 UI 데이터이므로 SaveGame에 포함하지 않습니다.
- 최고 점수는 현재 score보다 작아지지 않게 snapshot 시 보정합니다.
- 저장 경로는 `BEVY_PRACTICE_SAVE_PATH`로 재정의할 수 있습니다.
- 기본 실습 경로는 workspace의 `target/19a_save_game/save.ron`이라 Git에 포함되지 않습니다.

파일 읽기·쓰기는 blocking API입니다. 이 작은 실습 파일에는 충분하지만 큰 Scene이나 자동 저장에는 `IoTaskPool`을 사용하고, 완료 결과만 메인 World에 적용해야 합니다.

WASM에서는 일반 데스크톱 파일 경로를 사용할 수 없습니다. 브라우저 저장소나 서버 API에 연결하고 사용자 동의·용량·동기화 정책을 별도로 설계합니다.

## 실습 과제

1. F8에 자동 저장 체크포인트를 추가하고 스테이지가 바뀔 때만 저장하세요.
2. 저장 슬롯을 3개로 늘리고 현재 선택 슬롯을 HUD에 표시하세요.
3. 불러오기 전에 현재 Session과 저장 파일의 점수·스테이지 차이를 확인하는 미리보기를 추가하세요.

## 심화 과제

저장 작업을 `IoTaskPool`로 옮기고 저장 중 중복 요청을 합치는 queue를 만드세요. 완료·실패 Message를 UI에 전달하고 앱 종료 요청 시 진행 중 저장을 어떻게 처리할지 정책과 테스트를 작성하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/19a_save_game_roundtrip.md)를 확인하세요.

## 다음 챕터

저장된 최고 점수와 진행 상태를 유지한 채 [20. 게임오버와 재시작](20_GameOver.md) 흐름으로 연결합니다.
