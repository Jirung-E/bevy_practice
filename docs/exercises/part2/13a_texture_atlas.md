# 13A. TextureAtlas 과제 해설

[본문으로 돌아가기](../../13A_TextureAtlas.md#실습-과제)

## P2-C13A-P1 · 애니메이션 간격 비교

`0.08`초는 초당 약 12.5프레임, `0.25`초는 초당 4프레임입니다.

### 확인 기준

- 같은 Idle 또는 Walk 구간에서 비교한다.
- 이동 속도는 바꾸지 않는다.
- 애니메이션 속도와 Entity 이동 속도가 서로 독립적임을 설명한다.

프레임 수가 정해져 있어도 Timer 간격이 달라지면 한 주기를 도는 시간이 달라집니다.

## P2-C13A-P2 · 원본 픽셀과 화면 크기

`FRAME_SIZE`는 atlas에서 한 프레임을 자르는 원본 픽셀 크기이고, `Sprite.custom_size`는 월드에서 보이는 크기입니다. `custom_size`만 바꾸면 어떤 프레임을 읽는지는 그대로이고 화면 표시 크기만 달라집니다.

충돌 크기까지 자동으로 바뀌지는 않으므로 시각 크기와 판정 크기를 따로 점검하세요.

## P2-C13A-P3 · 마지막 수평 방향 유지

### 접근 방법

마지막 방향을 별도 데이터로 저장하고 수평 입력이 있을 때만 갱신합니다.

```rust
if direction.x < 0.0 {
    facing.0 = Facing::Left;
} else if direction.x > 0.0 {
    facing.0 = Facing::Right;
}
sprite.flip_x = facing.0 == Facing::Left;
```

위아래 이동이나 정지 상태에서는 기존 값을 유지합니다. `direction.x == 0.0`일 때 무조건 오른쪽으로 바꾸면 과제 조건을 만족하지 못합니다.

## P2-C13A-A1 · AnimationClip2d

```rust
#[derive(Clone, Copy)]
struct AnimationClip2d {
    start: usize,
    end: usize,
    fps: f32,
    repeat: bool,
}
```

Idle과 Walk의 범위는 데이터 상수에 한 번만 정의합니다. 전환 코드는 숫자 인덱스가 아니라 선택된 clip을 사용합니다.

```rust
let next_clip = if moving { WALK_CLIP } else { IDLE_CLIP };
animation.set_clip(next_clip);
```

### 확인 기준

- 전환 System에 `0`, `3`, `4`, `7`이 직접 등장하지 않는다.
- FPS에서 Timer 간격을 계산한다.
- 반복 clip은 끝에서 시작으로 돌아간다.
- 반복하지 않는 clip은 마지막 프레임에 머문다.
- clip 전환 시 현재 인덱스가 새 범위 밖이면 시작 프레임으로 보정된다.

숫자를 숨기는 것만으로는 충분하지 않습니다. 범위, 속도, 반복 정책을 하나의 데이터 단위로 다루어야 다른 캐릭터나 공격 애니메이션을 추가하기 쉽습니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin texture_atlas_solution
cargo test -p space_survivor --bin texture_atlas_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/texture_atlas_solution.rs`

