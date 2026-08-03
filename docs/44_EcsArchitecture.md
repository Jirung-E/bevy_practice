# 44. ECS 아키텍처

## 학습 목표

- 데이터 흐름을 Component, Resource, Message로 구분할 수 있다.
- SystemSet으로 의미 있는 실행 순서를 선언할 수 있다.
- 기능 사이 직접 호출을 ECS 계약으로 바꿀 수 있다.

## 이 내용으로 만들 수 있는 것

- 입력·시뮬레이션·표시 순서가 명확한 게임 루프
- gameplay와 presentation이 Message로 통신하는 구조
- 기능을 추가해도 System 의존성이 뒤엉키지 않는 ECS 설계

## 이번에 만들 결과물

Production Arena의 플레이 가능한 단계입니다. WASD로 이동하고 Space를 누르면 반경 4 안의 가장 가까운 적을 제거해 100점을 얻습니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p production_structure --bin 44_ecs_architecture
```

## 핵심 개념

이 예제의 한 프레임은 다음 단계로 흐릅니다.

```text
Input → Simulation → Feedback
키보드 → Velocity → Transform → EnemyDefeated → Score → HUD
```

System 등록 순서에 우연히 의존하지 않고 GameSet을 chain해 의도를 선언합니다. 적 처치와 점수 계산은 함수 호출이 아니라 EnemyDefeated Message로 연결됩니다.

## 샘플 코드

```rust
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameSet {
    Input,
    Simulation,
    Feedback,
}

app.configure_sets(
    Update,
    (GameSet::Input, GameSet::Simulation, GameSet::Feedback).chain(),
);
```

```rust
fn apply_score(
    mut messages: MessageReader<EnemyDefeated>,
    mut score: ResMut<Score>,
) {
    score.0 += messages
        .read()
        .map(|message| message.points)
        .sum::<u32>();
}
```

## 코드 설명

- Velocity Component는 입력과 이동 사이의 데이터 계약입니다.
- Score Resource는 앱 전체에 하나뿐인 누적 상태입니다.
- EnemyDefeated Message에는 소비자가 필요한 최소 결과만 담습니다.
- Presentation은 충돌이나 키보드를 알지 않고 Score만 읽습니다.
- wrap_axis 같은 순수 도메인 함수는 Bevy App 없이 테스트합니다.

SystemSet을 지나치게 세분화하면 모든 System이 직렬화됩니다. 실제 데이터 접근만으로 병렬 실행 가능한 부분은 순서를 강제하지 않습니다.

## 실습 과제

1. 적 제거 소리를 처리하는 두 번째 MessageReader를 추가하세요.
2. 공격 쿨다운 Resource를 추가하세요.
3. Feedback 안에서 점수와 HUD의 세부 순서를 명시하세요.

## 심화 과제

고정 시간 Simulation과 가변 시간 Presentation을 분리할 SystemSet을 설계하고, 어떤 데이터가 두 Schedule 사이 계약이 될지 표로 정리하세요. 실제 입력 기록과 재생 구현은 다음 챕터에서 진행합니다.

[선택형 과제 해설과 수행 예시 보기](exercises/part7/44_ecs_architecture.md)

## 다음 챕터

같은 초기 상태와 입력이 같은 결과를 만드는 결정론적 FixedUpdate와 입력 리플레이를 구현합니다.
