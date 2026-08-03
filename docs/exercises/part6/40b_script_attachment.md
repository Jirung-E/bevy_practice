# 40B 선택형 과제 해설

새 명령을 추가할 때는 세 위치를 함께 확인합니다.

1. 역직렬화할 `ScriptCommand` variant
2. 기준값이 필요하다면 Entity에 저장할 Component
3. 실행 System의 명령 처리 분기

크기나 위치를 현재 값에 계속 곱하거나 더하면 프레임 수에 따라 오차가 누적됩니다. `ScriptOrigin`처럼 원래 값을 보관하고 경과 시간에서 현재 값을 계산하세요.

Inspector 연결 UI에서는 사용자가 입력한 문자열을 곧바로 임의 파일 접근에 쓰지 말고 Asset 경로 정책을 적용합니다. 새 Script가 정상 로드됐다는 `LoadedWithDependencies`를 확인한 뒤 기존 Handle을 교체하면 잘못된 파일을 선택해도 현재 동작을 보존할 수 있습니다.
