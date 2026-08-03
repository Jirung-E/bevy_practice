# 12G. ECS 테스트 전략 과제 해설

[본문으로 돌아가기](../../12G_EcsTesting.md#실습-과제)

## P1-C12G-P1 · 경계값

Health 10에 피해 10, Health 11에 피해 10을 각각 검사합니다. 결과값과 처치 여부를 별도 순수 함수 결과로 표현하면 경계 조건이 명확합니다.

## P1-C12G-P2 · Filter 회귀

Enemy가 없는 Health Entity를 함께 생성하고 Update 전후 값이 같은지 검사합니다. 전체 Entity 수가 아니라 Query 조건의 결과를 검증합니다.

## P1-C12G-P3 · 중복 처치 방지

체력이 이번 프레임에 0이 된 경우만 `Defeated`를 증가시키거나, `Defeated` marker를 삽입하고 `Without<Defeated>`로 제외합니다. App을 두 번 업데이트해 증가량이 한 번인지 확인합니다.

## P1-C12G-A1 · 시나리오 Runner

tick별 입력 목록과 seed를 인수로 받고 매 tick FixedUpdate를 실행합니다. 마지막 결과뿐 아니라 실패한 tick의 command와 핵심 상태 hash를 남기면 재현과 회귀 분석이 쉬워집니다.
