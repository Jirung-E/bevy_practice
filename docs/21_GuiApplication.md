# 21. 게임이 아닌 Bevy 애플리케이션

## 학습 목표

- Bevy를 일반 데스크톱 앱 프레임워크로 사용할 수 있다.
- 게임과 GUI 애플리케이션의 공통 구조를 설명할 수 있다.
- File Lens 프로젝트의 데이터 흐름을 이해한다.

## 이번에 만들 결과물

`File Lens`라는 제목의 창과 시작 문구를 표시합니다. 이후 파일을 끌어다 놓아 내용과 메타데이터를 검사하는 도구로 발전시킵니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p file_lens --bin 21_gui_app
```

## 핵심 개념

Bevy App, ECS, Plugin, Schedule은 게임 전용 개념이 아닙니다. 창 이벤트를 받고 상태에 따라 UI를 갱신하는 일반 프로그램에도 적용할 수 있습니다.

File Lens의 데이터 흐름은 다음과 같습니다.

```text
운영체제 파일 이벤트 → FileModel Resource → UI Text
버튼 Interaction → 파일 작업 → 상태 문구
처리 결과 → AppMode State → 모드 표시
```

## 샘플 코드

```rust
App::new()
    .insert_resource(ClearColor(Color::srgb(0.025, 0.032, 0.05)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "File Lens - Bevy GUI Practice".into(),
            resolution: WindowResolution::new(1080, 700),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup)
    .run();
```

## 코드 설명

- DefaultPlugins는 창, 입력, UI 렌더링, 운영체제 이벤트를 제공합니다.
- GUI에서도 Camera2d가 UI 렌더링을 담당합니다.
- WindowPlugin 설정으로 제품 이름과 기본 창 크기를 지정합니다.
- 이번 Part는 `LessonConfig`로 단계별 기능을 활성화하지만 최종 코드는 하나의 라이브러리에 유지됩니다.

게임과 달리 GUI 도구는 프레임마다 모든 데이터를 계산할 필요가 없습니다. 변경 감지와 이벤트를 활용해 필요한 때만 화면을 갱신하는 설계가 중요합니다.

## 실습 과제

1. 창 제목과 크기를 바꾸세요.
2. 최소 창 크기를 설정하세요.
3. 시작 문구의 색과 글자 크기를 변경하세요.

## 심화 과제

창 크기가 달라져도 중앙 문구가 유지되는 이유를 Node의 percent 단위와 카메라 관점에서 설명하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part3/21_gui_application.md)

## 다음 챕터

Flexbox 기반 Node로 파일 목록, 미리보기, 도구 모음 영역을 배치합니다.
