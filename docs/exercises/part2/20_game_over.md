# 20. 게임오버 과제 해설

[본문으로 돌아가기](../../20_GameOver.md#실습-과제)

## P2-C20-P1 · 생존 시간 표시

게임오버 진입 직전의 생존 시간을 Resource에 보존하고 `OnEnter(GameOver)`에서 화면 Text를 만듭니다. GameOver 중에 타이머를 계속 tick하면 최종 기록이 바뀌므로 Playing System에서만 갱신합니다.

## P2-C20-P2 · Escape로 종료

입력 System에서 `ButtonInput<KeyCode>::just_pressed(KeyCode::Escape)`를 확인하고 `AppExit::Success` Message를 보냅니다. 웹 빌드에서는 창 종료의 의미가 다르므로 플랫폼별 UX를 별도로 고려합니다.

## P2-C20-P3 · 재시작 시 적 Timer 초기화

점수와 플레이어만 초기화해서는 충분하지 않습니다. 적 생성 Timer에 `reset()`을 호출하거나 세션 Resource를 새 값으로 교체해 이전 경과 시간이 남지 않게 합니다.

## P2-C20-A1 · Menu와 Paused

상태를 `Menu`, `Playing`, `Paused`, `GameOver`로 나누고 게임 로직은 `in_state(GameState::Playing)`에서만 실행합니다. 일시정지 진입 시 `Time<Virtual>`을 pause하고 복귀 시 unpause하면 가상 시간 기반 Timer도 함께 멈춥니다.

UI 입력은 Paused 전용 System으로 분리합니다. 예제의 순수 `GameFlow`는 일시정지 중 전달된 10초를 누적하지 않으며, 복귀 후 남아 있던 0.25초만 더해 적 하나를 생성합니다. 이 테스트가 “복귀 순간 적이 몰려 나오는” 회귀를 막습니다.

### 선택 기준

- `run_if(in_state(...))`는 어떤 로직이 멈추는지 스케줄에서 명확히 보입니다.
- `Time<Virtual>` pause는 같은 시간원을 쓰는 여러 Timer를 함께 멈추기 좋습니다.
- 현실 시간으로 계속 돌아야 하는 네트워크·로그 로직은 `Time<Real>` 등 별도 시간원을 사용합니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin game_flow_solution
cargo test -p space_survivor --bin game_flow_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/game_flow_solution.rs`
