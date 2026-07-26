# Bevy 실습 교재

Bevy 0.19와 Rust 2024 Edition을 기준으로, 작은 결과물을 직접 만들며 ECS와 게임·GUI 개발을 익히는 한국어 실습 교재입니다.

## 시작하기

1. [교재 안내와 전체 커리큘럼](docs/README.md)을 읽습니다.
2. [00. Bevy와 이 교재 소개](docs/00_Introduction.md)부터 순서대로 진행합니다.
3. 각 챕터의 예제 폴더에서 `cargo run -p <패키지명>`을 실행합니다.

전체 워크스페이스 확인:

```bash
cargo check --workspace
```

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
