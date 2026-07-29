# Part 4 전체 코드 · Product Showcase

## Cargo.toml
```toml
{{#include ../../examples/part4/product_showcase/Cargo.toml}}
```

## 공용 구현 · src/lib.rs
```rust
{{#include ../../examples/part4/product_showcase/src/lib.rs}}
```

## 챕터별 실행 파일

### 27 · Camera3d
```rust
{{#include ../../examples/part4/product_showcase/src/bin/27_camera3d.rs}}
```

### 28 · Mesh
```rust
{{#include ../../examples/part4/product_showcase/src/bin/28_mesh.rs}}
```

### 28A · UV와 PBR 텍스처 매핑
```rust
{{#include ../../examples/part4/product_showcase/src/bin/28a_pbr_textures.rs}}
```

## 텍스처 출처와 라이선스

```markdown
{{#include ../../examples/part4/product_showcase/assets/textures/sci_fi_panel/LICENSE.md}}
```

### 29 · Material
```rust
{{#include ../../examples/part4/product_showcase/src/bin/29_material.rs}}
```

### 30 · Light
```rust
{{#include ../../examples/part4/product_showcase/src/bin/30_light.rs}}
```

### 30A · 커스텀 Material과 PBR 셰이더

#### Rust
```rust
{{#include ../../examples/part4/product_showcase/src/bin/30a_custom_pbr_material.rs}}
```

#### WGSL
```wgsl
{{#include ../../examples/part4/product_showcase/assets/shaders/30a_custom_pbr.wgsl}}
```

### 30B · 카메라 후처리 셰이더

#### Rust
```rust
{{#include ../../examples/part4/product_showcase/src/bin/30b_camera_post_process.rs}}
```

#### WGSL
```wgsl
{{#include ../../examples/part4/product_showcase/assets/shaders/30b_camera_post_process.wgsl}}
```
