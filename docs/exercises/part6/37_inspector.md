# 37. Inspector 과제 해설

[본문으로 돌아가기](../../37_Inspector.md#실습-과제)

## P6-C37-P1 · Scale Action

UI가 Transform을 직접 바꾸지 않고 `SetScale { entity, scale }` EditorAction을 만듭니다. 같은 경로를 쓰면 Console과 undo/redo도 재사용할 수 있습니다.

## P6-C37-P2 · Y축 15도

각도 UI는 degree로 표시하되 Transform에는 quaternion/radian 변환을 적용합니다. 누적 회전과 절대 회전 명령을 이름으로 구분하세요.

## P6-C37-P3 · 표시 정밀도

모델 값을 반올림해 저장하지 말고 표시 포맷만 소수점 1자리 또는 3자리로 바꿉니다. 표현 정밀도와 데이터 정밀도는 별개입니다.

## P6-C37-A1 · Reflect 기반 공통 위젯

타입을 등록하고 `ReflectComponent`로 선택 Entity의 반사 값을 얻은 뒤 필드 타입이 f32, Vec3, bool인지 분기해 공통 위젯을 만듭니다.

- 표시 이름·범위·단위 같은 편집 metadata가 추가로 필요합니다.
- 모든 reflected 필드가 편집 가능해야 하는 것은 아니므로 allowlist를 둡니다.
- 수정은 EditorAction으로 변환해 검증과 undo 기록을 우회하지 않게 합니다.

수행 예제는 Inspector와 Console 모두 같은 `apply_action`을 사용하고 대상 Entity만 바뀌는지 검사합니다.

## 전체 코드 실행

```bash
cargo test -p world_editor --bin editor_model_solution
```

전체 코드: `examples/part6/world_editor/src/bin/editor_model_solution.rs`
