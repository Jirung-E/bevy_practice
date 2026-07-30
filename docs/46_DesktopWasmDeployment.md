# 46. 데스크톱과 WASM 빌드·배포

## 학습 목표

- 개발 빌드와 배포용 release 빌드의 차이를 설명할 수 있습니다.
- Windows 실행 파일과 자산을 하나의 배포 폴더로 묶을 수 있습니다.
- Bevy 애플리케이션을 WebAssembly로 빌드하고 로컬 브라우저에서 검사할 수 있습니다.
- GitHub Actions로 교재와 WASM 데모를 GitHub Pages에 함께 배포할 수 있습니다.
- 플랫폼별 저장 위치, 로그 확인법, 라이선스와 호환성 점검 항목을 구분할 수 있습니다.

## 이 내용으로 만들 수 있는 것

- 실행 파일과 에셋을 묶은 Windows 배포 폴더
- 브라우저에서 바로 체험하는 Bevy WASM 데모
- GitHub Actions와 Pages로 자동 배포되는 공개 포트폴리오

## 이번에 만들 결과물

이 챕터에서는 저장소에 포함된 배포 스크립트를 실행해 다음 두 결과물을 만듭니다.

- `target/dist/windows/space_survivor/`: 다른 폴더로 복사해 실행할 수 있는 Windows 배포본
- `target/dist/wasm/hello_bevy/`: HTTP 서버와 GitHub Pages에서 실행할 수 있는 브라우저 배포본

