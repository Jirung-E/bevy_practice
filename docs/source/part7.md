# Part 7 전체 코드 · Production Structure

## Cargo.toml
```toml
{{#include ../../examples/part7/production_structure/Cargo.toml}}
```

## 핵심 모듈

### src/lib.rs
```rust
{{#include ../../examples/part7/production_structure/src/lib.rs}}
```

### src/components.rs
```rust
{{#include ../../examples/part7/production_structure/src/components.rs}}
```

### src/resources.rs
```rust
{{#include ../../examples/part7/production_structure/src/resources.rs}}
```

### src/schedule.rs
```rust
{{#include ../../examples/part7/production_structure/src/schedule.rs}}
```

## Plugins

### src/plugins/mod.rs
```rust
{{#include ../../examples/part7/production_structure/src/plugins/mod.rs}}
```

### src/plugins/core.rs
```rust
{{#include ../../examples/part7/production_structure/src/plugins/core.rs}}
```

### src/plugins/gameplay.rs
```rust
{{#include ../../examples/part7/production_structure/src/plugins/gameplay.rs}}
```

### src/plugins/presentation.rs
```rust
{{#include ../../examples/part7/production_structure/src/plugins/presentation.rs}}
```

### src/plugins/asset_catalog.rs
```rust
{{#include ../../examples/part7/production_structure/src/plugins/asset_catalog.rs}}
```

### src/plugins/diagnostics.rs
```rust
{{#include ../../examples/part7/production_structure/src/plugins/diagnostics.rs}}
```

## 챕터별 실행 파일

### 41 · Plugin
```rust
{{#include ../../examples/part7/production_structure/src/bin/41_plugin.rs}}
```

### 42 · 모듈화
```rust
{{#include ../../examples/part7/production_structure/src/bin/42_modules.rs}}
```

### 43 · Assets
```rust
{{#include ../../examples/part7/production_structure/src/bin/43_assets.rs}}
```

### 44 · ECS Architecture
```rust
{{#include ../../examples/part7/production_structure/src/bin/44_ecs_architecture.rs}}
```

### 45 · Optimization
```rust
{{#include ../../examples/part7/production_structure/src/bin/45_optimization.rs}}
```
