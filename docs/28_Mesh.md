# 28. Mesh와 기본 도형

## 학습 목표

- Mesh가 정점과 인덱스 데이터임을 설명할 수 있다.
- Assets에 Mesh를 추가하고 Handle을 사용할 수 있다.
- 기본 도형과 계층 구조로 복합 모델을 만들 수 있다.

## 이 내용으로 만들 수 있는 것

- 기본 도형을 조합한 프로토타입 배경과 소품
- 코드로 생성하는 지형·격자·디버그 도형
- 부모와 자식 Mesh로 구성된 복합 오브젝트

## 이번에 만들 결과물

Cuboid 본체, Torus 테두리, Sphere 렌즈를 가진 카메라 모양 제품과 Plane 바닥을 만듭니다. 제품은 천천히 회전합니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p product_showcase --bin 28_mesh
```

## 핵심 개념

Mesh는 삼각형을 구성하는 정점 위치, 법선, UV, 인덱스 등의 기하 데이터입니다. Mesh 데이터는 Assets에 한 번 저장되고 Entity는 Handle을 감싼 `Mesh3d` Component로 참조합니다.

복합 제품은 부모 Entity에 Transform을 두고 도형 Entity를 자식으로 만듭니다. 부모를 회전하면 모든 부품이 함께 움직입니다.

## 샘플 코드

```rust
commands
    .spawn((
        Product,
        Transform::from_xyz(0.0, 1.35, 0.0),
        Visibility::default(),
    ))
    .with_children(|product| {
        product.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.6, 1.8, 1.8))),
            MeshMaterial3d(material.clone()),
        ));
        product.spawn((
            Mesh3d(meshes.add(Sphere::new(0.42))),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 0.0, 1.12),
        ));
    });
```

## 코드 설명

- `Assets<Mesh>::add`는 Mesh를 저장하고 Handle을 반환합니다.
- Mesh Handle 자체 대신 `Mesh3d(handle)`을 Entity에 붙입니다.
- 자식 Transform은 부모 좌표계 기준의 로컬 Transform입니다.
- 부모에는 공간 전파에 필요한 Transform과 Visibility를 둡니다.
- 이 단계의 Material은 조명 없이 형태를 볼 수 있도록 `unlit: true`입니다.

## 실습 과제

1. 제품 본체 크기를 바꾸세요.
2. 렌즈를 두 개로 만들어 쌍안경 형태로 바꾸세요.
3. Capsule3d 또는 Cylinder 도형을 새 부품으로 추가하세요.

## 심화 과제

같은 Mesh Handle을 공유하는 제품을 여러 개 생성하고, 메모리에서 Mesh가 복제되지 않는 이유를 설명하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part4/28_mesh.md)

## 다음 챕터

정점에 저장한 UV를 실제 이미지의 픽셀과 연결하고 Base Color, Normal, Emissive 맵을 함께 적용합니다.
