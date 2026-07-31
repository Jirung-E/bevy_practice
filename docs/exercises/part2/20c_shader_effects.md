# 20C. Space Survivor 셰이더 효과 과제 해설

[본문으로 돌아가기](../../20C_SpaceSurvivorShaderEffects.md#실습-과제)

## P2-C20C-P1 · dissolve 시간

`Dissolving` Timer의 지속 시간을 바꿉니다. WGSL progress 계산은 수정하지 않습니다. 같은 0..1 범위가 더 짧거나 긴 실제 시간에 걸쳐 전달되는지 확인하세요.

## P2-C20C-P2 · 발광 경계 폭

적 Material을 만들 때 `effect.z` 값을 변경합니다.

```rust
effect: Vec4::new(time.elapsed_secs(), 0.0, 0.03, 0.0),
```

값이 작으면 noise 임계값에 가까운 좁은 영역만 발광합니다. 지나치게 크면 남아 있는 몸체 대부분이 경계색으로 보일 수 있습니다.

## P2-C20C-P3 · 실드 파동 횟수

vertex shader의 `angle * 8.0`, `angle * 12.0`은 원 둘레에 나타나는 파동 반복 횟수입니다. 진폭과 반복 횟수를 한꺼번에 바꾸지 말고 한 값씩 비교하세요.

## P2-C20C-P4 · Timer 재시작

`H`를 빠르게 여러 번 누르면 `ShieldPulse` Timer가 `reset`됩니다. 상태 표시의 `IMPACT`가 유지되고 충격파가 안쪽에서 다시 시작해야 합니다. 새 실드 Entity가 생성되면 안 됩니다.

## P2-C20C-A1 · 사망 원인과 효과 설정

게임 규칙은 다음처럼 사망 원인만 기록합니다.

```rust
enum DefeatEffect {
    Burn,
    Freeze,
}
```

presentation System이 이 값을 읽어 Material 종류와 uniform을 선택하게 구성하세요. 점수 계산이나 충돌 판정이 WGSL 파일 경로를 알아서는 안 됩니다.

전체 코드:

- `examples/part2/space_survivor/src/bin/20c_shader_effects.rs`
- `examples/part2/space_survivor/assets/shaders/20c_dissolve.wgsl`
- `examples/part2/space_survivor/assets/shaders/20c_shield.wgsl`
