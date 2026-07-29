# 04. 개발 환경 점검하기

## 학습 목표

- Rust와 Cargo가 정상적으로 설치되었는지 확인할 수 있다.
- `Cargo.toml`에 사용 중인 Bevy 버전이 어디에 기록되는지 찾을 수 있다.
- 선택적으로 빠른 링커를 설정할 수 있다.
- 운영체제 의존성을 추가로 확인해야 하는 상황을 구분할 수 있다.

## 이번에 만들 결과물

03장의 창 예제를 다시 실행해 현재 개발 환경에서 Rust와 Bevy가 정상적으로 동작하는지 확인합니다. 아래 명령은 이 교재 저장소에 포함된 `hello_bevy` 샘플을 검사하고 실행합니다. 본문만 따라 만든 별도 프로젝트라면 `-p hello_bevy`를 빼고 `cargo check`, `cargo run`을 사용하세요.

```bash
rustc --version
cargo --version
cargo check -p hello_bevy
cargo run -p hello_bevy
```

03장의 창이 정상적으로 열렸다면 이 장의 필수 점검은 끝난 것입니다. 편집기 교체나 운영체제 패키지 재설치를 일부러 할 필요는 없습니다.

## 핵심 개념

### Rust 설치와 기본 점검

Rust를 아직 설치하지 않았다면 [Rust 공식 설치 페이지](https://www.rust-lang.org/tools/install)의 안내에 따라 `rustup`으로 설치하세요. 이미 03장 예제를 실행했다면 Rust, Cargo, 기본 컴파일러 도구는 준비된 상태입니다.

```bash
rustup update stable
rustup default stable
rustc --version
cargo --version
```

네 명령이 모두 실행되고 `rustc`와 `cargo`의 버전이 출력되면 됩니다. 이 교재 저장소가 요구하는 최소 Rust 버전은 루트 `Cargo.toml`의 `rust-version`에서 확인할 수 있습니다.

### Cargo.toml에 Bevy 추가하기

Bevy는 별도 프로그램을 설치하는 방식이 아니라 Rust 프로젝트의 의존성으로 추가합니다. 새 프로젝트에서는 다음 명령을 사용할 수 있습니다.

```bash
cargo add bevy@0.19
```

그러면 `Cargo.toml`에 다음 항목이 생깁니다.

```toml
[dependencies]
bevy = "0.19"
```

이 교재는 Bevy 0.19를 기준으로 합니다. 저장소에서는 여러 실습 패키지가 같은 버전을 쓰도록 루트 `Cargo.toml`의 `[workspace.dependencies]`에서 Bevy 버전을 한 번만 선언합니다.

### 첫 빌드와 이후 빌드

첫 `cargo check` 또는 `cargo run`은 Bevy와 많은 의존성을 내려받아 처음부터 컴파일하므로 오래 걸릴 수 있습니다. 정상적인 현상이며, 이후에는 바뀐 부분만 다시 빌드하므로 대체로 빨라집니다.

### 빠른 링커는 선택 사항

기본 링커로도 모든 실습을 진행할 수 있습니다. 다만 링크 단계가 오래 걸린다면 [Bevy 공식 Setup 문서](https://bevy.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional)의 운영체제별 대체 링커 설정을 적용할 수 있습니다.

Windows의 Bevy 0.19 공식 예시는 다음과 같습니다.

```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

프로젝트 루트의 `.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
```

Linux와 macOS의 권장 설정은 운영체제와 배포판에 따라 다르므로 공식 Setup 문서에서 해당 항목만 확인하세요. 빠른 링커와 `bevy/dynamic_linking`은 개발 시간을 줄이는 선택 사항이며, 설정하지 않아도 예제 결과는 달라지지 않습니다.

### 참고: 운영체제 의존성이 필요한 경우

정상적인 Rust 설치 뒤 03장의 창이 열렸다면 현재 환경에 필요한 의존성은 이미 갖춰졌다고 보면 됩니다. 아래 항목은 “지금 모두 설치해야 하는 준비물 목록”이 아니라, 빌드나 실행이 실패했을 때 확인할 참고 사항입니다.

- **Windows**: `link.exe` 또는 Windows SDK를 찾지 못한다는 오류가 날 때 Visual Studio Build Tools의 `Desktop development with C++` 워크로드를 확인합니다.
- **macOS**: 링커나 SDK를 찾지 못할 때 `xcode-select --install`로 Command Line Tools를 확인합니다.
- **Linux**: 창, 오디오 또는 입력 관련 시스템 라이브러리를 찾지 못할 때 [Bevy 공식 Linux 의존성 안내](https://bevy.org/learn/quick-start/getting-started/setup/#installing-os-dependencies)를 따라 현재 배포판에 필요한 패키지만 설치합니다.

오류가 없는데도 다른 운영체제용 도구나 패키지를 미리 설치할 필요는 없습니다.

## 샘플 코드

이 장에서 확인할 실제 프로젝트 설정은 루트 `Cargo.toml`의 다음 두 항목입니다.

```toml
[workspace.package]
rust-version = "1.95"

[workspace.dependencies]
bevy = "0.19"
```

환경 확인에는 소스 코드를 새로 작성하지 않고, 03장의 `hello_bevy` 샘플을 그대로 사용합니다.

## 코드 설명

- `rust-version`은 이 워크스페이스가 요구하는 최소 Rust 버전입니다.
- `bevy = "0.19"`는 모든 실습 패키지가 사용할 Bevy 버전입니다.
- `cargo check`는 실행 파일을 끝까지 링크하지 않고 타입과 컴파일 오류를 빠르게 검사합니다.
- `cargo run`은 빌드와 링크를 마친 뒤 실제 창을 실행합니다.
- 빠른 링커 설정은 마지막 링크 단계만 바꾸며 Bevy 코드나 게임 동작을 바꾸지 않습니다.

## 실습 과제

1. `rustc --version`과 `cargo --version`의 출력 결과를 기록하세요.
2. 루트 `Cargo.toml`에서 `rust-version`과 Bevy 버전을 찾아 기록하세요.
3. `cargo check -p hello_bevy`와 `cargo run -p hello_bevy`를 실행하고, 검사 성공과 창 실행 성공의 차이를 확인하세요.

## 심화 과제

현재 링크 시간이 불편할 정도로 길 때만 공식 Setup 문서의 빠른 링커를 설정하세요. 같은 소스를 한 번 수정한 뒤 설정 전후의 두 번째 빌드 시간을 비교하고, 차이가 작다면 기본 설정으로 되돌려도 됩니다.

과제를 먼저 수행한 뒤 필요할 때 [검증 절차와 기록 예시](exercises/part0/04_development_environment.md)를 확인하세요.

## 다음 챕터

준비를 마쳤습니다. 다음 Part에서는 화면에 나타나는 대상을 구성하는 첫 단위인 Entity부터 Bevy ECS를 배웁니다.
