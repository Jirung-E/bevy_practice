# Part 1 전체 코드 · ECS Basics

## Cargo.toml

```toml
{{#include ../../examples/part1/ecs_basics/Cargo.toml}}
```

## 챕터별 실행 파일

### 05 · Entity

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/05_entity.rs}}
```

### 06 · Component

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/06_component.rs}}
```

### 07 · System

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/07_system.rs}}
```

### 08 · Query

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/08_query.rs}}
```

### 09 · Resource

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/09_resource.rs}}
```

### 10 · Commands

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/10_commands.rs}}
```

### 10A · Entity 수명과 제거 감지

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/10a_entity_lifecycle.rs}}
```

### 11 · Messages and Events

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/11_messages.rs}}
```

### 12 · States

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12_states.rs}}
```

### 12A · AssetServer와 Loading State

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12a_asset_loading.rs}}
```

### 12B · Reflect와 DynamicWorld(Scene)

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12b_dynamic_world.rs}}
```

### 12C · Scene과 Save Game 설계

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12c_save_game_model.rs}}
```

### 12D · ECS 동작 추상화와 스킬 시스템

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12d_behavior_abstraction.rs}}
```

### 12E · 입력 Action과 장치 독립적인 명령

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12e_input_actions.rs}}
```

### 12F · FixedUpdate와 입력 버퍼

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12f_fixed_update.rs}}
```

### 12G · ECS 테스트 전략

```rust
{{#include ../../examples/part1/ecs_basics/src/bin/12g_ecs_testing.rs}}
```
