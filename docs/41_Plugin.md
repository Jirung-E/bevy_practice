# 41. Plugin 경계 설계

## 학습 목표

- 기능 단위를 Bevy Plugin으로 캡슐화할 수 있다.
- Plugin 사이의 공개 계약을 설계할 수 있다.
- 환경별 Plugin 조합을 구성할 수 있다.

## 이 내용으로 만들 수 있는 것

- 전투·UI·저장 기능을 독립적으로 켜고 끄는 기능 묶음
- 여러 프로젝트에서 재사용하는 사내 Bevy Plugin
- 서버·클라이언트·도구마다 다른 Plugin 조합

## 이번에 만들 결과물

CorePlugin과 PresentationPlugin만으로 실행되는 Production Arena의 최소 셸을 만듭니다. 화면에는 카메라, 조명, `PLUGIN SHELL READY` HUD가 표시됩니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p production_structure --bin 41_plugin
```

## 핵심 개념

Plugin은 관련 Resource, Message, System, SystemSet을 App에 등록하는 구성 단위입니다. 단순히 큰 파일을 옮기는 것이 아니라 기능의 소유권과 외부 계약을 정합니다.

Production Arena는 다음 Plugin으로 나뉩니다.

- CorePlugin: 공유 Score, EnemyDefeated, GameSet
- AssetCatalogPlugin: 렌더 에셋 Handle
- GameplayPlugin: 입력, 이동, 적 처치, 점수
- PresentationPlugin: 카메라, 조명, HUD
- DiagnosticsPlugin: 프레임 진단

## 샘플 코드

```rust
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_message::<EnemyDefeated>()
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Simulation,
                    GameSet::Feedback,
                )
                    .chain(),
            );
    }
}
```

## 코드 설명

- Plugin의 build는 App 구성을 선언하고 게임 프레임 로직을 직접 실행하지 않습니다.
- CorePlugin은 여러 기능이 공유할 타입과 실행 순서 계약을 소유합니다.
- run 함수는 LessonConfig에 따라 Plugin을 조합합니다.
- 서버 빌드는 Presentation을 제외하고 Gameplay를 넣는 식으로 같은 코드를 재사용할 수 있습니다.
- 서로 순환 의존하는 Plugin은 경계가 잘못되었거나 공통 계약이 Core로 이동해야 한다는 신호입니다.

## 실습 과제

1. PausePlugin을 추가하고 P 키 입력 Resource를 등록하세요.
2. PresentationPlugin 없이 App을 구성하는 함수를 만드세요.
3. 각 Plugin이 소유한 Component와 Resource를 표로 정리하세요.

## 심화 과제

클라이언트, 전용 서버, 자동 테스트용 PluginGroup을 각각 설계하고 공통·전용 Plugin 조합을 비교하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part7/41_plugin.md)

## 다음 챕터

Plugin 구현을 components, resources, schedule, plugins Rust 모듈로 나누고 공개 범위를 제한합니다.
