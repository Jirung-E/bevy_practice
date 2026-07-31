# Part 2 전체 코드 · Space Survivor

## Cargo.toml

```toml
{{#include ../../examples/part2/space_survivor/Cargo.toml}}
```

## 13A · 이미지와 TextureAtlas 애니메이션

```rust
{{#include ../../examples/part2/space_survivor/src/bin/13a_texture_atlas.rs}}
```

## 공용 구현 · src/lib.rs

```rust
{{#include ../../examples/part2/space_survivor/src/lib.rs}}
```

## 챕터별 실행 파일

### 13 · 플레이어 이동
```rust
{{#include ../../examples/part2/space_survivor/src/bin/13_player_movement.rs}}
```

### 14 · 총알
```rust
{{#include ../../examples/part2/space_survivor/src/bin/14_bullets.rs}}
```

### 15 · 적
```rust
{{#include ../../examples/part2/space_survivor/src/bin/15_enemies.rs}}
```

### 16 · 충돌
```rust
{{#include ../../examples/part2/space_survivor/src/bin/16_collision.rs}}
```

### 17 · UI
```rust
{{#include ../../examples/part2/space_survivor/src/bin/17_ui.rs}}
```

### 18 · 사운드
```rust
{{#include ../../examples/part2/space_survivor/src/bin/18_sound.rs}}
```

### 19 · 저장
```rust
{{#include ../../examples/part2/space_survivor/src/bin/19_save.rs}}
```

### 19A · 게임 상태 저장과 불러오기
```rust
{{#include ../../examples/part2/space_survivor/src/bin/19a_save_game.rs}}
```

### 20 · 게임오버
```rust
{{#include ../../examples/part2/space_survivor/src/bin/20_game_over.rs}}
```

## 보충 과정 · 2D 그래픽과 효과

### 20A · 2D 렌더링 파이프라인과 WGSL

#### Rust

```rust
{{#include ../../examples/part2/space_survivor/src/bin/20a_rendering_pipeline.rs}}
```

#### WGSL

```wgsl
{{#include ../../examples/part2/space_survivor/assets/shaders/20a_pipeline.wgsl}}
```

### 20B · 절차적 우주 배경과 UV 애니메이션

#### Rust

```rust
{{#include ../../examples/part2/space_survivor/src/bin/20b_procedural_background.rs}}
```

#### WGSL

```wgsl
{{#include ../../examples/part2/space_survivor/assets/shaders/20b_starfield.wgsl}}
```

### 20C · Space Survivor 실전 셰이더 효과

#### Rust

```rust
{{#include ../../examples/part2/space_survivor/src/bin/20c_shader_effects.rs}}
```

#### Dissolve WGSL

```wgsl
{{#include ../../examples/part2/space_survivor/assets/shaders/20c_dissolve.wgsl}}
```

#### Shield WGSL

```wgsl
{{#include ../../examples/part2/space_survivor/assets/shaders/20c_shield.wgsl}}
```

### 20D · Rust-WGSL 연결과 Shader Hot Reload

20D는 20B와 20C의 실행 프로젝트 및 WGSL 파일을 그대로 사용합니다. 위 Rust-WGSL 연결을 유지한 채 shader 계산식과 상수를 실행 중 편집합니다.
