# 02. Cargo로 프로젝트 관리하기

## 학습 목표

- Cargo 패키지를 생성하고 빌드·실행·검사할 수 있다.
- `Cargo.toml`과 `Cargo.lock`의 역할을 구분할 수 있다.
- 이 교재가 Cargo 워크스페이스를 사용하는 이유를 이해한다.

## 이번에 만들 결과물

새 Rust 패키지를 생성하고 자주 사용하는 Cargo 명령을 실행합니다. 이어서 이 저장소의 워크스페이스에서 특정 Bevy 예제만 선택해 실행합니다.

## 핵심 개념

### 패키지

Cargo 패키지는 하나의 `Cargo.toml`과 하나 이상의 빌드 대상(binary 또는 library)으로 구성됩니다. `cargo new hello_bevy`는 실행 프로그램 패키지를 만듭니다.

### 매니페스트와 잠금 파일

- `Cargo.toml`: 패키지 정보와 허용할 의존성 버전 범위를 사람이 작성합니다.
- `Cargo.lock`: 실제로 선택된 모든 의존성 버전을 Cargo가 기록합니다.

애플리케이션 저장소에서는 `Cargo.lock`을 버전 관리에 포함해 같은 의존성을 재현하는 것이 일반적입니다.

### 워크스페이스

워크스페이스는 여러 Cargo 패키지를 한 저장소에서 관리합니다. 이 교재는 챕터별 완성본을 독립 패키지로 보존하면서 빌드 결과와 잠금 파일을 공유합니다.

## 샘플 코드

새 프로젝트를 만드는 기본 명령:

```bash
cargo new hello_bevy
cd hello_bevy
cargo run
```

이 교재의 루트 `Cargo.toml`은 다음과 같은 형태입니다.

```toml
[workspace]
resolver = "3"
members = [
    "examples/part0/hello_bevy",
]

[workspace.package]
edition = "2024"
rust-version = "1.95"

[workspace.dependencies]
bevy = "0.19"
```

특정 예제 실행:

```bash
cargo run -p hello_bevy
```

## 코드 설명

- `resolver = "3"`은 Rust 2024 Edition에 맞는 Cargo 기능 선택 규칙을 사용합니다.
- `members`에는 워크스페이스에 속한 패키지 경로가 들어갑니다.
- `[workspace.package]`와 `[workspace.dependencies]`는 여러 예제가 공유할 기준값입니다.
- 자식 패키지는 `bevy.workspace = true`로 루트의 Bevy 버전을 상속할 수 있습니다.
- `-p hello_bevy`는 패키지 이름으로 실행 대상을 선택합니다.

자주 사용하는 명령:

| 명령 | 용도 |
|---|---|
| `cargo check` | 빠른 컴파일 검사 |
| `cargo build` | 개발용 실행 파일 빌드 |
| `cargo run` | 빌드 후 실행 |
| `cargo test` | 테스트 실행 |
| `cargo fmt --all --check` | 코드 형식 검사 |
| `cargo clippy --workspace` | 흔한 실수와 개선점 검사 |

## 실습 과제

1. 임시 폴더에 `cargo new cargo_practice` 패키지를 만드세요.
2. `println!` 내용을 바꾸고 `cargo run`으로 확인하세요.
3. 이 교재 루트에서 `cargo check --workspace`를 실행하세요.

## 심화 과제

`cargo metadata --no-deps`를 실행해 워크스페이스 패키지 정보를 살펴보세요. 출력에서 `hello_bevy`의 매니페스트 경로와 Edition을 찾아보세요.

과제를 먼저 수행한 뒤 필요할 때 [확인 명령과 결과 읽는 법](exercises/part0/02_cargo.md)을 확인하세요.

## 다음 챕터

다음 챕터에서는 워크스페이스에 포함된 첫 Bevy 프로그램을 읽고 실제 창을 실행합니다.
