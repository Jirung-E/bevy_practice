# 18. 사운드 과제 해설

[본문으로 돌아가기](../../18_Sound.md#실습-과제)

## P2-C18-P1 · 주파수 비교

효과음 생성 함수의 주파수만 각각 `440.0`, `1200.0`으로 바꾸고 다른 조건은 유지해 비교합니다. 한 번에 여러 변수를 바꾸면 무엇이 소리 차이를 만들었는지 알기 어렵습니다.

## P2-C18-P2 · 길이와 볼륨

재생 길이는 생성하는 샘플 수, 볼륨은 샘플 진폭 또는 `PlaybackSettings`의 Volume으로 조절합니다. 진폭을 크게 만들 때는 출력 범위를 넘지 않게 제한합니다.

## P2-C18-P3 · 피격음 Handle

낮은 주파수의 `AudioSource`를 한 번 생성해 Handle로 보관하고 피격 Message를 받은 System에서 복제해 재생합니다. Handle 복제는 오디오 버퍼 전체 복사가 아닙니다.

## P2-C18-A1 · BGM과 효과음 설정 분리

라이선스를 확인한 `background.ogg`를 `assets/audio`에 두었다고 가정하면 반복 재생의 핵심은 다음과 같습니다.

```rust
commands.spawn((
    AudioPlayer::new(asset_server.load("audio/background.ogg")),
    PlaybackSettings::LOOP.with_volume(Volume::Linear(settings.music_volume)),
));
```

`AudioSettings`에는 `music_volume`과 `effects_volume`을 별도 필드로 둡니다. 예제는 두 값을 독립적으로 0~1 범위에 제한하는 순수 로직을 테스트합니다.

### 선택 기준

- 음악과 효과음은 사용자가 서로 다르게 조절하는 경우가 많으므로 한 필드로 합치지 않습니다.
- 실제 OGG 파일은 저장소에 포함하기 전에 배포 라이선스와 출처 표기 조건을 확인합니다.
- 많은 소리를 그룹 단위로 조절해야 한다면 Audio Entity 또는 별도 오디오 플러그인의 채널 기능을 검토합니다.

## 전체 코드 실행

```bash
cargo run -p space_survivor --bin game_flow_solution
cargo test -p space_survivor --bin game_flow_solution
```

전체 코드: `examples/part2/space_survivor/src/bin/game_flow_solution.rs`
