# 46. 데스크톱과 WASM 배포 과제 해설

[본문으로 돌아가기](../../46_DesktopWasmDeployment.md#실습-과제)

## P7-C46-P1 · 깨끗한 Windows 배포 검사

저장소의 `target/release`에서 직접 실행하지 말고 `package-desktop.ps1`의 출력 폴더만 새 위치로 복사합니다. 실행 여부뿐 아니라 터미널의 `WARN`·`ERROR`, 자산 로드 실패, 파일 수와 크기를 함께 기록해야 재검사가 가능합니다.

## P7-C46-P2 · WASM 네트워크 검사

`file://`가 아니라 로컬 HTTP 서버를 사용합니다. 개발자 도구에서 다음을 확인합니다.

- `hello_bevy_bg.wasm` 응답 상태가 200
- Content-Type이 `application/wasm`
- Console에 panic, 경고, 오류가 없음
- 로딩 문구가 사라지고 canvas가 생성됨

## P7-C46-P3 · 대소문자 오류

Windows 파일 시스템에서는 지나갈 수 있는 실수를 Pages의 Linux 빌드와 웹 서버가 드러냅니다. 코드와 실제 파일명 중 하나만 바꾸어 404를 재현하고, 둘의 철자를 정확히 맞춰 복구합니다.

## P7-C46-A1 · CI 배포 검증

배포 작업과 검증 작업을 분리합니다. 검증 단계는 최소한 다음 조건을 자동 검사합니다.

- Windows: `.exe`, `assets`, `THIRD_PARTY_LICENSES.md`
- WASM: `index.html`, `.js`, `.wasm`
- 모든 필수 파일의 크기가 0보다 큼

크기 제한은 즉시 실패시키기보다 기준값과 빌드 환경을 함께 기록한 뒤 경고 임계값부터 운영하는 편이 안전합니다. 웹 전송량은 압축 전 WASM 파일 크기와 별도 지표로 관리합니다.

## 관련 스크립트

- `scripts/package-desktop.ps1`
- `scripts/build-wasm.ps1`
- `scripts/serve-static.mjs`
- `.github/workflows/pages.yml`
