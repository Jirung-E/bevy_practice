# 11. Messages와 Events 과제 해설

[본문으로 돌아가기](../../11_MessagesAndEvents.md#실습-과제)

## P1-C11-P1 · 한 프레임에 두 Message

Writer에서 두 값을 차례로 쓰면 Reader는 자신의 커서 이후에 있는 두 Message를 모두 읽습니다. 한 프레임에 하나만 온다고 가정해 `single`처럼 처리하지 말고 반복해야 합니다.

## P1-C11-P2 · bonus 필드

```rust
#[derive(Message)]
struct EnemyDefeated {
    score: u32,
    bonus: u32,
}
```

점수 Reader는 `score + bonus`를 누적합니다. bonus가 점수 규칙에 속한다면 Message에 최종 점수만 담는 대안도 있습니다. 어느 System이 규칙을 소유할지에 따라 선택하세요.

## P1-C11-P3 · 독립적인 사운드 Reader

각 `MessageReader`는 독립적인 읽기 위치를 가지므로 점수 System이 먼저 읽어도 사운드 System이 같은 두 Message를 모두 받을 수 있습니다.

### 확인 기준

- 점수와 사운드 Reader가 각각 두 건을 처리한다.
- 한 Reader가 다른 Reader를 직접 호출하지 않는다.
- 처리 횟수는 별도 Resource나 로그로 검증할 수 있다.

## P1-C11-A1 · Observer 즉시 반응

```rust
#[derive(Event)]
struct EnemyDefeatedNow {
    score: u32,
}

fn trigger_now(mut commands: Commands) {
    commands.trigger(EnemyDefeatedNow { score: 100 });
}

fn observe_now(event: On<EnemyDefeatedNow>) {
    println!("즉시 처리: {}", event.score);
}
```

App에는 `.add_observer(observe_now)`를 등록합니다.

| 기준 | Message | Event + Observer |
|---|---|---|
| 처리 방식 | 버퍼를 Reader가 순회 | trigger 시 Observer 실행 |
| 여러 소비자 | Reader별 독립 커서 | Observer별 반응 |
| 시간 결합 | 한두 프레임 지연 허용 | 즉시 반응 |
| 대상 지정·전파 | 일반적으로 전역 버퍼 | EntityEvent와 전파 제어 가능 |
| 장기 보관 | 부적합 | 부적합 |

점수·사운드·UI처럼 느슨하게 결합된 여러 소비자에는 Message가 단순합니다. 즉시 취소하거나 Entity 관계를 따라 전파해야 하는 상호작용에는 Observer가 적합합니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin messages_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/messages_solution.rs`

