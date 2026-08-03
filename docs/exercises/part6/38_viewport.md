# 38. Viewport 과제 해설

[본문으로 돌아가기](../../38_Viewport.md#실습-과제)

## P6-C38-P1 · 선택 상자

고정 크기 Gizmo 대신 Mesh AABB를 world Transform으로 변환해 대상 크기에 맞춥니다. 회전 대상은 단순 world AABB와 oriented box 표현의 차이를 확인합니다.

## P6-C38-P2 · F focus

선택 Entity의 world translation 또는 bounds 중심을 orbit focus로 지정하고, bounds가 화면에 들어오는 거리로 보간합니다.

## P6-C38-P3 · XZ Grid

일정 간격의 X/Z 선을 Gizmo로 그리며 원점 축은 다른 색으로 표시합니다. 카메라 거리별 간격 단계가 있으면 멀리서 생기는 선 밀집을 줄일 수 있습니다.

## P6-C38-P4 · 리사이즈와 DPI 확인

본문 예제는 UI의 논리 픽셀 rect에 창 scale factor를 곱해 `Camera.viewport`의 물리 픽셀 position/size를 갱신합니다. Windows 디스플레이 배율을 바꾸거나 서로 다른 DPI의 모니터 사이로 창을 이동해 보세요.

## P6-C38-A1 · 크기 조절 패널

Splitter가 바꾼 패널 폭을 Resource 하나에 보관하세요. UI Node 폭, 물리 Viewport 계산과 `cursor_in_editor_viewport`가 같은 Resource를 읽어야 서로 어긋나지 않습니다. 0 크기 Viewport는 최소 1픽셀로 제한하거나 Camera를 비활성화합니다.

## 전체 코드 실행

```bash
cargo test -p world_editor --bin editor_model_solution
```

전체 코드: `examples/part6/world_editor/src/bin/editor_model_solution.rs`
