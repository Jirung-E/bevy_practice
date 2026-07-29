# 18. 사운드

## 학습 목표

- Bevy의 AudioPlayer와 PlaybackSettings를 사용할 수 있다.
- Message를 사운드 재생 트리거로 활용할 수 있다.
- 커스텀 Decodable 에셋의 역할을 이해한다.

## 이번에 만들 결과물

적을 총알로 처치할 때마다 짧은 880Hz 효과음이 한 번 재생됩니다.

아래 명령은 이 교재 저장소에 포함된 완성 샘플을 실행합니다. 본문만 따라 만든 별도 프로젝트에서는 현재 프로젝트 구성에 맞춰 `cargo run`을 실행하세요.

```bash
cargo run -p space_survivor --bin 18_sound
```

## 핵심 개념

일반 프로젝트는 AssetServer로 WAV/OGG 파일을 로드합니다. 이 교재 예제는 저장소를 복제한 즉시 실행되도록 `BeepAudio` 에셋과 `BeepDecoder`를 구현해 사인파 샘플을 생성합니다.

`EnemyDefeated` Message를 읽는 점수 System에서 효과음을 생성하므로 충돌 System은 오디오를 알 필요가 없습니다.

## 샘플 코드

```rust
#[derive(Asset, TypePath)]
struct BeepAudio {
    frequency: f32,
    duration: Duration,
}

fn setup_audio(mut commands: Commands, mut assets: ResMut<Assets<BeepAudio>>) {
    let handle = assets.add(BeepAudio {
        frequency: 880.0,
        duration: Duration::from_millis(90),
    });
    commands.insert_resource(DefeatSound(handle));
}
```

재생:

```rust
commands.spawn((
    AudioPlayer(sound.0.clone()),
    PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.15)),
));
```

## 코드 설명

- `Asset`은 Bevy의 핸들 기반 에셋 저장소에 넣을 수 있게 합니다.
- `Decodable` 구현은 에셋을 실제 오디오 샘플 Iterator로 바꿉니다.
- 짧은 fade를 적용해 파형이 갑자기 끊길 때 생기는 클릭음을 줄입니다.
- `PlaybackSettings::DESPAWN`은 재생이 끝난 AudioPlayer Entity를 제거합니다.
- Handle 복제는 오디오 데이터를 복사하지 않고 에셋 참조만 늘립니다.

실제 배포 게임에서는 라이선스를 확인한 사운드 파일을 `assets/audio`에 두고 AssetServer로 로드하는 흐름이 일반적입니다.

## 실습 과제

1. 주파수를 440Hz와 1200Hz로 바꾸어 비교하세요.
2. 재생 시간과 볼륨을 조절하세요.
3. 플레이어 피격용 낮은 음을 별도 Handle로 추가하세요.

## 심화 과제

배경 음악용 OGG 파일을 추가하고 `PlaybackSettings::LOOP`로 반복 재생하세요. 음악과 효과음 볼륨을 별도 Resource로 관리하세요.

[선택형 과제 해설과 수행 예시 보기](exercises/part2/18_sound.md)

## 다음 챕터

게임을 종료해도 최고 점수가 유지되도록 파일에 저장하고 시작할 때 불러옵니다.
