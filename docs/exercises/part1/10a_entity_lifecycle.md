# 10A. Entity 수명과 제거 감지 과제 해설

[본문으로 돌아가기](../../10A_EntityLifecycle.md#실습-과제)

## P1-C10A-P1 · Shield 제거 관찰

`RemovedComponents<Shield>`는 Health Reader와 독립적입니다. 제거 횟수를 타입별 필드로 나누고, 한 Entity에서 두 Component를 같은 프레임에 제거해도 각각 한 번 기록되는지 확인합니다.

## P1-C10A-P2 · 사망 연출 중간 상태

Health를 제거한 프레임에는 Entity를 바로 despawn하지 않고 `Defeated`와 `DespawnTimer`를 삽입합니다. 이동·충돌 Query는 `Without<Defeated>`로 제외하고 연출 Timer가 끝난 뒤 despawn합니다.

## P1-C10A-P3 · 추적 대상 구분

제거 Reader가 돌려준 Entity와 `TrackedTarget.0`이 같은 경우에만 추적 상태를 바꿉니다. 다른 적의 제거 이벤트가 전역 lock-on을 해제하지 않는지 두 Entity로 테스트합니다.

## P1-C10A-A1 · 여러 참조 소유자

제거 감지를 `TargetRemoved(Entity)` Message로 변환하고 UI, AI, 카메라가 각자 Reader를 가집니다. 한 소비자가 메시지를 읽어도 다른 Reader의 메시지가 사라지지 않는지 확인합니다.
