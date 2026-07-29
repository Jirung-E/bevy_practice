# 09. Resource 과제 해설

[본문으로 돌아가기](../../09_Resource.md#실습-과제)

## P1-C09-P1 · 처치 점수 변경

`ScorePerEnemy(250)`로 초기값을 바꾸면 첫 두 프레임의 점수는 250, 500입니다. 실행 전에 두 값을 예상한 뒤 출력과 비교하세요.

## P1-C09-P2 · HighScore Resource

### 힌트

현재 점수와 최고 점수는 역할과 저장 시점이 다르므로 별도 Resource로 정의합니다.

```rust
#[derive(Resource, Debug, Default)]
struct HighScore(u32);
```

## P1-C09-P3 · 최고 점수 갱신

### 접근 방법

점수 추가 뒤 최고 점수를 비교해야 하므로 System 순서를 명시합니다.

```rust
fn update_high_score(score: Res<GameScore>, mut high_score: ResMut<HighScore>) {
    high_score.0 = high_score.0.max(score.0);
}
```

### 확인 기준

- 현재 점수가 더 클 때만 최고 점수가 바뀐다.
- `(add_score, update_high_score, print_score).chain()`으로 순서가 드러난다.
- 현재 점수와 최고 점수를 서로 다른 Resource로 조회할 수 있다.

## P1-C09-A1 · Local 프레임 카운터

`Local<T>`는 World에 타입별 하나가 아니라 **해당 System 인스턴스 내부에 하나** 존재합니다. 같은 함수로 System을 두 번 등록하면 각각 독립된 Local 값을 가집니다.

```rust
fn count_frames(mut frame: Local<u32>) {
    *frame += 1;
    println!("이 System이 실행된 횟수: {frame}");
}
```

- 다른 System도 읽어야 하는 전체 프레임 번호: Resource가 적합
- 특정 System의 재시도 횟수나 내부 캐시: Local이 적합
- 저장하거나 외부에서 검사해야 하는 게임 진행 데이터: Resource가 적합

Local은 편리하지만 의존성이 함수 시그니처 안에만 보이므로, 여러 System이 공유해야 할 상태를 억지로 숨기는 용도로 사용하면 안 됩니다.

## 전체 코드 실행

```bash
cargo run -p ecs_basics --bin resource_solution
```

전체 코드: `examples/part1/ecs_basics/src/bin/resource_solution.rs`

