$ErrorActionPreference = "Stop"

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$docsRoot = Join-Path $repositoryRoot "docs"
$coveragePath = Join-Path $docsRoot "exercises\coverage.md"
$learningHeading = "## " + (([char[]]@(0xD559, 0xC2B5, 0x20, 0xBAA9, 0xD45C)) -join "")
$applicationHeading = "## " + (([char[]]@(0xC774, 0x20, 0xB0B4, 0xC6A9, 0xC73C, 0xB85C, 0x20, 0xB9CC, 0xB4E4, 0x20, 0xC218, 0x20, 0xC788, 0xB294, 0x20, 0xAC83)) -join "")
$resultHeading = "## " + (([char[]]@(0xC774, 0xBC88, 0xC5D0, 0x20, 0xB9CC, 0xB4E4, 0x20, 0xACB0, 0xACFC, 0xBB3C)) -join "")
$practiceHeading = "## " + (([char[]]@(0xC2E4, 0xC2B5, 0x20, 0xACFC, 0xC81C)) -join "")
$advancedHeading = "## " + (([char[]]@(0xC2EC, 0xD654, 0x20, 0xACFC, 0xC81C)) -join "")
$pendingText = (([char[]]@(0xC608, 0xC815)) -join "")

$chapters = Get-ChildItem -LiteralPath $docsRoot -File |
    Where-Object { $_.Name -match '^\d+[A-Z]?_.+\.md$' } |
    Sort-Object Name

$errors = [System.Collections.Generic.List[string]]::new()

foreach ($chapter in $chapters) {
    $content = Get-Content -LiteralPath $chapter.FullName -Raw -Encoding utf8
    $learningIndex = $content.IndexOf($learningHeading)
    $applicationIndex = $content.IndexOf($applicationHeading)
    $resultIndex = $content.IndexOf($resultHeading)

    if ($learningIndex -lt 0) {
        $errors.Add("$($chapter.Name): learning heading missing")
    }
    if ($applicationIndex -lt 0) {
        $errors.Add("$($chapter.Name): application heading missing")
    }
    elseif ($content.LastIndexOf($applicationHeading) -ne $applicationIndex) {
        $errors.Add("$($chapter.Name): application heading duplicated")
    }
    if ($resultIndex -lt 0) {
        $errors.Add("$($chapter.Name): result heading missing")
    }
    if (
        $learningIndex -ge 0 -and
        $applicationIndex -ge 0 -and
        $resultIndex -ge 0 -and
        -not ($learningIndex -lt $applicationIndex -and $applicationIndex -lt $resultIndex)
    ) {
        $errors.Add("$($chapter.Name): learning/application/result heading order invalid")
    }
    if ($applicationIndex -ge 0 -and $resultIndex -gt $applicationIndex) {
        $applicationBodyStart = $applicationIndex + $applicationHeading.Length
        $applicationBody = $content.Substring(
            $applicationBodyStart,
            $resultIndex - $applicationBodyStart
        )
        $applicationCount = (
            [regex]::Matches($applicationBody, "(?m)^-\s+\S")
        ).Count
        if ($applicationCount -lt 2 -or $applicationCount -gt 4) {
            $errors.Add(
                "$($chapter.Name): application examples must contain 2-4 bullets (found $applicationCount)"
            )
        }
    }
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

$markdownFiles = Get-ChildItem -LiteralPath $docsRoot -Recurse -File -Filter "*.md"
foreach ($markdownFile in $markdownFiles) {
    $content = Get-Content -LiteralPath $markdownFile.FullName -Raw -Encoding utf8
    foreach ($linkMatch in [regex]::Matches($content, "\]\((?<target>[^)]+)\)")) {
        $target = $linkMatch.Groups["target"].Value.Trim().Trim("<", ">")
        if ($target -match "^\S+\s+[`"']") {
            $target = ($target -split "\s+", 2)[0]
        }
        if (
            $target -match "^(https?://|mailto:|#|javascript:)" -or
            $target.StartsWith("/")
        ) {
            continue
        }

        $target = ($target -split "[#?]", 2)[0]
        if ([string]::IsNullOrWhiteSpace($target)) {
            continue
        }

        $decodedTarget = [Uri]::UnescapeDataString($target)
        $resolvedTarget = [IO.Path]::GetFullPath(
            (Join-Path $markdownFile.DirectoryName $decodedTarget)
        )
        if (-not (Test-Path -LiteralPath $resolvedTarget)) {
            $relativeMarkdownPath = [IO.Path]::GetRelativePath(
                $repositoryRoot,
                $markdownFile.FullName
            )
            $errors.Add(
                "${relativeMarkdownPath}: local link target missing ($target)"
            )
        }
    }
}

if ($errors.Count -gt 0) {
    $errors | ForEach-Object { Write-Error $_ }
    exit 1
}

Write-Host "Exercise coverage structure OK: $($chapters.Count) chapters"
