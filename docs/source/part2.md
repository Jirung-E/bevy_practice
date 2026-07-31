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

### 20B · Material2d 커스텀 셰이더

#### Rust

```rust
{{#include ../../examples/part2/space_survivor/src/bin/20b_material_2d.rs}}
```

#### WGSL

```wgsl
{{#include ../../examples/part2/space_survivor/assets/shaders/20b_sprite_effect.wgsl}}
```

### 20C · Rust-WGSL 연결과 Shader Hot Reload

20C는 20B의 실행 프로젝트를 그대로 사용하며, 아래 두 파일을 실행 중 편집합니다.

#### Rust

```rust
{{#include ../../examples/part2/space_survivor/src/bin/20b_material_2d.rs}}
```

#### Hot reload 대상 WGSL

```wgsl
{{#include ../../examples/part2/space_survivor/assets/shaders/20b_sprite_effect.wgsl}}
```

### 20D · ECS로 만드는 2D 파티클 시스템

```rust
{{#include ../../examples/part2/space_survivor/src/bin/20d_particle_system.rs}}
```
