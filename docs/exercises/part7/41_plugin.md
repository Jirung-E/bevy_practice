# 41. Plugin 과제 해설

[본문으로 돌아가기](../../41_Plugin.md#실습-과제)

## P7-C41-P1 · PausePlugin

PausePlugin은 P 키 설정과 상태 전환만 소유하고 Gameplay 내부 데이터를 직접 수정하지 않습니다. 입력 매핑 Resource를 등록하면 키 변경이 Plugin 밖으로 퍼지지 않습니다.

## P7-C41-P2 · Presentation 없는 App

공통 Core·Assets·Gameplay만 추가하는 구성을 만들고 창·카메라·UI 없이 테스트가 실행되는지 확인합니다. headless라고 해서 Gameplay 규칙까지 빠지면 같은 프로그램을 검증하는 것이 아닙니다.

## P7-C41-P3 · 소유권 표

| Plugin | 소유 데이터 |
|---|---|
| Core | 공통 State, 시간 정책 |
| Assets | 에셋 catalog와 loading 상태 |
| Gameplay | 플레이어·적 Component, 점수·쿨다운 |
| Presentation | 카메라, Mesh/UI/오디오 |
| Pause | 입력 매핑, pause 전환 |
| Diagnostics | 측정 Resource와 출력 |

## P7-C41-A1 · 프로필별 PluginGroup

수행 예제는 Client에 Presentation/Pause를 포함하고, DedicatedServer와 AutomatedTest에서는 제외합니다. 서버는 진단을 유지하고 자동 테스트는 가장 작은 공통 구성만 사용합니다.

공통 Plugin 순서는 의존 대상이 먼저 오게 하며, 전용 Plugin이 공통 Plugin을 역으로 참조하지 않게 합니다. “서버 플래그가 켜졌으니 Presentation System이 실행되지 않는다”보다 아예 Plugin을 추가하지 않는 구성이 명확합니다.

## 전체 코드 실행

```bash
cargo test -p production_structure --bin production_solution
```

전체 코드: `examples/part7/production_structure/src/bin/production_solution.rs`
