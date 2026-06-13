param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$')]
    [string] $Version,

    [string] $OutputPath
)

$ErrorActionPreference = "Stop"

$repoRoot = (git rev-parse --show-toplevel).Trim()
$changelog = Join-Path $repoRoot "CHANGELOG.md"

if (-not (Test-Path $changelog)) {
    throw "CHANGELOG.md not found"
}

$text = Get-Content $changelog -Raw
$escapedVersion = [regex]::Escape($Version)
$pattern = "(?ms)^## \[$escapedVersion\][^\r\n]*\r?\n(?<body>.*?)(?=^## \[|\z)"
$match = [regex]::Match($text, $pattern)

if (-not $match.Success) {
    throw "No changelog section found for version $Version"
}

$notes = $match.Groups["body"].Value.Trim()
if (-not $notes) {
    $notes = "Release $Version"
}

if ($OutputPath) {
    Set-Content -Path $OutputPath -Value $notes -NoNewline
} else {
    Write-Output $notes
}
