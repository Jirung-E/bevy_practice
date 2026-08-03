# 44A. 결정론적 시뮬레이션과 입력 리플레이

## 학습 목표

- 같은 입력이 같은 결과를 만든다는 결정론의 조건을 설명합니다.
- 입력을 렌더 프레임이 아니라 고정 tick 단위로 기록합니다.
- 난수 seed와 실행 순서를 고정해 기록을 재생합니다.
- 최종 상태 비교로 재현 실패를 자동 검출합니다.

## 이 내용으로 만들 수 있는 것

- 버그가 발생한 플레이를 그대로 다시 실행하는 재현 파일
- 고스트·리플레이와 자동 회귀 테스트
- lockstep 네트워크나 rollback 시스템으로 확장 가능한 시뮬레이션 경계

## 이번에 만들 결과물

5개 고정 tick의 이동·발사 입력을 기록한 뒤 같은 seed로 두 번 재생하고 최종 위치와 점수가 정확히 같은지 출력합니다.

```bash
cargo run -p production_structure --bin deterministic_replay
```

예상되는 핵심 출력:

```text
recorded frames: 5
deterministic match: true
```

## 핵심 개념

결정론은 “게임이 언제나 똑같이 보인다”가 아니라, 같은 초기 상태와 같은 tick별 입력으로 시뮬레이션한 상태가 같다는 뜻입니다.

```text
초기 State + RNG seed + InputFrame[0..N]
                         ↓ FixedUpdate
                    최종 State
```

키보드가 눌린 운영체제 시각이나 가변 `delta_secs()`를 기록하면 PC 성능에 따라 tick 배치가 달라집니다. `Update`에서 입력을 수집하더라도 기록에는 “몇 번째 FixedUpdate에서 사용할 명령인지”를 함께 저장합니다.

### 결정론을 깨뜨리는 흔한 원인

| 원인 | 대응 |
|---|---|
| 시스템 시각·가변 delta | Fixed tick과 tick 번호 사용 |
| thread RNG 또는 매번 다른 seed | 세션 seed 저장, 시뮬레이션 전용 RNG Resource 사용 |
| 순서 없는 Query 결과에 의존 | 안정적인 게임 ID로 정렬하거나 순서 독립 계산 |
| 병렬 System의 쓰기 순서 | 명시적 SystemSet·Message 집계 경계 사용 |
| 플랫폼별 부동소수점 오차 | 허용 오차·양자화 또는 중요한 도메인의 정수 fixed-point 검토 |
| Entity ID를 저장 데이터로 사용 | 실행마다 재할당되지 않는 별도 식별자 사용 |

예제는 차이를 눈에 잘 보이게 하려고 위치를 밀리미터 정수로 저장합니다. 모든 게임을 정수로 바꾸라는 뜻은 아닙니다. 물리 엔진까지 서로 다른 플랫폼에서 bit 단위로 같게 만드는 것은 훨씬 어려우므로 프로젝트가 필요한 재현 수준부터 정해야 합니다.

## 샘플 코드

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InputFrame {
    tick: u32,
    move_x: i8,
    fire: bool,
}

app.add_systems(
    FixedUpdate,
    (read_recorded_input, simulate_tick).chain(),
);
```

전체 코드는 [44a_deterministic_replay.rs](source/part7.md#44a--결정론적-입력-리플레이)에서 확인할 수 있습니다.

## 코드 설명

- `ReplayInput`은 기록과 현재 cursor를 보관합니다.
- `CurrentInput`은 입력 재생과 시뮬레이션 사이의 tick별 계약입니다.
- 입력 frame의 tick이 Simulation tick과 다르면 즉시 실패해 누락과 중복을 숨기지 않습니다.
- 시뮬레이션 전용 LCG RNG는 초기 seed가 같으면 같은 숫자 순서를 만듭니다.
- 테스트는 같은 입력·seed의 최종 `SimulationState`를 전체 비교합니다.
- 실제 리플레이 파일에는 게임 버전, 맵 ID, tick rate, seed와 입력 스키마 버전도 함께 저장해야 합니다.

## 실습 과제

1. seed를 기록 구조에 포함하고 다른 seed에서 점수가 달라지는 테스트를 작성하세요.
2. tick 하나를 중복하거나 제거해 검증 오류를 확인하세요.
3. 최종 State뿐 아니라 매 tick의 간단한 checksum을 기록해 처음 달라진 tick을 찾으세요.

## 심화 과제

실제 키보드 입력을 `Update`에서 버퍼링하고 FixedUpdate가 소비한 명령만 리플레이 파일에 기록하세요. 재생 모드에서는 운영체제 입력을 완전히 차단하고 기록만 공급하며, 게임 버전이 다르면 실행을 거부하는 헤더를 추가하세요.

[선택한 과제 해설과 수행 예시 보기](exercises/part7/44a_deterministic_replay.md)

## 다음 챕터

프로파일링 진단과 변경 감지를 적용하고, 추측이 아닌 측정 기반 최적화와 조건부 풀링을 정리합니다.
