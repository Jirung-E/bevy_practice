# 29. Material 과제 해설

[본문으로 돌아가기](../../29_Material.md#실습-과제)

## P4-C29-P1 · metallic

같은 조명과 roughness에서 0과 1만 비교합니다. 금속은 base color를 반사색에 사용하므로 비금속과 하이라이트가 다르게 보입니다.

## P4-C29-P2 · roughness

0.05, 0.5, 1.0을 비교하면 반사가 날카로운 상태에서 넓고 흐린 상태로 바뀝니다. 조명이나 환경 반사가 없으면 차이가 잘 드러나지 않습니다.

## P4-C29-P3 · Material 공유

본체와 렌즈 Entity에 같은 `MeshMaterial3d<StandardMaterial>` Handle을 부여합니다. 한쪽의 Handle만 복제한 것이므로 material asset 수정은 둘 모두에 보입니다.

## P4-C29-A1 · 런타임 재질 편집기

선택한 Material Handle로 `Assets<StandardMaterial>::get_mut`을 호출해 metallic과 roughness를 변경합니다. 수행 예제는 키 입력에 해당하는 증감 뒤 PBR 유효 범위를 벗어나지 않게 clamp합니다.

- 공유 Handle 수정은 모든 사용 Entity에 반영됩니다.
- 한 제품만 바꾸려면 Material을 복제해 새 Handle을 부여합니다.
- 키를 누른 매 프레임이 아니라 `just_pressed` 또는 시간 기반 증감을 선택해 조작 속도를 통제합니다.

## 전체 코드 실행

```bash
cargo test -p product_showcase --bin showcase_solution
```

전체 코드: `examples/part4/product_showcase/src/bin/showcase_solution.rs`
