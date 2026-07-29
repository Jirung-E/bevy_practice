param(
    [string]$Package = "space_survivor",
    [string]$Binary = "20_game_over",
    [string]$AssetsDirectory = "examples/part2/space_survivor/assets",
    [string]$OutputDirectory = "target/dist/windows/space_survivor"
)

$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$output = [System.IO.Path]::GetFullPath((Join-Path $workspace $OutputDirectory))
$executable = Join-Path $workspace "target/release/$Binary.exe"
$assets = [System.IO.Path]::GetFullPath((Join-Path $workspace $AssetsDirectory))
$notices = Join-Path $workspace "THIRD_PARTY_LICENSES.md"

Push-Location $workspace
try {
    cargo build --release -p $Package --bin $Binary
    if ($LASTEXITCODE -ne 0) {
        throw "release build가 실패했습니다."
    }

    New-Item -ItemType Directory -Force -Path $output | Out-Null
    Copy-Item -LiteralPath $executable -Destination $output -Force
    if (Test-Path -LiteralPath $assets) {
        Copy-Item -LiteralPath $assets -Destination (Join-Path $output "assets") -Recurse -Force
    }
    if (Test-Path -LiteralPath $notices) {
        Copy-Item -LiteralPath $notices -Destination $output -Force
    }
} finally {
    Pop-Location
}

$files = Get-ChildItem -LiteralPath $output -File -Recurse
$size = ($files | Measure-Object -Property Length -Sum).Sum
Write-Output "Desktop package: $output"
Write-Output "Files: $($files.Count)"
Write-Output "Bytes: $size"
Write-Output "Clean-machine check: copy this directory only, then run $Binary.exe"
