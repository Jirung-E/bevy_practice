# 40B. Entity에 스크립트 연결하고 Hot Reload하기

## 학습 목표

- Bevy에서 스크립트 연결이 어떤 계층으로 구성되는지 설명합니다.
- 사용자 정의 Asset Loader로 RON 스크립트를 읽습니다.
- Script Handle을 Component로 Entity에 연결하고 System에서 실행합니다.
- 파일 변경으로 교체된 Asset을 실행 중 반영합니다.

## 이 내용으로 만들 수 있는 것

- 코드를 다시 컴파일하지 않고 조절하는 오브젝트 동작
- 에디터 Inspector에서 Entity별로 연결하는 행동 파일
- 대화, 컷신, 퀘스트와 간단한 레벨 이벤트 명령

## 이번에 만들 결과물

Cube Entity에 `spin_and_bob.editor_script`를 연결합니다. 스크립트의 회전 속도, 상하 이동 높이와 색을 바꾸고 저장하면 실행 중인 장면에 다시 반영됩니다.

```bash
cargo run -p world_editor --bin script_attachment
```

실행한 상태에서 다음 파일을 수정합니다.

```text
examples/part6/world_editor/assets/scripts/spin_and_bob.editor_script
```

## 핵심 개념

Bevy에는 특정 스크립트 언어가 내장되어 있지 않습니다. 프로젝트는 Lua·Rhai 같은 VM을 붙일 수도 있고, 이 장처럼 허용된 명령만 역직렬화하는 선언형 스크립트를 만들 수도 있습니다. 중요한 ECS 연결 경계는 어느 방식을 선택해도 같습니다.

```text
스크립트 파일
  → AssetLoader
  → Assets<EditorScript>
  → Handle<EditorScript>를 가진 AttachedScript Component
  → 실행 System
  → Transform / Material 등 허용된 Component 변경
```

이 예제의 스크립트는 Rust 코드를 임의 실행하지 않습니다. `RotateY`, `Bob`, `Tint`처럼 엔진이 명시적으로 허용한 명령만 실행하므로 입력 검증과 권한 범위가 분명합니다.

### Hot Reload 흐름

`file_watcher` 기능과 `AssetPlugin::watch_for_changes_override`를 켜면 Asset Server가 원본 파일 변경을 감시합니다. 파일이 바뀌면 같은 Handle이 새 Asset 값을 가리키고 `AssetEvent::Modified`가 전달됩니다. 실행 System은 매 프레임 Handle로 현재 Asset을 조회하므로 Entity에 다시 연결할 필요가 없습니다.

Rust 소스 변경은 Asset Hot Reload가 아닙니다. Rust 코드는 다시 컴파일해야 합니다. 셰이더와 이 장의 스크립트는 Asset이므로 파일 감시 경로를 사용할 수 있습니다.

| 변경 대상 | 일반적인 반영 방식 |
|---|---|
| Rust System | 재컴파일·프로세스 재시작 또는 별도 dynamic linking 도구 |
| WGSL Shader Asset | Asset watcher가 다시 로드하고 렌더 파이프라인 재구성 |
| RON EditorScript | Asset watcher가 다시 로드하고 다음 Update부터 새 명령 실행 |

## 샘플 코드

```rust
#[derive(Component)]
struct AttachedScript(Handle<EditorScript>);

for (attached, mut transform) in &mut entities {
    let Some(script) = scripts.get(&attached.0) else { continue };
    for command in &script.commands {
        execute(command, &mut transform);
    }
}
```

전체 Rust 코드는 [40b_script_attachment.rs](source/part6.md#40b--스크립트-연결과-hot-reload), 스크립트 원본은 같은 전체 코드 페이지에서 확인할 수 있습니다.

## 코드 설명

- `EditorScriptLoader`가 `.editor_script` bytes를 RON 구조체로 변환합니다.
- `AttachedScript`는 Script 복사본이 아니라 Handle을 보관하므로 여러 Entity가 같은 Asset을 공유할 수 있습니다.
- `ScriptOrigin`은 Bob 명령의 기준 위치를 보관해 매 프레임 현재 위치에 오차를 누적하지 않습니다.
- Loader는 RON 오류를 `InvalidData` 오류로 돌려 앱 panic 대신 Asset 로드 실패로 보고합니다.
- 스크립트에서 ECS `World` 전체를 직접 노출하지 않고 실행 System이 허용한 데이터만 변경합니다.
- 배포 빌드에서 사용자가 원본 Asset을 수정하게 할지, 패키지 내부 Asset 감시를 끌지는 제품 정책으로 결정해야 합니다.

## 실습 과제

1. RON 파일의 `degrees_per_second`를 바꾸고 재실행 없이 속도가 달라지는지 확인하세요.
2. `ScalePulse` 명령을 추가해 원래 크기를 기준으로 확대·축소하세요.
3. 서로 다른 Script Handle을 가진 Cube와 Sphere를 한 장면에 배치하세요.

## 심화 과제

에디터 Inspector에서 Script Asset 경로를 선택해 `AttachedScript`를 추가·교체·제거하는 UI를 만드세요. 로드 실패 시 기존 Script를 유지하고 Console에 파일 경로와 파싱 위치를 표시하세요.

[선택한 과제 해설과 수행 예시 보기](exercises/part6/40b_script_attachment.md)

## 다음 챕터

Part 7에서는 완성한 기능을 Plugin과 모듈 경계로 나누고 실제 프로젝트 구조로 발전시킵니다.
