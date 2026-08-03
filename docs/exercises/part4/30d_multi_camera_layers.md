# 30D 선택형 과제 해설

`RenderLayers`는 Entity마다 하나의 숫자만 선택하는 분류가 아닙니다. `with` 또는 `from_layers`로 여러 비트를 켤 수 있으므로 “메인에도 보이고 미니맵에도 보이는 대상”을 복제할 필요가 없습니다.

반응형 Viewport는 다음 순서로 계산하세요.

1. `Window::physical_width/height`에서 짧은 변을 찾습니다.
2. 비율을 곱하고 최소·최대 크기를 제한합니다.
3. 오른쪽 위 여백을 뺀 `physical_position`과 정사각형 `physical_size`를 만듭니다.
4. `WindowResized`가 왔을 때 Camera의 Viewport를 교체합니다.

Camera를 숨길 때 Entity를 제거할 필요는 없습니다. `Camera::is_active`는 Transform과 Layer 구성을 보존한 채 렌더 패스만 끌 수 있습니다.
