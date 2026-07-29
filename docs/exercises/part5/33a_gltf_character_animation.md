# 33A. glTF 캐릭터 애니메이션 과제 해설

## 실습 과제 힌트

1. 화면 아래 상태 문자열과 캐릭터 동작을 함께 확인하세요. 정지 상태의 Survey는 완전히 멈춘 자세가 아닙니다.
2. `change_animation`은 `Motion` 값이 달라질 때만 `play`를 호출합니다.
3. `Duration::from_millis(220)`을 바꾸면 이전 클립과 새 클립이 동시에 섞이는 시간이 달라집니다.

## 심화 과제 수행 방향

모델별 설정과 런타임 상태를 분리합니다.

```rust
struct CharacterAnimationSet {
    scene_path: &'static str,
    idle_name: &'static str,
    walk_name: &'static str,
    run_name: &'static str,
    walk_speed: f32,
    run_speed: f32,
}
```

이름을 인덱싱하기 전에 `get`으로 확인하고 누락된 항목을 수집하세요.

```rust
let Some(walk) = gltf.named_animations.get(set.walk_name) else {
    error!("필요한 애니메이션이 없습니다: {}", set.walk_name);
    return;
};
```

여러 이름이 필요하다면 하나씩 즉시 종료하기보다 전체 누락 목록을 UI에 보여 주는 편이 에셋 제작자가 한 번에 수정하기 좋습니다.
