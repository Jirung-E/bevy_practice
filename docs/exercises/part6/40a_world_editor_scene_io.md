# 40A. World Editor Scene I/O 과제 해설

## 실습 과제 힌트

1. 저장 파일은 저장소 루트의 `target/world_editor_scene.scn.ron`에 생성됩니다.
2. dirty 상태는 저장 이후의 생성·이동·삭제에서 다시 참이 됩니다.
3. 열기는 역직렬화와 관계 검증이 모두 성공한 뒤에만 기존 Editable Entity를 제거합니다.

## 심화 과제 수행 방향

원자적 저장은 같은 디렉터리에 임시 파일을 써야 파일 교체가 같은 파일 시스템 안에서 처리됩니다.

```text
scene.scn.ron.tmp에 전체 내용 쓰기
→ flush/sync 확인
→ 기존 scene.scn.ron 교체
```

Windows에서는 대상 파일이 이미 있을 때 rename 동작이 플랫폼별로 다를 수 있으므로 백업 파일을 거치는 전략과 실패 복구를 함께 설계하세요.

dirty 상태를 단순 bool이 아니라 내용 기반으로 계산하려면 마지막 저장 직렬화 결과의 해시와 현재 staging World 직렬화 결과의 해시를 비교할 수 있습니다. Entity 순서 때문에 같은 내용이 다른 문자열이 되지 않도록 `SceneId` 순으로 정렬된 별도 문서 모델을 만드는 것이 좋습니다.
