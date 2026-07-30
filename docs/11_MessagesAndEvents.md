# 11. Messages와 Events: 시스템 사이 통신

## 학습 목표

- Bevy 0.19의 Message와 Observer Event를 구분할 수 있다.
- Message를 등록하고 쓰고 읽을 수 있다.
- 직접 함수 호출 대신 메시지가 유리한 상황을 판단할 수 있다.

## 이 내용으로 만들 수 있는 것

- 피격, 득점, 파일 열기처럼 발생 주체와 처리 주체가 다른 알림을 전달할 수 있습니다.
- 하나의 사건을 UI, 사운드와 저장 System이 각각 받아 반응하게 만들 수 있습니다.

## 이번에 만들 결과물

적 처치 System이 `EnemyDefeated` Message를 쓰고, 점수 System이 이를 읽어 누적 점수를 갱신합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin messages
```

## 핵심 개념

Bevy 0.19에서는 이전 버전에서 흔히 Event라고 부르던 버퍼 기반 통신이 **Message**라는 이름으로 분리되었습니다.

- **Message**: `MessageWriter`로 쓰고 `MessageReader`가 자신의 읽기 위치를 추적하며 읽는 버퍼 기반 통신
- **Event/Observer**: 특정 Entity 또는 전역 대상을 trigger하면 Observer가 반응하는 즉시 전파 방식

점수, 사운드, UI처럼 여러 System이 같은 사건을 독립적으로 처리하고 한두 프레임의 지연을 허용할 수 있다면 Message가 잘 맞습니다. 즉시 반응과 전파 제어가 필요하면 Observer를 검토합니다.

## 샘플 코드

```rust
#[derive(Message, Debug)]
struct EnemyDefeated {
    score: u32,
}

#[derive(Resource, Default)]
struct Score(u32);

fn defeat_enemy(mut messages: MessageWriter<EnemyDefeated>) {
    messages.write(EnemyDefeated { score: 100 });
}

fn update_score(
    mut messages: MessageReader<EnemyDefeated>,
    mut score: ResMut<Score>,
) {
    for message in messages.read() {
        score.0 += message.score;
        println!("{message:?} 수신, 누적 점수: {}", score.0);
    }
}
```

App에는 `.add_message::<EnemyDefeated>()`를 등록하고 두 System을 `chain()`으로 연결합니다.

## 코드 설명

- `#[derive(Message)]`는 버퍼에 저장할 Message 타입을 정의합니다.
- `add_message`는 Message 저장소와 유지 관리 System을 App에 추가합니다.
- `MessageWriter::write`는 값을 버퍼에 씁니다.
- 각 `MessageReader`는 어디까지 읽었는지 독립적으로 기억합니다.
- 같은 Message를 점수와 사운드 Reader가 각각 한 번씩 처리할 수 있습니다.
- 오래 읽지 않은 Message는 무한히 보관되지 않으므로 영구 기록에는 Resource나 저장 파일을 사용하세요.

구버전 자료의 `EventWriter::send`, `EventReader::read`, `add_event` 코드를 0.19 코드에 그대로 복사하지 않도록 주의하세요.

## 실습 과제

1. 한 프레임에 Message를 두 개 쓰고 점수가 모두 반영되는지 확인하세요.
2. `bonus: u32` 필드를 추가해 점수에 함께 더하세요.
3. 별도 `play_sound` Reader System을 추가해 같은 Message를 독립적으로 읽으세요.

## 심화 과제

`commands.trigger(...)`와 `Observer`를 사용하는 즉시 반응 예제를 공식 문서에서 찾아 같은 적 처치 상황으로 다시 작성하세요. Message 방식과 실행 시점, 결합도, 전파 범위를 비교하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/11_messages.md)를 확인하세요.

## 다음 챕터

Menu, Playing, GameOver 상태를 만들고 상태에 따라 실행할 System을 분리합니다.
