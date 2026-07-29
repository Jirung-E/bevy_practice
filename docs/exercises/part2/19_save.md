# 19. 저장 과제 해설

[본문으로 돌아가기](../../19_Save.md#실습-과제)

## P2-C19-P1 · 저장 파일 확인

최고 점수를 갱신한 뒤 게임을 정상 종료하고 저장 경로의 파일을 직접 엽니다. 실행 작업 디렉터리에 따라 상대 경로가 달라질 수 있으므로 로그에 최종 경로를 출력하면 확인이 쉽습니다.

## P2-C19-P2 · 손상된 파일의 기본값

읽기 실패와 파싱 실패를 구분해 로그로 남기되, 게임 시작은 기본값으로 계속합니다. 손상된 파일을 즉시 덮어쓰지 말아야 원인을 조사하거나 복구할 여지가 남습니다.

## P2-C19-P3 · 마지막 점수

최고 점수와 마지막 점수는 의미가 다릅니다. `high_score = high_score.max(score)`로 갱신하고 `last_score = score`는 매 게임 종료 시 그대로 기록합니다.

## P2-C19-A1 · 버전 저장과 교체

수행 예시는 `version`, 설정, 최고 점수, 마지막 점수, 통계를 `SaveGame`에 모아 RON으로 직렬화합니다. `version`이 달라지면 곧바로 역직렬화만 시도하지 말고 버전별 구조를 읽어 현재 구조로 변환하는 마이그레이션 경로를 둡니다.

저장은 다음 순서를 사용합니다.

1. 같은 디렉터리의 `.tmp` 파일에 전부 쓰고 `sync_all`합니다.
2. 기존 파일이 있으면 `.bak`으로 이동합니다.
3. `.tmp`를 본 파일명으로 rename합니다.
4. 성공하면 백업을 지우고, 실패하면 백업을 복구합니다.

표준 라이브러리의 rename은 Windows에서 기존 대상 덮어쓰기 동작이 다르므로 백업과 롤백을 명시했습니다. 운영체제가 제공하는 원자적 replace API를 쓰면 더 강한 보장을 얻지만 플랫폼별 구현이 필요합니다.

### 선택 기준

- 사람이 읽고 고치기 쉬운 저장은 RON/JSON이 편합니다.
- 파일 크기와 속도가 중요하면 바이너리 형식을 고려하되 버전 필드는 유지합니다.
- 중요한 데이터는 앱 데이터 디렉터리, 백업 정책, 쓰기 실패 안내까지 함께 설계합니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin game_flow_solution
cargo test -p space_survivor --bin game_flow_solution
```

PowerShell에서 실제 파일 교체까지 확인하려면 저장 경로를 명시합니다.

```powershell
$env:BEVY_PRACTICE_WRITE_SAVE = "target/practice-save.ron"
cargo run -p space_survivor --bin game_flow_solution
Remove-Item Env:BEVY_PRACTICE_WRITE_SAVE
```

전체 코드: `examples/part2/space_survivor/src/bin/game_flow_solution.rs`
