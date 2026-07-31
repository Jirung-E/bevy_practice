# Part 2 보충. 2D 그래픽과 효과

Space Survivor의 이동·전투·저장·게임오버를 먼저 완성한 뒤 선택해서 진행하는 렌더링 심화 과정입니다.

- 20A에서는 GPU가 Mesh의 triangle primitive를 화면 픽셀로 만드는 과정을 관찰합니다.
- 20B에서는 `Material2d`와 WGSL로 스프라이트의 정점과 색상을 바꿉니다.
- 20C에서는 Rust와 WGSL의 binding 관계 및 shader hot reload를 실습합니다.
- 20D에서는 ECS 기반 파티클로 추진 불꽃과 폭발을 만듭니다.

2D 게임 제작만 목표라면 이 과정을 건너뛰고 Part 3으로 이동해도 됩니다. Part 4의 30A·30B에서 3D 커스텀 Material과 후처리 셰이더를 학습할 예정이라면 20A~20C를 먼저 진행하는 것이 좋습니다.