WASM 샘플의 Pages 배포 주소는 [Hello Bevy WASM 데모](https://jirung-e.github.io/bevy_practice/demos/hello_bevy/)입니다. 아직 워크플로를 배포하지 않았다면 404가 나올 수 있습니다.

## 핵심 개념

### release 프로필

루트 `Cargo.toml`은 배포 빌드에 다음 옵션을 사용합니다.

```toml
[profile.release]
lto = "thin"
codegen-units = 1
strip = "debuginfo"
```

`lto`와 `codegen-units`는 실행 성능과 파일 크기에 도움을 줄 수 있지만 빌드 시간이 길어집니다. `strip`은 디버그 정보를 제거하므로 배포 파일은 작아지지만, 상세한 크래시 분석이 필요하면 심볼을 별도로 보관해야 합니다.

### 실행 파일만 복사하면 부족한 이유

`cargo build --release`가 만든 `.exe`만 복사하면 런타임에 `assets/...`를 찾지 못할 수 있습니다. 실행 파일, 자산, 제3자 라이선스 고지를 같은 배포 폴더에 넣고 **저장소 밖의 깨끗한 위치에 그 폴더만 복사하여** 검사해야 누락을 발견할 수 있습니다.

Windows의 MSVC 타깃으로 빌드한 프로그램은 대상 PC의 Visual C++ 런타임 상태에도 영향을 받을 수 있습니다. 개발 PC에서 실행된다는 사실만으로 배포 검사가 끝난 것은 아닙니다.

### WASM은 웹 서버가 필요합니다

브라우저는 `file://`로 연 WASM 모듈을 정상적으로 불러오지 못할 수 있습니다. `.wasm`을 `application/wasm` MIME 타입으로 제공하는 HTTP 서버에서 확인합니다. 운영 배포에서는 HTTPS를 사용해야 일부 브라우저 API도 안정적으로 사용할 수 있습니다.

데스크톱의 일반 파일 시스템, 스레드, 파일 감시 API는 웹에서 그대로 사용할 수 없습니다. 플랫폼 차이가 있는 코드는 다음처럼 분리합니다.

```rust
#[cfg(target_arch = "wasm32")]
fn platform_name() -> &'static str {
    "web"
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_name() -> &'static str {
    "desktop"
}
```

### 로그와 저장 위치

- Windows: 터미널 로그 또는 파일 로그를 사용하고, 저장 데이터는 실행 파일 옆보다 운영체제의 사용자 데이터 디렉터리에 둡니다.
- 브라우저: 개발자 도구의 Console에서 오류를 확인합니다. 저장은 `localStorage`, IndexedDB 또는 다운로드 기능을 사용하며 임의의 데스크톱 경로에는 쓸 수 없습니다.

웹 서버는 경로의 대소문자를 구분할 수 있습니다. Windows에서 우연히 로드된 `Player.PNG`가 Pages의 `player.png` 요청에서는 실패할 수 있으므로 자산 경로의 철자를 정확히 맞춥니다.

## 샘플 코드

### Windows 패키지 만들기

PowerShell에서 다음 저장소 샘플 스크립트를 실행합니다.

```powershell
.\scripts\package-desktop.ps1
```

완료 후 `target/dist/windows/space_survivor/20_game_over.exe`를 실행합니다. 실제 배포 전에는 출력 폴더만 다른 PC나 임시 폴더로 복사해 실행하고, 자산 누락 및 경고·오류가 없는지 확인합니다.

다른 패키지를 묶을 때는 매개변수를 지정할 수 있습니다.

```powershell
.\scripts\package-desktop.ps1 `
  -Package product_showcase `
  -Binary 30_light `
  -AssetsDirectory examples/part4/product_showcase/assets `
  -OutputDirectory target/dist/windows/product_showcase
```

### WASM 패키지 만들기

도구를 한 번 설치합니다.

```powershell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126 --locked
```

`wasm-bindgen-cli` 버전은 `Cargo.lock`의 `wasm-bindgen` 버전과 맞춰야 합니다. 그다음 저장소 샘플 스크립트를 실행합니다.

```powershell
.\scripts\build-wasm.ps1
node .\scripts\serve-static.mjs target/dist/wasm/hello_bevy 8000
```

브라우저에서 `http://127.0.0.1:8000/`을 열고 화면과 개발자 도구의 Console·Network 탭을 확인합니다. 이 교재의 최소 기능 Bevy 0.19 샘플은 `hello_bevy` 패키지에 필요한 2D 기능만 켜서 빌드합니다.

### GitHub Pages 자동 배포

`.github/workflows/pages.yml`은 다음 순서로 작업합니다.

1. mdBook HTML을 `book/`에 생성합니다.
2. `hello_bevy`를 WASM으로 빌드합니다.
3. 세 필수 파일을 `book/demos/hello_bevy/`에 넣고 존재 여부를 검사합니다.
4. 하나의 Pages artifact로 업로드하고 배포합니다.

따라서 프로젝트 저장소가 `bevy_practice`라면 데모 URL은 `/bevy_practice/demos/hello_bevy/`가 됩니다.

## 코드 설명

`scripts/package-desktop.ps1`은 release 실행 파일, 해당 프로젝트의 `assets`, `THIRD_PARTY_LICENSES.md`를 한 폴더로 복사합니다. 파일 수와 전체 바이트 수도 출력하므로 결과가 갑자기 비어 있거나 크게 달라졌는지 찾는 기준이 됩니다.

`scripts/build-wasm.ps1`은 `wasm32-unknown-unknown` 타깃으로 빌드한 뒤 `wasm-bindgen --target web`으로 JavaScript 접착 코드와 WASM 모듈을 생성하고 HTML을 복사합니다. 로컬 측정에서 압축 전 파일 합계는 약 47 MB였지만 도구 버전과 기능에 따라 달라집니다. 이 값은 서버가 gzip 또는 Brotli로 압축한 **네트워크 전송량**과 다르므로 Network 탭에서 둘을 구분해 확인합니다.

배포 전에는 다음 항목을 확인합니다.

- 새 폴더에서 실행 파일이 시작되고 자산 오류가 없는가
- 브라우저 Console에 panic, 경고, 404가 없는가
- Chrome, Firefox 등 지원할 브라우저에서 입력과 렌더링이 동작하는가
- 자산 파일명 대소문자와 상대 경로가 정확한가
- Rust crate, 모델, 이미지, 글꼴, 음원 라이선스가 배포를 허용하는가
- 필요한 저작권 및 라이선스 고지가 패키지에 포함됐는가

## 실습 과제

1. Windows 배포 스크립트를 실행하고 출력 폴더만 임시 위치로 복사해 게임을 실행하세요. 검사 결과에 실행 파일 크기, 자산 파일 수, 경고·오류 유무를 기록하세요.
2. WASM 샘플을 로컬 HTTP 서버로 열고 Console과 Network 탭에서 오류, 404, `.wasm` 응답의 MIME 타입을 확인하세요.
3. 이미지 경로의 대소문자를 일부러 틀려 웹에서 발생하는 404를 확인한 뒤 원래대로 복구하세요.

## 심화 과제

GitHub Actions의 배포 검사를 확장해 Windows 패키지의 실행 파일, `assets` 폴더, 라이선스 고지가 모두 존재하지 않으면 실패하도록 만드세요. 이어서 WASM 결과물의 압축 전 크기를 기록하고 기준보다 지나치게 커지면 경고하는 단계를 설계하세요.

[선택한 과제 해설과 수행 예시 보기](exercises/part7/46_deployment.md)

## 다음 챕터

전체 교재의 기본 과정은 여기서 끝납니다. 실제 프로젝트에서는 배포 대상 운영체제와 브라우저를 먼저 정하고, 지원 환경별 빌드·실행·저장·업데이트 검사를 CI와 출시 체크리스트에 추가하세요.
