# 12C. Scene과 Save Game 설계 과제 해설

[본문으로 돌아가기](../../12C_SceneAndSaveGame.md#실습-과제)

## P1-C12C-P1 · 설정 추가

진행 데이터와 설정의 변경 주기가 다르면 `SaveGame` 안의 `settings` 하위 구조로 묶거나 별도 Settings 파일로 분리할 수 있습니다. 같은 파일에 넣을 때도 `AudioSettings { music_volume, effects_volume }`처럼 의미를 가진 구조를 사용하고 0~1 범위를 검증하세요.

## P1-C12C-P2 · migration 기본값 변경

테스트가 먼저 실패하는 것은 기존 migration 계약이 바뀌었다는 신호입니다. 단순히 예상값만 5로 바꾸기 전에 “구버전 사용자는 왜 최대 체력 5로 시작하는가”를 정책에 기록합니다.

## P1-C12C-P3 · 오류 구분

RON parser 오류와 지원하지 않는 `version`을 별도 enum으로 표현합니다.

```rust
enum SaveLoadError {
    Damaged(String),
    UnsupportedVersion(u32),
}
```

문자열 하나보다 UI 문구·로그 수준·복구 행동을 안정적으로 선택할 수 있습니다.

## P1-C12C-A1 · 단계별 migration

V1을 바로 V3로 만드는 거대한 함수보다 `V1 -> V2`, `V2 -> V3` 변환을 순서대로 적용합니다. 각 단계에는 이전 버전 fixture와 예상 결과 테스트를 둡니다.

- migration 전 원본을 `.bak`으로 보존합니다.
- migration 성공 뒤 현재 버전 파일을 즉시 쓸지 다음 명시적 저장까지 기다릴지 정합니다.
- 실패하면 원본을 덮어쓰지 않고 새 게임, 재시도, 파일 위치 열기 같은 선택지를 제공합니다.
- downgrade는 자동 지원한다고 가정하지 않습니다.

## 실행 가능한 수행 예시

```bash
cargo run -p ecs_basics --bin save_game_model
cargo test -p ecs_basics --bin save_game_model
```

전체 코드: `examples/part1/ecs_basics/src/bin/12c_save_game_model.rs`
