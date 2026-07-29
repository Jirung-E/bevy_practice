# 02. Cargo 과제 해설

[본문으로 돌아가기](../../02_Cargo.md#실습-과제)

## P0-C02-P1 · 임시 패키지 생성

저장소 워크스페이스 바깥의 임시 폴더에서 실행하세요.

```bash
cargo new cargo_practice
cd cargo_practice
```

`Cargo.toml`과 `src/main.rs`가 생성되면 성공입니다. 교재 저장소 안에 만들면 workspace 구성에 영향을 줄 수 있습니다.

## P0-C02-P2 · 출력 변경과 실행

`src/main.rs`의 문자열을 바꾼 뒤 `cargo run`의 마지막 출력이 수정한 문자열과 같은지 확인합니다. 컴파일 성공만 보고 끝내지 말고 실행 결과까지 확인하세요.

## P0-C02-P3 · 워크스페이스 검사

교재 루트에서 다음 명령이 종료 코드 0으로 끝나야 합니다.

```bash
cargo check --workspace
```

특정 패키지만 검사하려면 `cargo check -p hello_bevy`를 사용합니다.

## P0-C02-A1 · metadata 읽기

```bash
cargo metadata --no-deps --format-version 1
```

출력 JSON의 `packages` 배열에서 `name == "hello_bevy"`인 항목을 찾습니다.

### 확인 기준

- `manifest_path`가 `examples/part0/hello_bevy/Cargo.toml`을 가리킨다.
- `edition`이 루트 workspace 설정과 일치한다.
- `workspace_members`에 해당 패키지 ID가 포함된다.

경로는 운영체제에 따라 구분자와 절대 경로 앞부분이 다르므로 문자열 전체를 외우는 과제가 아닙니다.

