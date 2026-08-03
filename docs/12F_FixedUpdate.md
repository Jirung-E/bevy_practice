# 12F. FixedUpdate와 입력 버퍼

## 학습 목표

- `Update`와 `FixedUpdate`의 실행 목적을 구분할 수 있다.
- 렌더 프레임 사이의 짧은 입력을 버퍼링할 수 있다.
- 고정 timestep을 사용하는 시뮬레이션과 화면 표시를 분리할 수 있다.
- `Time<Fixed>`의 timestep으로 프레임률과 독립적인 계산을 할 수 있다.

## 이 내용으로 만들 수 있는 것

- 프레임률이 달라도 같은 속도로 진행되는 전투·물리 시뮬레이션
- FixedUpdate 사이에서 짧은 버튼 입력을 놓치지 않는 입력 처리
- 입력 기록, 리플레이, 네트워크 예측으로 확장 가능한 명령 버퍼

## 이번에 만들 결과물

Update에서 한 번 발생한 Attack을 `InputBuffer`에 저장하고 두 번의 FixedUpdate에서 정확히 한 번만 소비합니다. 60Hz에서 속도 6인 대상은 두 tick 뒤 0.2만큼 이동합니다.

```bash
cargo run -p ecs_basics --bin fixed_update
```

## 핵심 개념

`Update`는 화면 프레임마다 실행되며 횟수와 간격이 달라질 수 있습니다. `FixedUpdate`는 누적된 시간에 따라 고정 간격으로 0회, 1회 또는 여러 번 실행됩니다.

```text
Update:       입력 읽기, UI, 카메라, 화면 보간
FixedUpdate:  물리, 전투 판정, 결정적인 시뮬레이션 tick
```

`just_pressed`를 FixedUpdate에서 직접 읽으면 두 fixed tick 사이에 눌렀다 놓은 입력을 놓칠 수 있습니다. Update에서 `PlayerCommand`를 queue에 넣고 FixedUpdate가 소비해야 합니다.

```text
Update → InputBuffer.push_back(Attack)
FixedUpdate → pop_front() → Attack 1회 실행
다음 FixedUpdate → queue가 비어 있으므로 재실행하지 않음
```

모든 움직임을 반드시 FixedUpdate로 옮길 필요는 없습니다. 화면 연출과 카메라는 Update가 자연스럽고, 충돌·전투처럼 같은 조건에서 같은 결과가 중요하거나 물리 schedule과 맞춰야 하는 로직은 FixedUpdate가 적합합니다.

## 샘플 코드

전체 코드: `examples/part1/ecs_basics/src/bin/12f_fixed_update.rs`

```rust
app.insert_resource(Time::<Fixed>::from_hz(60.0))
    .add_systems(Update, buffer_input)
    .add_systems(FixedUpdate, simulate);
```

```rust
simulation.position += velocity.0 * fixed_time.timestep().as_secs_f32();
while let Some(command) = buffer.0.pop_front() {
    match command {
        PlayerCommand::Attack => simulation.attacks += 1,
    }
}
```

## 코드 설명

- `VecDeque`는 입력 순서를 유지하는 queue입니다.
- Update는 edge 입력을 기록한 뒤 원본 flag를 지웁니다.
- FixedUpdate는 고정 timestep으로 위치를 계산합니다.
- queue를 비우므로 같은 Attack이 다음 tick에 반복되지 않습니다.
- 테스트는 Update 1회와 FixedUpdate 2회를 직접 실행해 tick 수와 공격 횟수를 검증합니다.

## 실습 과제

1. 고정 주파수를 30Hz로 바꾸고 같은 실제 시간의 이동 거리가 같도록 tick 수를 조정하세요.
2. Attack 두 개를 queue에 넣어 순서대로 두 번 소비되는지 확인하세요.
3. 유지형 `Move(Vec2)`와 일회성 `Attack`의 버퍼 정책을 다르게 설계하세요.

## 심화 과제

FixedUpdate가 만든 이전·현재 위치 snapshot을 보관하고 Update에서 interpolation alpha로 화면 Transform을 보간하세요. 시뮬레이션 위치와 표시 위치가 서로 다른 Component인지 테스트하세요.

[힌트와 수행 예시](exercises/part1/12f_fixed_update.md)

## 다음 챕터

순수 함수, World, headless App을 사용해 ECS 규칙을 자동으로 검증합니다.
