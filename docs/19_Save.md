# 19. 최고 점수 저장

## 학습 목표

- 런타임 데이터와 영구 저장 데이터를 구분할 수 있다.
- 파일 읽기·쓰기 오류를 안전하게 처리할 수 있다.
- 저장 시점과 경로 정책을 설계할 수 있다.

## 이번에 만들 결과물

최고 점수를 `save/high_score.txt`에 기록하고 다음 실행에서 다시 불러옵니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 19_save
```

## 핵심 개념

Score는 현재 플레이 세션의 값이고 HighScore는 실행 사이에도 유지할 값입니다. 예제는 이해하기 쉬운 숫자 텍스트 파일을 사용합니다.

파일 시스템은 권한, 디스크, 손상된 내용 등으로 언제든 실패할 수 있습니다. 저장 실패 때문에 게임 전체가 panic하지 않도록 오류를 로그로 남기고 계속 실행합니다.

## 샘플 코드

```rust
fn load_high_score(mut high_score: ResMut<HighScore>) {
    if let Ok(contents) = fs::read_to_string("save/high_score.txt")
        && let Ok(value) = contents.trim().parse()
    {
        high_score.0 = value;
    }
}

fn save_high_score(value: u32) {
    if let Err(error) = fs::create_dir_all("save") {
        warn!("저장 폴더 생성 실패: {error}");
        return;
    }
    if let Err(error) = fs::write("save/high_score.txt", value.to_string()) {
        warn!("최고 점수 저장 실패: {error}");
    }
}
```

## 코드 설명

- 시작 시 파일이 없으면 기본값 0을 그대로 사용합니다.
- `trim().parse()`로 줄바꿈을 제거하고 u32로 변환합니다.
- 최고 점수가 바뀔 때만 파일을 씁니다.
- `create_dir_all`은 폴더가 이미 있어도 성공합니다.
- 완성 코드는 `PathBuf`를 사용해 경로 구성을 한 함수에 모읍니다.

현재 상대 경로는 실행한 작업 디렉터리를 기준으로 합니다. 실제 제품은 운영체제별 사용자 데이터 디렉터리를 사용하고, 중요한 데이터에는 임시 파일 작성 후 rename하는 원자적 저장 방식을 고려하세요.

## 실습 과제

1. 게임을 실행해 점수를 만든 뒤 파일 내용을 확인하세요.
2. 파일에 잘못된 문자열을 넣어도 게임이 시작되는지 확인하세요.
3. 마지막 플레이 점수도 별도 줄에 저장하세요.

## 심화 과제

설정, 최고 점수, 통계를 하나의 버전 필드가 있는 저장 구조로 설계하고 RON 또는 JSON으로 직렬화하세요. 임시 파일과 rename을 사용해 저장 중 종료에도 기존 파일을 보호하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part2/19_save.md)

## 다음 챕터

[19A. 게임 상태 저장과 불러오기](19A_SaveGameRoundTrip.md)에서 위치·체력·점수·진행 상태 전체를 버전 있는 SaveGame으로 확장합니다.
