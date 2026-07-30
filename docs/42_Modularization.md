# 42. Rust 모듈화

## 학습 목표

- 파일 구조와 기능 의존 방향을 일치시킬 수 있다.
- `pub`, `pub(crate)`, 비공개 항목을 구분할 수 있다.
- prelude 남용 없이 명시적인 모듈 계약을 만들 수 있다.

## 이 내용으로 만들 수 있는 것

- gameplay·presentation·assets가 분리된 유지보수 가능한 crate
- 공개 API가 작고 의존 방향이 분명한 기능 모듈
- 팀원이 동시에 수정해도 충돌이 적은 프로젝트 구조

## 이번에 만들 결과물

동일한 Plugin 셸을 여러 Rust 모듈로 구성하고 crate 루트가 외부에 노출할 최소 API만 re-export합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p production_structure --bin 42_modules
```

프로젝트 구조:

```text
src/
├── lib.rs
├── components.rs
├── resources.rs
├── schedule.rs
├── plugins/
│   ├── mod.rs
│   ├── core.rs
│   ├── asset_catalog.rs
│   ├── gameplay.rs
│   ├── presentation.rs
│   └── diagnostics.rs
└── bin/
```

## 핵심 개념

좋은 모듈 경계는 함께 변경되는 코드를 모으고 의존 방향을 예측 가능하게 만듭니다. 파일 하나당 타입 하나라는 규칙보다 기능 응집도가 중요합니다.

crate 외부의 실행 파일에는 `run`과 `LessonConfig`만 공개합니다. Plugin 구현 모듈은 private이며 plugins/mod.rs가 필요한 Plugin 타입만 crate 내부에 다시 노출합니다.

## 샘플 코드

```rust
mod components;
mod plugins;
mod resources;
mod schedule;

use plugins::{
    AssetCatalogPlugin, CorePlugin, DiagnosticsPlugin,
    GameplayPlugin, PresentationPlugin,
};

pub fn run(config: LessonConfig) {
    // App 조합만 담당
}
```

## 코드 설명

- `mod`는 파일을 컴파일 단위가 아닌 이름 공간과 공개 경계로 포함합니다.
- private 모듈의 세부 타입은 crate 사용자가 의존할 수 없습니다.
- `pub(crate)`는 테스트나 같은 crate의 다른 기능에만 필요한 API에 적합합니다.
- 거대한 project prelude는 출처를 숨기고 이름 충돌을 늘릴 수 있어 신중히 사용합니다.
- bin 파일은 라이브러리 API를 소비하는 얇은 진입점입니다.

## 실습 과제

1. 입력 관련 코드를 input.rs로 분리하세요.
2. 외부에서 plugins 모듈에 접근할 수 없는지 확인하세요.
3. 공개할 필요가 없는 pub를 찾아 줄이세요.

## 심화 과제

GameplayPlugin을 별도 workspace library crate로 옮긴다고 가정하고 필요한 공개 계약과 순환 의존 방지 규칙을 설계하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part7/42_modularization.md)

## 다음 챕터

Mesh와 Material 생성·로딩을 ArenaAssets Resource에 모아 시스템마다 에셋을 찾는 문제를 해결합니다.
