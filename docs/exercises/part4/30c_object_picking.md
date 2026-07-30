# 30C. 3D Object Picking 과제 해설

[본문으로 돌아가기](../../30C_ObjectPicking.md#실습-과제)

## P4-C30C-P1 · 선택 대상 추가

기존 `spawn_object`를 재사용해 Mesh, 기본·hover·selected 재질과 `Pickable::default()`를 함께 넣습니다. 화면에 보인다는 사실과 선택 가능하다는 사실은 별개이므로 marker 누락 여부를 먼저 확인합니다.

## P4-C30C-P2 · 선택 제외

이 예제는 `require_markers: true`이므로 `Pickable`이 없으면 Mesh picking 대상이 아닙니다. 모든 Mesh를 기본 대상으로 사용하는 설정에서는 `Pickable::IGNORE`가 더 명시적입니다.

## P4-C30C-P3 · Inspector 정보

`Selection`이 변경됐을 때만 선택 Entity의 `SelectableObject`와 `Transform`을 조회해 Text를 갱신합니다. 매 프레임 문자열을 새로 만들 필요가 없습니다.

## P4-C30C-A1 · hit gizmo와 다중 선택

`PointerInteraction::get_nearest_hit()`에서 hit 위치와 법선을 읽어 `Gizmos::sphere`와 `Gizmos::arrow`로 표시합니다. 다중 선택은 단일 `Option<Entity>` 대신 `HashSet<Entity>`를 사용하고, 일반 클릭은 집합을 비운 뒤 하나를 추가하며 Shift 클릭은 대상의 포함 여부를 토글합니다.

## 전체 코드 실행

```bash
cargo run -p product_showcase --bin 30c_object_picking
cargo test -p product_showcase --bin 30c_object_picking
```

전체 코드: `examples/part4/product_showcase/src/bin/30c_object_picking.rs`
