# 33. 캐릭터 애니메이션

## 학습 목표

- 게임 플레이 Transform과 시각 애니메이션을 분리할 수 있다.
- 속도에 따라 절차적 애니메이션을 재생할 수 있다.
- glTF AnimationPlayer로 확장할 구조를 이해한다.

## 이번에 만들 결과물

캐릭터가 이동할 때 캡슐 모델이 걸음 주기에 맞춰 위아래로 움직이고 좌우로 기울어집니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p tps_training --bin 33_animation
```

## 핵심 개념

이 챕터는 외부 모델 없이 실행되도록 절차적 애니메이션을 사용합니다. 이동 System은 MotionAmount만 기록하고, 애니메이션 System이 PlayerVisual 자식 Transform을 변경합니다.

실제 캐릭터 에셋에서는 같은 상태 값으로 AnimationGraph와 AnimationPlayer의 Idle, Walk, Run 클립을 전환하고 혼합합니다. 게임 로직이 특정 클립 Handle을 직접 알지 않게 하는 원칙은 같습니다.

## 샘플 코드

```rust
fn animate_player(
    time: Res<Time>,
    player: Single<&MotionAmount, With<Player>>,
    mut visuals: Query<&mut Transform, With<PlayerVisual>>,
) {
    let phase = time.elapsed_secs() * 10.0;
    for mut transform in &mut visuals {
        transform.translation.y = phase.sin().abs() * 0.08 * player.0;
        transform.rotation =
            Quat::from_rotation_z(phase.sin() * 0.08 * player.0);
    }
}
```

## 코드 설명

- `elapsed_secs`로 반복 위상을 만들고 sin으로 부드러운 값을 얻습니다.
- abs(sin)은 양수인 발걸음 높이로 사용합니다.
- MotionAmount가 0이면 모델이 기본 자세로 돌아갑니다.
- 루트 Transform을 흔들지 않으므로 Collider와 카메라는 안정적입니다.
- 시각 전용 System은 물리 결과에 영향을 주지 않습니다.

## 실습 과제

1. 흔들림 주파수와 높이를 바꾸세요.
2. 달릴 때 주파수가 증가하게 하세요.
3. 별도 팔·다리 자식 Mesh를 만들고 반대 위상으로 회전하세요.

## 심화 과제

무료 glTF 캐릭터와 Idle/Walk 클립을 추가하고 AnimationGraph에서 0.2초 동안 전환 혼합하세요. 에셋 라이선스도 문서에 기록하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part5/33_animation.md)

## 다음 챕터

다음 장에서는 지금 만든 절차적 흔들림을 실제 glTF 캐릭터의 Skin·Survey·Walk·Run 애니메이션으로 교체합니다.
