$ErrorActionPreference = "Stop"

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$docsRoot = Join-Path $repositoryRoot "docs"
$coveragePath = Join-Path $docsRoot "exercises\coverage.md"
$practiceHeading = "## " + (([char[]]@(0xC2E4, 0xC2B5, 0x20, 0xACFC, 0xC81C)) -join "")
$advancedHeading = "## " + (([char[]]@(0xC2EC, 0xD654, 0x20, 0xACFC, 0xC81C)) -join "")
$pendingText = (([char[]]@(0xC608, 0xC815)) -join "")

$chapters = Get-ChildItem -LiteralPath $docsRoot -File |
    Where-Object { $_.Name -match '^\d+[A-Z]?_.+\.md$' } |
    Sort-Object Name

$errors = [System.Collections.Generic.List[string]]::new()

foreach ($chapter in $chapters) {
    $content = Get-Content -LiteralPath $chapter.FullName -Raw -Encoding utf8
    if ($content -notmatch "(?m)^$([regex]::Escape($practiceHeading))\s*$") {
        $errors.Add("$($chapter.Name): practice heading missing")
    }
    if ($content -notmatch "(?m)^$([regex]::Escape($advancedHeading))\s*$") {
        $errors.Add("$($chapter.Name): advanced heading missing")
    }
    if ($content -notmatch [regex]::Escape("](exercises/")) {
        $errors.Add("$($chapter.Name): optional solution link missing")
    }
}

$coverage = Get-Content -LiteralPath $coveragePath -Raw -Encoding utf8
if ($coverage.Contains($pendingText)) {
    $errors.Add("coverage.md: pending row remains")
}
foreach ($chapter in $chapters) {
    $chapterNumber = [regex]::Match($chapter.Name, '^(\d+[A-Z]?)_').Groups[1].Value
    if ($coverage -notmatch "\|\s*$([regex]::Escape($chapterNumber))\.") {
        $errors.Add("$($chapter.Name): coverage row missing")
    }
}

if ($errors.Count -gt 0) {
    $errors | ForEach-Object { Write-Error $_ }
    exit 1
}

Write-Host "Exercise coverage structure OK: $($chapters.Count) chapters"
