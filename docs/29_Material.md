# 29. StandardMaterial과 PBR

## 학습 목표

- Mesh와 Material의 역할을 구분할 수 있다.
- StandardMaterial의 기본 PBR 속성을 조절할 수 있다.
- Material Handle을 여러 Entity가 공유할 수 있다.

## 이번에 만들 결과물

파란 금속 본체, 주황색 렌즈 테두리, 거친 바닥 재질을 적용한 제품 전시 장면을 만듭니다.

```bash
cargo run -p product_showcase --bin 29_material
```

## 핵심 개념

PBR은 물리 기반 렌더링으로 재질과 조명 관계를 일관된 규칙으로 계산합니다.

- `base_color`: 표면의 기본 색
- `metallic`: 0은 비금속, 1은 금속
- `perceptual_roughness`: 낮을수록 반사가 선명하고 높을수록 퍼짐

Material도 Assets에 저장되며 `MeshMaterial3d<StandardMaterial>` Component가 Handle을 보관합니다.

## 샘플 코드

```rust
let body_material = materials.add(StandardMaterial {
    base_color: Color::srgb(0.08, 0.35, 0.82),
    metallic: 0.65,
    perceptual_roughness: 0.22,
    ..default()
});

commands.spawn((
    Mesh3d(meshes.add(Cuboid::new(2.6, 1.8, 1.8))),
    MeshMaterial3d(body_material),
));
```

## 코드 설명

- 색은 sRGB 공간에서 입력하고 렌더러가 조명 계산용 선형 공간으로 처리합니다.
- 금속 표면은 base color가 반사 색에 영향을 줍니다.
- 거칠기 0은 완벽한 거울을 뜻하지 않으며 환경과 조명이 있어야 반사가 보입니다.
- 같은 Material Handle을 공유하면 Assets 값을 수정할 때 모든 사용자가 함께 바뀝니다.
- 이 단계는 형태를 확인할 최소 AmbientLight만 두며 다음 챕터에서 의도적인 조명을 구성합니다.

Bevy 0.19에서 AmbientLight는 전역 Resource가 아니라 장면에 spawn하는 Component입니다.

## 실습 과제

1. metallic을 0과 1로 비교하세요.
2. roughness를 0.05, 0.5, 1.0으로 비교하세요.
3. 본체와 렌즈가 같은 Material을 공유하게 바꾸세요.

## 심화 과제

키 입력으로 Materials의 StandardMaterial 값을 수정해 런타임 재질 편집기를 만드세요.

## 다음 챕터

DirectionalLight와 PointLight, 그림자를 추가해 형태와 재질을 입체적으로 드러냅니다.

