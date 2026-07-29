# Bevy 실습 교재

이 교재는 API 목록을 외우는 대신, 매 챕터에서 실행 가능한 결과물을 하나씩 완성하도록 구성합니다. 앞에서 만든 개념과 코드는 뒤의 프로젝트에서 다시 사용합니다.

## 학습 방법

1. 각 챕터의 **학습 목표**와 **이번에 만들 결과물**을 먼저 확인합니다.
2. 샘플 코드를 직접 입력하거나 해당 예제 폴더를 엽니다.
3. `cargo run -p <패키지명>`으로 실행합니다.
4. 코드 설명을 읽고 값을 바꾸어 다시 실행합니다.
5. 실습 과제와 심화 과제를 해결합니다.

막히면 먼저 `cargo check --workspace`로 컴파일 오류를 확인하세요. 컴파일러가 표시한 첫 번째 오류부터 해결하는 것이 좋습니다.

## 버전 정책

이 교재의 현재 기준은 **Bevy 0.19.0**, **Rust 1.95 이상**, **Rust 2024 Edition**입니다.

- `Cargo.toml`은 호환 가능한 Bevy 0.19 패치를 허용합니다.
- `Cargo.lock`은 실제 검증한 의존성 버전을 고정합니다.
- Bevy의 새 마이너 버전으로 올릴 때는 공식 마이그레이션 가이드를 따라 전체 예제를 다시 검사합니다.

## 전체 커리큘럼

### Part 0. 준비

- [00. Bevy와 이 교재 소개](00_Introduction.md)
- [01. 실습에 필요한 Rust 기초](01_RustBasics.md)
- [02. Cargo로 프로젝트 관리하기](02_Cargo.md)
- [03. 첫 Bevy 프로젝트 만들기](03_GettingStarted.md)
- [04. 개발 환경 점검하기](04_DevelopmentEnvironment.md)

### Part 1. Bevy 기초

- [05. Entity: 월드의 대상 만들기](05_Entity.md)
- [06. Component: Entity에 데이터 붙이기](06_Component.md)
- [07. System: 데이터에 로직 적용하기](07_System.md)
- [08. Query: 원하는 데이터 찾기](08_Query.md)
- [09. Resource: 전역 데이터 관리하기](09_Resource.md)
- [10. Commands: 월드 구조 변경하기](10_Commands.md)
- [11. Messages와 Events: 시스템 사이 통신](11_MessagesAndEvents.md)
- [12. States: 화면과 흐름 관리하기](12_States.md)

### Part 2. 2D 게임 제작

- [13. 플레이어 이동](13_PlayerMovement.md)
- [14. 총알 발사](14_Bullets.md)
- [15. 적 생성과 이동](15_Enemies.md)
- [16. 충돌과 점수](16_Collision.md)
- [17. 게임 UI](17_GameUI.md)
- [18. 사운드](18_Sound.md)
- [19. 최고 점수 저장](19_Save.md)
- [20. 게임오버와 재시작](20_GameOver.md)

### Part 3. GUI 애플리케이션 제작

- [21. 게임이 아닌 Bevy 애플리케이션](21_GuiApplication.md)
- [22. GUI 레이아웃](22_GuiLayout.md)
- [23. 버튼과 이벤트](23_GuiEvents.md)
- [24. 파일 Drag & Drop](24_DragAndDrop.md)
- [25. 파일 입출력](25_FileIO.md)
- [26. GUI 상태 관리](26_GuiState.md)

### Part 4. 3D 입문

- [27. Camera3d와 3D 좌표](27_Camera3d.md)
- [28. Mesh와 기본 도형](28_Mesh.md)
- [29. StandardMaterial과 PBR](29_Material.md)
- [30. Light와 그림자](30_Light.md)

### Part 5. 3D 게임 제작

- [31. TPS 플레이어 기초](31_TpsCore.md)
- [32. TPS 추적 카메라](32_TpsCamera.md)
- [33. 캐릭터 애니메이션](33_Animation.md)
- [34. Avian 3D 물리](34_Physics.md)
- [35. Landmass NavMesh](35_NavMesh.md)

### Part 6. 게임 에디터 제작

- [36. Hierarchy](36_Hierarchy.md)
- [37. Inspector](37_Inspector.md)
- [38. Viewport](38_Viewport.md)
- [39. Asset Browser](39_AssetBrowser.md)
- [40. Console](40_Console.md)

### Part 7. 실전 프로젝트 구조

- [41. Plugin 경계 설계](41_Plugin.md)
- [42. Rust 모듈화](42_Modularization.md)
- [43. Assets 관리](43_Assets.md)
- [44. ECS 아키텍처](44_EcsArchitecture.md)
- [45. 측정 기반 최적화](45_Optimization.md)
- [46. 데스크톱과 WASM 빌드·배포](46_DesktopWasmDeployment.md)

## 예제 디렉터리

```text
examples/
├── part0/
│   └── hello_bevy/
├── part1/
│   └── ecs_basics/
└── part2/
│   └── space_survivor/
└── part3/
│   └── file_lens/
└── part4/
│   └── product_showcase/
└── part5/
│   └── tps_training/
└── part6/
│   └── world_editor/
└── part7/
    └── production_structure/
```

챕터가 추가될 때마다 독립 실행 가능한 패키지가 워크스페이스에 추가됩니다. 덕분에 최신 챕터를 수정해도 이전 단계의 완성 코드를 비교할 수 있습니다.

## 공식 참고 자료

- [Bevy Quick Start](https://bevy.org/learn/quick-start/)
- [Bevy API 문서](https://docs.rs/bevy/0.19.0/bevy/)
- [Bevy 공식 예제](https://bevy.org/examples/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
