# 09. Resource: 전역 데이터 관리하기

## 학습 목표

- Resource와 Component의 사용 기준을 구분할 수 있다.
- Resource를 삽입하고 `Res`, `ResMut`로 접근할 수 있다.
- 전역 데이터를 하나의 거대한 구조체로 만들 때의 문제를 이해한다.

## 이 내용으로 만들 수 있는 것

- 점수, 난이도, 게임 설정처럼 월드 전체가 공유하는 값을 관리할 수 있습니다.
- 타이머와 전역 서비스를 Resource로 두어 여러 System이 같은 상태를 사용할 수 있습니다.

## 이번에 만들 결과물

적 처치 점수 규칙과 현재 점수를 Resource로 등록하고, 두 프레임 동안 점수를 누적합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p ecs_basics --bin resource
```

## 핵심 개념

Resource는 World에 타입별로 하나만 존재하는 데이터입니다. 현재 점수, 설정, 에셋 모음처럼 게임 전체가 공유하는 단일 값에 적합합니다.

여러 개 존재하며 개별적으로 검색해야 하는 데이터는 Component가 더 적합합니다. 모든 전역 값을 하나의 `GameData` Resource에 넣으면 사소한 필드 하나를 수정하는 System끼리도 쓰기 충돌이 생깁니다. 변경 이유와 접근 패턴이 다른 데이터는 별도 Resource로 나누세요.

## 샘플 코드

```rust
#[derive(Resource, Debug)]
struct GameScore(u32);

#[derive(Resource)]
struct ScorePerEnemy(u32);

fn add_score(mut score: ResMut<GameScore>, rule: Res<ScorePerEnemy>) {
    score.0 += rule.0;
}

fn print_score(score: Res<GameScore>) {
    println!("현재 점수: {}", score.0);
}

fn main() {
    let mut app = App::new();
    app.insert_resource(GameScore(0))
        .insert_resource(ScorePerEnemy(100))
        .add_systems(Update, (add_score, print_score).chain());

    app.update();
    app.update();
}
```

## 코드 설명

- `#[derive(Resource)]`는 타입을 Resource로 사용할 수 있게 합니다.
- `insert_resource`는 초기값을 직접 넣으며 같은 타입이 있으면 교체합니다.
- `Res<T>`는 공유 읽기 접근, `ResMut<T>`는 독점 쓰기 접근입니다.
- `Res`와 `ResMut`는 스마트 포인터처럼 필드에 접근합니다.
- 두 프레임 실행 후 점수는 100, 200 순서로 출력됩니다.

기본값이 자연스러운 Resource는 `#[derive(Resource, Default)]`와 `init_resource::<T>()`를 사용할 수 있습니다.

## 실습 과제

1. 적 처치 점수를 250으로 바꾸세요.
2. `HighScore(u32)` Resource를 추가하세요.
3. 현재 점수가 최고 점수를 넘으면 갱신하는 System을 작성하세요.

## 심화 과제

`Local<u32>`를 사용하는 System을 하나 만드세요. Resource와 Local이 각각 어디에 저장되고 몇 개 존재하는지 비교하고, 프레임 카운터에는 어느 쪽이 적합한지 상황별로 설명해 보세요.

과제를 먼저 직접 수행한 뒤 필요할 때 [힌트와 수행 예시](exercises/part1/09_resource.md)를 확인하세요.

## 다음 챕터

Commands로 Entity 생성·수정·제거를 예약하고, System 실행 중 World 구조를 안전하게 변경합니다.
