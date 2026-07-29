# 16. 충돌과 점수

## 학습 목표

- 축 정렬 사각형 AABB 충돌을 구현할 수 있다.
- 충돌 후 Entity 제거를 Commands로 예약할 수 있다.
- 적 처치 결과를 Message로 다른 System에 전달할 수 있다.

## 이번에 만들 결과물

총알이 적에 맞으면 둘 다 제거되고 100점을 얻습니다. 적이 플레이어에 닿으면 체력이 감소합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 16_collision
```

## 핵심 개념

회전하지 않는 사각형 두 개는 X축과 Y축 모두에서 중심 거리보다 반 크기 합이 클 때 겹칩니다.

```text
abs(a.x - b.x) < (a.width + b.width) / 2
abs(a.y - b.y) < (a.height + b.height) / 2
```

렌더링 Sprite 크기와 충돌 크기를 분리하기 위해 `HitBox(Vec2)` Component를 사용합니다.

## 샘플 코드

```rust
fn overlaps(a_position: Vec2, a_size: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let distance = (a_position - b_position).abs();
    distance.x < (a_size.x + b_size.x) / 2.0
        && distance.y < (a_size.y + b_size.y) / 2.0
}

#[derive(Message)]
struct EnemyDefeated {
    points: u32,
}
```

충돌 System은 Bullet Query와 Enemy Query를 비교하고 겹치면 두 Entity를 despawn한 뒤 `EnemyDefeated { points: 100 }`를 씁니다.

## 코드 설명

- AABB는 계산이 단순해 작은 2D 게임과 넓은 단계 충돌 검사에 적합합니다.
- `HitBox`는 Sprite보다 작게 만들어 판정을 관대하게 조정할 수 있습니다.
- 점수 System은 충돌 구현을 몰라도 EnemyDefeated Message만 읽으면 됩니다.
- 체력 감소에는 `saturating_sub`를 사용해 0 아래 정수 오버플로를 막습니다.
- `overlaps`는 순수 함수이므로 창 없이 빠르게 단위 테스트할 수 있습니다.

현재 예제는 규모가 작아 모든 총알과 적 쌍을 비교합니다. 대상이 많아지면 공간 격자나 물리 엔진으로 후보 쌍을 줄여야 합니다.

## 실습 과제

1. 적 처치 점수를 250으로 바꾸세요.
2. 플레이어 HitBox를 Sprite보다 작게 조정하세요.
3. 총알이 적 두 개와 동시에 겹치는 상황에서 점수가 중복되는지 실험하세요.

## 심화 과제

한 프레임에 같은 Entity를 여러 번 처리하지 않도록 `HashSet<Entity>`로 제거 예약 대상을 추적하거나 충돌 단계를 재설계하세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part2/16_collision.md)를 확인하세요.

## 다음 챕터

현재 점수, 체력, 최고 점수와 조작법을 화면 UI로 표시합니다.
