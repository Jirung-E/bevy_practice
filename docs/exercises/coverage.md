# 과제 해설 coverage

이 표는 본문 과제와 선택형 해설의 연결 상태를 추적합니다.

- **본문**: 실습·심화 과제가 존재하고 목표가 명확함
- **해설**: 힌트, 접근 방법, 확인 기준, 수행 예시가 연결됨
- **코드 검증**: 코드 과제의 예제가 `cargo check` 또는 테스트로 검증됨
- `—`: 코드 작성이 핵심이 아닌 조사·관찰 과제

| Part | 챕터 | 본문 | 해설 | 코드 검증 |
|---:|---|:---:|:---:|:---:|
| 0 | 00. 교재 소개 | ✓ | [보기](part0/00_introduction.md) | — |
| 0 | 01. Rust 기초 | ✓ | [보기](part0/01_rust_basics.md) | `rust_basics_solution` |
| 0 | 02. Cargo | ✓ | [보기](part0/02_cargo.md) | workspace 명령 |
| 0 | 03. 첫 Bevy 프로젝트 | ✓ | [보기](part0/03_getting_started.md) | `getting_started_solution` |
| 0 | 04. 개발 환경 | ✓ | [보기](part0/04_development_environment.md) | — |
| 1 | 05. Entity | ✓ | [보기](part1/05_entity.md) | `entity_solution` |
| 1 | 06. Component | ✓ | [보기](part1/06_component.md) | `component_solution` |
| 1 | 07. System | ✓ | [보기](part1/07_system.md) | `system_solution` |
| 1 | 08. Query | ✓ | [보기](part1/08_query.md) | `query_solution` |
| 1 | 09. Resource | ✓ | [보기](part1/09_resource.md) | `resource_solution` |
| 1 | 10. Commands | ✓ | [보기](part1/10_commands.md) | `commands_solution` |
| 1 | 10A. Entity 수명 | ✓ | [보기](part1/10a_entity_lifecycle.md) | `entity_lifecycle` |
| 1 | 11. Messages와 Events | ✓ | [보기](part1/11_messages.md) | `messages_solution` |
| 1 | 12. States | ✓ | [보기](part1/12_states.md) | `states_solution` |
| 1 | 12A. Asset Loading | ✓ | [보기](part1/12a_asset_loading.md) | `asset_loading_solution` |
| 1 | 12B. Reflect와 DynamicWorld | ✓ | [보기](part1/12b_reflect_dynamic_world.md) | `dynamic_world`, `dynamic_world_solution` |
| 1 | 12C. Scene과 Save Game | ✓ | [보기](part1/12c_scene_save_game.md) | `save_game_model` |
| 1 | 12D. ECS 동작 추상화 | ✓ | [보기](part1/12d_behavior_abstraction.md) | `behavior_abstraction` |
| 1 | 12E. 입력 Action | ✓ | [보기](part1/12e_input_actions.md) | `input_actions` |
| 1 | 12F. FixedUpdate | ✓ | [보기](part1/12f_fixed_update.md) | `fixed_update` |
| 1 | 12G. ECS 테스트 | ✓ | [보기](part1/12g_ecs_testing.md) | `ecs_testing` |
| 2 | 13. 플레이어 이동 | ✓ | [보기](part2/13_player_movement.md) | `movement_solution` |
| 2 | 13A. TextureAtlas | ✓ | [보기](part2/13a_texture_atlas.md) | `texture_atlas_solution` |
| 2 | 14. 총알 | ✓ | [보기](part2/14_bullets.md) | `combat_solution` |
| 2 | 15. 적 | ✓ | [보기](part2/15_enemies.md) | `combat_solution` |
| 2 | 16. 충돌 | ✓ | [보기](part2/16_collision.md) | `combat_solution` |
| 2 | 17. UI | ✓ | [보기](part2/17_game_ui.md) | `game_flow_solution` |
| 2 | 18. 사운드 | ✓ | [보기](part2/18_sound.md) | `game_flow_solution` |
| 2 | 19. 저장 | ✓ | [보기](part2/19_save.md) | `game_flow_solution` |
| 2 | 19A. 게임 상태 저장 | ✓ | [보기](part2/19a_save_game_roundtrip.md) | `19a_save_game` |
| 2 | 20. 게임오버 | ✓ | [보기](part2/20_game_over.md) | `game_flow_solution` |
| 2 | 20A. 렌더링 파이프라인 | ✓ | [보기](part2/20a_rendering_pipeline.md) | `rendering_pipeline_solution` |
| 2 | 20B. 절차적 우주 배경 | ✓ | [보기](part2/20b_procedural_background.md) | `procedural_background` |
| 2 | 20C. 실전 셰이더 효과 | ✓ | [보기](part2/20c_shader_effects.md) | `shader_effects` |
| 2 | 20D. Shader Hot Reload | ✓ | [보기](part2/20d_shader_hot_reload.md) | `shader_reload_status_solution` |
| 3 | 21. GUI 애플리케이션 | ✓ | [보기](part3/21_gui_application.md) | `gui_workflow_solution` |
| 3 | 22. GUI 레이아웃 | ✓ | [보기](part3/22_gui_layout.md) | `gui_workflow_solution` |
| 3 | 23. GUI 이벤트 | ✓ | [보기](part3/23_gui_events.md) | `gui_workflow_solution` |
| 3 | 23A. 텍스트 입력·포커스·IME | ✓ | [보기](part3/23a_text_input_focus.md) | `text_input_focus` |
| 3 | 24. Drag & Drop | ✓ | [보기](part3/24_drag_and_drop.md) | `gui_workflow_solution` |
| 3 | 25. 파일 입출력 | ✓ | [보기](part3/25_file_io.md) | `gui_workflow_solution` |
| 3 | 25A. 비동기 파일 입출력 | ✓ | [보기](part3/25a_background_file_io.md) | `background_file_io` |
| 3 | 26. GUI 상태 | ✓ | [보기](part3/26_gui_state.md) | `gui_workflow_solution` |
| 4 | 27. Camera3d | ✓ | [보기](part4/27_camera3d.md) | `showcase_solution` |
| 4 | 28. Mesh | ✓ | [보기](part4/28_mesh.md) | `showcase_solution` |
| 4 | 28A. UV와 PBR 텍스처 | ✓ | [보기](part4/28a_uv_pbr_textures.md) | `28a_pbr_textures` |
| 4 | 29. Material | ✓ | [보기](part4/29_material.md) | `showcase_solution` |
| 4 | 30. Light | ✓ | [보기](part4/30_light.md) | `showcase_solution` |
| 4 | 30A. 커스텀 PBR Material | ✓ | [보기](part4/30a_custom_pbr_material.md) | `30a_custom_pbr_material` |
| 4 | 30B. 카메라 후처리 | ✓ | [보기](part4/30b_camera_post_process.md) | `30b_camera_post_process` |
| 4 | 30C. 3D Object Picking | ✓ | [보기](part4/30c_object_picking.md) | `30c_object_picking` |
| 4 | 30D. 멀티 카메라·RenderLayers | ✓ | [보기](part4/30d_multi_camera_layers.md) | `multi_camera_layers` |
| 5 | 31. TPS 기초 | ✓ | [보기](part5/31_tps_core.md) | `tps_rules_solution` |
| 5 | 32. TPS 카메라 | ✓ | [보기](part5/32_tps_camera.md) | `tps_rules_solution` |
| 5 | 33. 애니메이션 | ✓ | [보기](part5/33_animation.md) | `tps_rules_solution` |
| 5 | 33A. glTF 캐릭터 애니메이션 | ✓ | [보기](part5/33a_gltf_character_animation.md) | `33a_gltf_character` |
| 5 | 34. 물리 | ✓ | [보기](part5/34_physics.md) | `tps_rules_solution` |
| 5 | 34A. TPS 카메라 충돌 | ✓ | [보기](part5/34a_tps_camera_collision.md) | `camera_collision` |
| 5 | 35. NavMesh | ✓ | [보기](part5/35_navmesh.md) | `tps_rules_solution` |
| 6 | 36. Hierarchy | ✓ | [보기](part6/36_hierarchy.md) | `editor_model_solution` |
| 6 | 37. Inspector | ✓ | [보기](part6/37_inspector.md) | `editor_model_solution` |
| 6 | 38. Viewport | ✓ | [보기](part6/38_viewport.md) | `editor_model_solution` |
| 6 | 39. Asset Browser | ✓ | [보기](part6/39_asset_browser.md) | `editor_model_solution` |
| 6 | 40. Console | ✓ | [보기](part6/40_console.md) | `editor_model_solution` |
| 6 | 40A. World Editor Scene I/O | ✓ | [보기](part6/40a_world_editor_scene_io.md) | `40a_scene_io` |
| 6 | 40B. 스크립트 연결·Hot Reload | ✓ | [보기](part6/40b_script_attachment.md) | `script_attachment` |
| 7 | 41. Plugin | ✓ | [보기](part7/41_plugin.md) | `production_solution` |
| 7 | 42. 모듈화 | ✓ | [보기](part7/42_modularization.md) | `production_solution` |
| 7 | 43. Assets | ✓ | [보기](part7/43_assets.md) | `production_solution` |
| 7 | 44. ECS 아키텍처 | ✓ | [보기](part7/44_ecs_architecture.md) | `production_solution` |
| 7 | 44A. 결정론·입력 리플레이 | ✓ | [보기](part7/44a_deterministic_replay.md) | `deterministic_replay` |
| 7 | 45. 최적화 | ✓ | [보기](part7/45_optimization.md) | `production_solution` |
| 7 | 46. 데스크톱과 WASM 배포 | ✓ | [보기](part7/46_deployment.md) | 배포 스크립트와 CI 검증 |
