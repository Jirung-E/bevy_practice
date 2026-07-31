# 20B. 절차적 우주 배경 과제 해설

[본문으로 돌아가기](../../20B_ProceduralBackground.md#실습-과제)

## P2-C20B-P1 · 반대 방향 레이어

두 번째 `star_layer` 호출의 `drift`만 음수로 바꾸세요. Rust의 시간과 속도 uniform은 그대로 유지합니다. 두 레이어가 같은 시간 값을 읽더라도 레이어 상수가 부호를 바꾸므로 반대 방향으로 흐릅니다.

## P2-C20B-P2 · 격자 크기

`grid_size`가 커지면 화면에 더 많은 cell이 생기지만 cell 하나의 화면 크기는 작아집니다. `radius`는 cell 내부 좌표 기준이므로 별도 함께 작아집니다. 밀도와 격자 크기는 서로 다른 설정이라는 점을 비교하세요.

## P2-C20B-P3 · 반짝임 제거

다음 값으로 고정해 시간에 따른 밝기 변화만 제거합니다.

```wgsl
let twinkle = 1.0;
```

별의 이동은 UV 시간 오프셋이 담당하므로 계속 유지되어야 합니다.

## P2-C20B-P4 · 세 번째 레이어

새 레이어는 기존 레이어 호출을 복사한 뒤 적어도 다음 세 값을 다르게 설정하세요.

- `grid_size`
- `drift`
- 최종 합성 색상

UV 오프셋도 다르게 주면 서로 다른 cell seed를 사용하므로 별 위치가 겹치는 현상을 줄일 수 있습니다.

## P2-C20B-A1 · 마우스 왜곡

윈도 좌표를 바로 WGSL에 보내지 말고 카메라 기준 월드 좌표 또는 0..1 UV로 변환해야 합니다. fragment에서 현재 UV와 마우스 UV 사이의 방향·거리를 계산하고, 일정 반경 안에서만 원래 UV를 밀어낸 뒤 `star_layer`에 전달하세요.

전체 코드:

- `examples/part2/space_survivor/src/bin/20b_procedural_background.rs`
- `examples/part2/space_survivor/assets/shaders/20b_starfield.wgsl`
