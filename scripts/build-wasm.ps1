param(
    [string]$OutputDirectory = "target/dist/wasm/hello_bevy"
)

$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$output = if ([System.IO.Path]::IsPathRooted($OutputDirectory)) {
    [System.IO.Path]::GetFullPath($OutputDirectory)
} else {
    [System.IO.Path]::GetFullPath((Join-Path $workspace $OutputDirectory))
}
$wasm = Join-Path $workspace "target/wasm32-unknown-unknown/release/hello_bevy.wasm"
$template = Join-Path $workspace "web/hello_bevy/index.html"

if (-not (rustup target list --installed | Select-String -SimpleMatch "wasm32-unknown-unknown")) {
    throw "wasm32-unknown-unknown target이 없습니다. rustup target add wasm32-unknown-unknown을 먼저 실행하세요."
}
if (-not (Get-Command wasm-bindgen -ErrorAction SilentlyContinue)) {
    throw "wasm-bindgen CLI가 없습니다. cargo install wasm-bindgen-cli --version 0.2.126 --locked를 먼저 실행하세요."
}

Push-Location $workspace
try {
    cargo build --release --target wasm32-unknown-unknown -p hello_bevy --bin hello_bevy
    if ($LASTEXITCODE -ne 0) {
        throw "WASM cargo build가 실패했습니다."
    }

    New-Item -ItemType Directory -Force -Path $output | Out-Null
    wasm-bindgen `
        --target web `
        --no-typescript `
        --out-dir $output `
        --out-name hello_bevy `
        $wasm
    if ($LASTEXITCODE -ne 0) {
        throw "wasm-bindgen 패키징이 실패했습니다."
    }
    Copy-Item -LiteralPath $template -Destination (Join-Path $output "index.html") -Force
} finally {
    Pop-Location
}

$files = Get-ChildItem -LiteralPath $output -File
$size = ($files | Measure-Object -Property Length -Sum).Sum
Write-Output "WASM package: $output"
Write-Output "Files: $($files.Count)"
Write-Output "Bytes: $size"
