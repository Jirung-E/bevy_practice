# 17. 게임 UI 과제 해설

[본문으로 돌아가기](../../17_GameUI.md#실습-과제)

## P2-C17-P1 · HUD를 오른쪽 위로 옮기기

HUD의 `Node`에서 `left` 대신 `right`를 지정합니다. 텍스트 정렬만 바꾸면 배치 영역은 그대로이므로, 먼저 레이아웃 기준점을 옮겨야 합니다.

## P2-C17-P2 · 체력 1일 때 경고색

HUD 갱신 Query에 `TextColor`를 포함하고 현재 체력에 따라 색을 선택합니다. 점수까지 같은 색이 되지 않게 체력 Text에 별도 표식 Component를 붙이는 편이 안전합니다.

## P2-C17-P3 · 생존 시간

게임 플레이 중에만 증가하는 Resource를 만들고 중앙 상단 Text와 연결합니다. 프레임 수가 아니라 `Time::delta_secs()`를 더해야 컴퓨터 성능과 무관합니다.

## P2-C17-A1 · 체력 아이콘과 변경 감지

체력 아이콘 세 개를 UI 자식으로 미리 만들고 `Visibility`만 바꾸면 매번 despawn과 spawn을 반복하지 않아도 됩니다. `Changed<PlayerHealth>` 조건 또는 마지막 체력을 기억하는 캐시를 사용해 값이 바뀐 프레임에만 갱신합니다.

수행 예시의 `HeartHud::update_if_changed`는 같은 체력이 들어오면 `false`를 반환합니다. 테스트는 두 번째 호출에서 실제 변경이 없음을 확인합니다.

### 선택 기준

- 아이콘 수가 고정이면 미리 생성 후 `Visibility` 변경이 단순합니다.
- 최대 체력이 자주 달라지면 필요한 차이만큼 자식을 추가·제거하는 방식이 낫습니다.
- 화면 갱신 최적화는 먼저 변경 조건을 좁히고, Entity 재구성은 그다음에 고려합니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin game_flow_solution
cargo test -p space_survivor --bin game_flow_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/game_flow_solution.rs`
