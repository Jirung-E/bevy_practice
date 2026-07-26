# 기여 가이드

## 문서 변경

모든 챕터는 다음 섹션을 유지합니다.

- 학습 목표
- 이번에 만들 결과물
- 핵심 개념
- 샘플 코드
- 코드 설명
- 실습 과제
- 심화 과제
- 다음 챕터

API를 변경할 때는 같은 챕터의 실제 예제와 코드 블록을 함께 수정합니다. 각 챕터는 이전 결과물에서 자연스럽게 이어지고, 직접 실행할 수 있는 명령을 제공해야 합니다.

## 제출 전 검증

저장소 루트에서 다음 명령을 실행합니다.

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

그래픽 예제는 해당 챕터의 실행 명령으로 직접 열어 조작과 화면 구성을 확인합니다.

Windows에서는 모든 예제를 순차적으로 짧게 실행해 경고, 오류, panic을 검사할 수 있습니다.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\smoke_examples.ps1
```

Windows MSVC에서는 Bevy 전체 기능의 디버그 심볼이 PDB 크기 한도를 넘을 수 있습니다. 루트의 `[profile.dev] debug = 0`은 `LNK1140: limit exceeded for program database`를 방지하기 위한 설정이므로 제거하지 않습니다.

## 버전 업데이트

Bevy 버전을 올릴 때는 공식 마이그레이션 가이드를 먼저 읽고 다음 항목을 함께 갱신합니다.

1. 루트 `Cargo.toml`과 `Cargo.lock`
2. 최소 Rust 버전
3. Avian과 bevy_landmass 호환 버전
4. 모든 문서의 버전 표기와 API 코드
5. 전체 워크스페이스 검증 결과
