param(
    [double]$SecondsPerExample = 3.0
)

$ErrorActionPreference = "Stop"
$workspace = Split-Path -Parent $PSScriptRoot
$logDirectory = Join-Path $workspace "target\smoke-logs"
New-Item -ItemType Directory -Force -Path $logDirectory | Out-Null

$metadata = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$binaries = $metadata.packages |
    ForEach-Object {
        $package = $_
        $_.targets |
            Where-Object { $_.kind -contains "bin" } |
            ForEach-Object {
                [pscustomobject]@{
                    Package = $package.name
                    Name = $_.name
                }
            }
    } |
    Sort-Object Package, Name

$issues = [System.Collections.Generic.List[object]]::new()
$index = 0

foreach ($binary in $binaries) {
    $index++
    $executable = Join-Path $workspace "target\debug\$($binary.Name).exe"
    $stdout = Join-Path $logDirectory "$($binary.Name).stdout.log"
    $stderr = Join-Path $logDirectory "$($binary.Name).stderr.log"

    if (-not (Test-Path -LiteralPath $executable)) {
        $issues.Add([pscustomobject]@{
            Example = $binary.Name
            Detail = "실행 파일이 없습니다."
        })
        Write-Output "[$index/$($binaries.Count)] MISSING $($binary.Name)"
        continue
    }

    $process = Start-Process `
        -FilePath $executable `
        -WorkingDirectory $workspace `
        -WindowStyle Hidden `
        -RedirectStandardOutput $stdout `
        -RedirectStandardError $stderr `
        -PassThru

    $exited = $process.WaitForExit([int]($SecondsPerExample * 1000))
    if (-not $exited) {
        Stop-Process -Id $process.Id -Force
        $process.WaitForExit()
    }
    $process.Refresh()

    $output = @(
        if (Test-Path -LiteralPath $stdout) {
            Get-Content -Raw -Encoding utf8 $stdout
        }
        if (Test-Path -LiteralPath $stderr) {
            Get-Content -Raw -Encoding utf8 $stderr
        }
    ) -join "`n"

    $problemLines = $output -split "\r?\n" |
        Where-Object {
            $_ -match "(?i)\bWARN\b|\bERROR\b|panicked at|fatal error|stack backtrace"
        }

    if ($exited -and $null -ne $process.ExitCode -and $process.ExitCode -ne 0) {
        $issues.Add([pscustomobject]@{
            Example = $binary.Name
            Detail = "비정상 종료 코드 $($process.ExitCode)"
        })
    }
    foreach ($line in $problemLines) {
        $issues.Add([pscustomobject]@{
            Example = $binary.Name
            Detail = $line.Trim()
        })
    }

    $status = if (
        $problemLines -or
        ($exited -and $null -ne $process.ExitCode -and $process.ExitCode -ne 0)
    ) {
        "ISSUE"
    } else {
        "OK"
    }
    Write-Output "[$index/$($binaries.Count)] $status $($binary.Name)"
}

Write-Output ""
Write-Output "Examples=$($binaries.Count)"
Write-Output "Issues=$($issues.Count)"

if ($issues.Count -gt 0) {
    $issues | Format-Table -AutoSize | Out-String | Write-Output
    exit 1
}
