# 20D. Shader Hot Reload 과제 해설

[본문으로 돌아가기](../../20D_ShaderHotReload.md#실습-과제)

## P2-C20D-P1 · 별 색상

`20b_starfield.wgsl`의 최종 합성에서 가까운 별에 곱하는 색만 변경합니다. 함수 signature와 binding은 그대로 두므로 Rust 재컴파일 없이 반영되어야 합니다.

## P2-C20D-P2 · dissolve 경계

`20c_dissolve.wgsl`에서 `glowing_edge` 색과 뒤의 밝기 배율을 한 번에 바꾸지 말고 각각 저장해 영향을 구분하세요. 새로 맞은 적뿐 아니라 이미 dissolve 중인 적의 다음 프레임에도 같은 shader가 사용됩니다.

## P2-C20D-P3 · 실드 진폭

`20c_shield.wgsl`의 `effect.y * 0.09`에서 마지막 값을 변경합니다. fragment impact ring은 그대로 유지되므로 geometry 변형만 비교할 수 있습니다.

## P2-C20D-P4 · 문법 오류와 복구

의도적으로 세미콜론 하나를 제거하고 다음을 기록하세요.

- 로그에 표시된 WGSL 파일
- line과 column
- 예상한 token
- 복구 후 앱 재시작 필요 여부

오류가 발생한 상태에서 저장을 반복하기보다 먼저 첫 번째 parse 오류를 복구하세요.

## P2-C20D-A1 · 개발 상태 Plugin

`AssetEvent<Shader>`와 `AssetLoadFailedEvent<Shader>`를 읽을 수 있지만 pipeline 컴파일 성공까지 보장하지 않는다는 제한을 UI 문구에 반영해야 합니다.

적절한 상태 예:

- `CHANGE DETECTED`
- `SHADER ASSET LOADED`
- `FILE LOAD FAILED`
- `CHECK RENDER LOG`

부적절한 상태 예:

- `GPU COMPILE SUCCESS`

실행:

```bash
cargo run -p space_survivor --bin shader_effects
```
