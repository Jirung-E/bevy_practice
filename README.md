# Bevy 실습 교재

Bevy 0.19와 Rust 2024 Edition을 기준으로, 작은 결과물을 직접 만들며 ECS와 게임·GUI 개발을 익히는 한국어 실습 교재입니다.

> 이 교재의 문서와 실행 가능한 예제 코드는 작성자와 OpenAI Codex가 협업하여 제작했습니다.

## 시작하기

1. [교재 안내와 전체 커리큘럼](docs/README.md)을 읽습니다.
2. [00. Bevy와 이 교재 소개](docs/00_Introduction.md)부터 순서대로 진행합니다.
3. 각 챕터의 예제 폴더에서 `cargo run -p <패키지명>`을 실행합니다.

전체 워크스페이스 확인:

```bash
cargo check --workspace
```

## HTML 교재

mdBook이 설치되어 있다면 전체 교재를 검색 가능한 정적 HTML로 만들 수 있습니다.

```powershell
mdbook build
mdbook serve --open
```

빌드 결과는 `book/`에 생성되며 Git에는 포함되지 않습니다. `mdbook serve`는 Markdown을 수정할 때 브라우저 화면을 자동으로 갱신합니다.

각 챕터 제목 아래의 **Part 전체 코드 보기** 버튼에서는 `examples/`의 실제 `Cargo.toml`, 실행 파일과 공용 구현을 확인할 수 있습니다. 문서에 코드 복사본을 두지 않으므로 예제를 수정한 뒤 다시 빌드하면 HTML 코드도 함께 갱신됩니다.

## GitHub Pages 배포

`.github/workflows/pages.yml`이 `main` 또는 `master` 브랜치에 푸시될 때 교재를 빌드하고 GitHub Pages에 배포합니다.

1. 프로젝트를 GitHub 저장소에 푸시합니다.
2. 저장소의 **Settings → Pages → Build and deployment → Source**에서 **GitHub Actions**를 선택합니다.
3. **Actions** 탭의 `Deploy mdBook to GitHub Pages` 실행이 끝나면 배포 주소를 확인합니다.

워크플로는 수동 실행도 지원합니다. 생성된 `book/` 폴더를 별도 브랜치에 커밋할 필요는 없습니다.

## 기준 환경

- Bevy 0.19
- Rust 1.95 이상
- Cargo resolver 3
- Rust 2024 Edition

Bevy는 아직 빠르게 발전하고 있어 버전 사이에 API 변경이 있을 수 있습니다. 이 저장소의 코드는 `Cargo.lock`에 기록된 버전을 기준으로 검증합니다.

## 완성 프로젝트

| Part | 프로젝트 | 최종 실행 명령 |
|---|---|---|
| 0 | Hello Bevy | `cargo run -p hello_bevy` |
| 1 | ECS Basics | `cargo run -p ecs_basics --bin states` |
| 2 | Space Survivor | `cargo run -p space_survivor --bin 20_game_over` |
| 3 | File Lens | `cargo run -p file_lens --bin 26_state` |
| 4 | Product Showcase | `cargo run -p product_showcase --bin 30_light` |
| 5 | TPS Training Ground | `cargo run -p tps_training --bin 35_navmesh` |
| 6 | World Editor | `cargo run -p world_editor --bin 40_console` |
| 7 | Production Arena | `cargo run -p production_structure --bin 45_optimization` |

기여와 버전 업데이트 절차는 [CONTRIBUTING.md](CONTRIBUTING.md)를 참고하세요.
