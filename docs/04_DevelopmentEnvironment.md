# 04. 개발 환경 점검하기

## 학습 목표

- 운영체제별 Bevy 필수 도구를 준비할 수 있다.
- rust-analyzer와 자동 서식을 활용할 수 있다.
- 빌드, 린트, 실행 단계에서 발생한 문제를 구분할 수 있다.

## 이번에 만들 결과물

첫 Bevy 예제를 검사하고 실행할 수 있는 개발 환경을 완성합니다. 마지막에는 포맷, 컴파일, 린트 검사를 차례로 통과시킵니다.

## 핵심 개념

### Rust 도구 체인

Bevy는 Rust 안정판을 사용합니다. `rustup update stable`로 업데이트하고 `rustup default stable`로 기본 도구 체인을 선택할 수 있습니다.

### 운영체제 의존성

- **Windows**: Visual Studio Build Tools의 `Desktop development with C++`, 최신 MSVC와 Windows SDK가 필요합니다.
- **macOS**: `xcode-select --install`로 Command Line Tools를 설치합니다.
- **Linux**: 배포판에 맞는 C/C++ 도구, ALSA, udev, X11/Wayland 개발 패키지가 필요합니다. 정확한 목록은 Bevy 공식 Linux Dependencies 문서를 확인합니다.

### 편집기

VS Code와 rust-analyzer 조합을 권장하지만, LSP를 지원하는 편집기라면 사용할 수 있습니다. 저장 시 `rustfmt`를 실행하고, 입력 중 rust-analyzer 진단을 확인하도록 설정하면 피드백 주기가 짧아집니다.

### 오류를 읽는 순서

1. 터미널의 첫 번째 `error`를 찾습니다.
2. `--> 파일:줄:열` 위치를 엽니다.
3. 오류 설명과 `help` 제안을 읽습니다.
4. 하나를 수정한 뒤 다시 `cargo check`를 실행합니다.

뒤의 오류는 첫 오류 때문에 연쇄적으로 생길 수 있으므로 처음부터 모두 고치려고 하지 않습니다.

## 샘플 코드

교재 루트에서 다음 검사를 순서대로 실행합니다.

```bash
rustc --version
cargo --version
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets
cargo run -p hello_bevy
```

선택적으로 컴파일 시간을 줄이고 싶다면 Bevy 공식 Setup 문서의 동적 링크와 대체 링커 설정을 참고하세요. 배포 빌드에서는 동적 링크 기능을 끄는 것이 권장됩니다.

## 코드 설명

- `cargo fmt --all --check`는 파일을 변경하지 않고 형식이 맞는지만 검사합니다.
- `cargo check --workspace`는 모든 챕터 예제를 빠르게 컴파일 검사합니다.
- `cargo clippy --workspace --all-targets`는 모든 패키지와 빌드 대상을 정적 분석합니다.
- `cargo run -p hello_bevy`는 첫 예제만 선택해 빌드하고 실행합니다.

첫 Bevy 빌드는 많은 의존성을 컴파일하므로 오래 걸릴 수 있습니다. 이후에는 변경된 부분만 다시 빌드하므로 빨라집니다.

## 실습 과제

1. 위 명령 중 창 실행을 제외한 모든 검사를 통과시키세요.
2. `src/main.rs`의 들여쓰기를 일부러 흐트러뜨린 뒤 `cargo fmt --all`로 복구하세요.
3. 존재하지 않는 타입 이름을 입력해 rust-analyzer와 Cargo가 보여 주는 오류 위치를 비교하세요.

## 심화 과제

사용하는 편집기에서 저장 시 자동 포맷을 활성화하세요. 이어서 운영체제에 맞는 빠른 링커 설정을 공식 Bevy Setup 문서에서 찾아 적용 전후의 두 번째 빌드 시간을 비교하세요.

## 다음 챕터

준비를 마쳤습니다. 다음 Part에서는 첫 화면에 여러 Entity와 Component를 만들며 Bevy ECS를 본격적으로 시작합니다.

