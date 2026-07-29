# 19A. 게임 상태 저장과 불러오기 과제 해설

[본문으로 돌아가기](../../19A_SaveGameRoundTrip.md#실습-과제)

## P2-C19A-P1 · 스테이지 체크포인트

매 프레임 저장하지 말고 마지막으로 저장한 stage를 기억합니다. stage 변경을 감지하면 저장 요청을 만들고, 실제 파일 쓰기는 한 System 또는 작업 queue가 담당하게 합니다.

## P2-C19A-P2 · 3개 슬롯

파일 이름을 사용자 입력 문자열로 직접 만들지 말고 `SaveSlot(0..3)`처럼 검증된 값에서 `slot-1.ron` 경로를 생성합니다. 슬롯마다 마지막 저장 시각, 점수, 스테이지 metadata를 별도 목록으로 읽으면 선택 UI가 단순해집니다.

## P2-C19A-P3 · 불러오기 미리보기

파일을 곧바로 Session에 적용하지 말고 `PendingLoad(SaveGame)`에 보관합니다. HUD에 현재/저장 점수와 stage를 표시하고 확인 입력에서만 적용합니다.

## P2-C19A-A1 · 비동기 저장 queue

저장 요청마다 파일 작업을 동시에 시작하면 오래된 snapshot이 마지막에 완료되어 최신 파일을 덮을 수 있습니다. 한 번에 하나만 실행하고 대기 중에는 가장 최신 snapshot 하나로 합치는 queue가 실용적입니다.

- `Idle`: 요청을 받으면 Task 시작
- `Saving`: 새 요청은 `pending_latest`를 교체
- 완료: 결과 Message 전송 후 pending이 있으면 다음 Task 시작
- 종료: 저장 완료를 기다릴지, timeout 뒤 종료할지 플랫폼 정책 적용

파일 쓰기 함수 자체는 현재 예제의 `save_atomic`을 재사용합니다. UI는 Task 완료 Message만 읽어 `SAVED` 또는 `SAVE ERROR`를 표시합니다.

## 실행 가능한 수행 예시

```bash
cargo run -p space_survivor --bin 19a_save_game
cargo test -p space_survivor --bin 19a_save_game
```

전체 코드: `examples/part2/space_survivor/src/bin/19a_save_game.rs`
