# 12G. ECS 테스트 전략

## 학습 목표

- 순수 함수, `World`, headless `App` 테스트의 역할을 구분할 수 있다.
- 렌더링 없이 실제 System과 Resource를 실행할 수 있다.
- Component 조합과 schedule 결과를 자동 검증할 수 있다.
- 시간·입력·랜덤 의존성을 테스트 가능한 경계로 분리할 수 있다.

## 이 내용으로 만들 수 있는 것

- 창을 열지 않고 실행되는 빠른 gameplay 회귀 테스트
- Component 누락, System 순서, State 전환 오류를 잡는 테스트
- 고정 입력과 seed로 재현할 수 있는 전투 시나리오

## 이번에 만들 결과물

피해 계산은 순수 함수로, Entity 조합은 `World` Query로, 실제 피해 System과 Resource는 headless `App`으로 각각 테스트합니다.

```bash
cargo test -p ecs_basics --bin ecs_testing
cargo run -p ecs_basics --bin ecs_testing
```

## 핵심 개념

가장 작은 경계부터 테스트하면 실패 원인을 찾기 쉽습니다.

| 테스트 경계 | 사용 시점 | 장점 |
|---|---|---|
| 순수 함수 | 숫자 계산·규칙 | Bevy 없이 가장 빠르다 |
| `World` | Component 조합·Query | schedule 없이 ECS 저장 구조를 검사한다 |
| headless `App` | System·Resource·Message·State | 실제 schedule 계약을 검증한다 |
| 그래픽 통합 테스트 | 렌더 pipeline·GPU | 느리고 플랫폼 영향이 있어 별도로 운영한다 |

headless App은 `DefaultPlugins`를 추가하지 않고 필요한 Resource, Message, System만 등록합니다. gameplay Plugin이 창·카메라 없이 등록되지 않는다면 렌더링과 도메인 규칙의 경계가 잘못된 신호일 수 있습니다.

시간 테스트는 실제 sleep에 의존하지 말고 고정 delta나 직접 schedule 실행을 사용합니다. 랜덤 로직은 seed를 입력으로 받고, 입력은 `PlayerCommand`처럼 기록 가능한 데이터로 바꿉니다.

## 샘플 코드

전체 코드: `examples/part1/ecs_basics/src/bin/12g_ecs_testing.rs`

```rust
fn test_app() -> App {
    let mut app = App::new();
    app.init_resource::<Defeated>()
        .add_systems(Update, damage_enemies);
    app
}
```

```rust
#[test]
fn headless_app_runs_real_systems_and_resources() {
    let mut app = test_app();
    app.world_mut().spawn((Enemy, Health(5)));
    app.update();
    assert_eq!(app.world().resource::<Defeated>().0, 1);
}
```

## 코드 설명

- `apply_damage`는 ECS와 무관한 규칙이라 순수 함수로 테스트합니다.
- `World::query_filtered`는 Enemy marker가 있는 Health만 선택하는지 검사합니다.
- headless App은 실제 `damage_enemies` System을 Update schedule에서 실행합니다.
- 결과를 로그 문자열이 아니라 Component와 Resource 값으로 검증합니다.
- 한 테스트가 다른 테스트의 World 상태를 공유하지 않도록 매번 App을 새로 만듭니다.

## 실습 과제

1. Health가 10보다 큰 적은 처치 수에 포함되지 않는 경계값 테스트를 추가하세요.
2. Enemy marker가 없는 Health가 피해를 받지 않는 테스트를 추가하세요.
3. 같은 App을 두 번 업데이트했을 때 이미 체력 0인 적을 중복 집계하지 않도록 규칙을 수정하고 테스트하세요.

## 심화 과제

입력 명령 목록을 받아 여러 FixedUpdate tick을 진행하는 시나리오 runner를 만들고, 마지막 World 상태를 snapshot 구조체로 변환해 비교하세요. 실패 시 tick 번호와 입력 명령을 출력해 재현 가능하게 만드세요.

[힌트와 수행 예시](exercises/part1/12g_ecs_testing.md)

## 다음 챕터

Part 2에서 입력 명령, 고정 시간 처리, 자동 테스트를 실제 2D 게임 기능에 적용합니다.
