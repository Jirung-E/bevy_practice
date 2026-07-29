# 28. Mesh 과제 해설

[본문으로 돌아가기](../../28_Mesh.md#실습-과제)

## P4-C28-P1 · 본체 크기

Mesh 자체 치수를 바꾸는 방식과 Transform scale을 바꾸는 방식을 비교하세요. 충돌체·UV·법선까지 제품 규격으로 유지해야 한다면 Mesh 생성 치수를 명시하는 편이 이해하기 쉽습니다.

## P4-C28-P2 · 쌍안경 렌즈

같은 렌즈 Mesh Handle을 가진 Entity 두 개를 만들고 X 위치만 대칭으로 둡니다. 기하 데이터는 공유하면서 Transform은 각 인스턴스가 소유합니다.

## P4-C28-P3 · 새 부품

Capsule3d 또는 Cylinder를 `Assets<Mesh>`에 한 번 추가한 뒤 `Mesh3d(handle)`로 배치합니다. 축 방향이 기대와 다르면 Mesh를 다시 만들기보다 Transform 회전을 먼저 검토합니다.

## P4-C28-A1 · Mesh Handle 공유

`Handle<Mesh>`은 `Assets<Mesh>`에 저장된 기하 데이터의 참조입니다. Handle을 복제해도 vertex/index 버퍼 전체가 복제되지 않습니다. 각 Entity는 같은 asset ID와 별도 Transform을 가집니다.

수행 예제는 네 제품의 body/lens ID가 모두 같은지 검사합니다. 제품별로 Mesh를 수정해야 하면 먼저 `Assets<Mesh>::get(...).clone()`으로 명시적인 복제본을 만든 뒤 새 Handle을 부여해야 합니다.

## 전체 코드 실행

```bash
cargo test -p product_showcase --bin showcase_solution
```

전체 코드: `examples/part4/product_showcase/src/bin/showcase_solution.rs`
