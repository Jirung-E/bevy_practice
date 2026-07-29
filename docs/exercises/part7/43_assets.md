# 43. Assets 과제 해설

[본문으로 돌아가기](../../43_Assets.md#실습-과제)

## P7-C43-P1 · 바닥 에셋

바닥 Mesh와 Material 생성도 `ArenaAssets` 초기화로 옮기고 spawn System은 Handle만 복제합니다. System 실행마다 Assets에 같은 기하를 추가하지 않습니다.

## P7-C43-P2 · Material 배열

색상 변형 Handle을 고정 배열이나 Vec으로 관리하고 의미 있는 enum/index 규칙을 둡니다. 원시 숫자를 여러 System에서 공유하면 순서 변경이 버그가 됩니다.

## P7-C43-P3 · 이미지 로드 상태

AssetServer Handle을 Resource에 보관하고 `LoadState`가 Loaded/Failed로 바뀔 때만 로그를 남깁니다. 매 프레임 Loading 로그를 출력하지 않습니다.

## P7-C43-A1 · AssetLoadingPlugin

에셋을 UI, gameplay, audio 같은 그룹으로 나누고 각 Handle 상태에서 진행률과 실패 목록을 계산합니다. 실패가 필수 에셋인지 선택 에셋인지에 따라 Error 전환 또는 fallback 사용을 결정합니다.

수행 예제는 실패 에셋을 fallback 상태로 바꾼 뒤 그룹이 완료되는지 검사합니다.

- 단순 파일 개수 진행률은 큰 파일과 작은 파일을 같은 비중으로 봅니다.
- fallback 자체도 미리 생성하거나 반드시 패키지에 포함해야 합니다.
- typed Handle과 논리 경로 상수를 사용해 문자열 경로 중복을 줄입니다.

## 전체 코드 실행

```bash
cargo test -p production_structure --bin production_solution
```

전체 코드: `examples/part7/production_structure/src/bin/production_solution.rs`
