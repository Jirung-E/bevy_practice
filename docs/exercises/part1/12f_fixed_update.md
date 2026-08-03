# 12F. FixedUpdate 과제 해설

[본문으로 돌아가기](../../12F_FixedUpdate.md#실습-과제)

## P1-C12F-P1 · 주파수 비교

30Hz의 timestep은 `1 / 30`초입니다. 60Hz 두 tick과 30Hz 한 tick은 같은 실제 시간 동안 속도 6에서 모두 0.2 이동해야 합니다.

## P1-C12F-P2 · Queue 순서

서로 다른 ID를 가진 Attack 명령 두 개를 넣고 `pop_front` 결과가 삽입 순서와 같은지 기록합니다. `VecDeque`를 stack처럼 사용하지 않도록 주의합니다.

## P1-C12F-P3 · 유지형과 일회성 입력

Move 방향은 최신 값을 덮어쓰는 Resource가 적합하고 Attack은 발생 횟수를 보존하는 queue가 적합합니다. 모든 입력을 같은 queue 정책으로 처리할 필요는 없습니다.

## P1-C12F-A1 · 표시 보간

시뮬레이션 Entity에 이전·현재 위치를 저장하고 화면 Entity의 Transform만 두 값 사이에서 보간합니다. 표시 System이 authoritative simulation 위치를 덮어쓰지 않는지 테스트합니다.
