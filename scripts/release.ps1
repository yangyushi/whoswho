param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$')]
    [string] $Version,

    [switch] $NoCommit,
    [switch] $NoTag,
    [switch] $Push,
    [switch] $AllowNonMain
)

$ErrorActionPreference = "Stop"

function Fail($Message) {
    Write-Error $Message
    exit 1
}

function Require-Command($Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        Fail "Required command not found: $Name"
    }
}

function Run-Native($Command, [string[]] $Arguments) {
    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        Fail "Command failed: $Command $($Arguments -join ' ')"
    }
}

Require-Command git
Require-Command cargo

$repoRoot = (git rev-parse --show-toplevel).Trim()
if ($LASTEXITCODE -ne 0) {
    Fail "Not inside a git repository."
}
Set-Location $repoRoot

$branch = (git branch --show-current).Trim()
if (-not $AllowNonMain -and $branch -ne "main") {
    Fail "Release must run on main. Current branch: $branch. Use -AllowNonMain to override."
}

$dirty = git status --porcelain
if ($dirty) {
    Fail "Working tree must be clean before preparing a release."
}

$tag = "v$Version"
git rev-parse -q --verify "refs/tags/$tag" *> $null
if ($LASTEXITCODE -eq 0) {
    Fail "Tag already exists: $tag"
}

$cargoToml = Join-Path $repoRoot "Cargo.toml"
$cargoText = Get-Content $cargoToml -Raw
if ($cargoText -notmatch '(?m)^version = "([^"]+)"') {
    Fail "Could not find package version in Cargo.toml"
}

$currentVersion = $Matches[1]
if ($currentVersion -eq $Version) {
    Fail "Cargo.toml is already at version $Version"
}

Write-Host "Preparing release $currentVersion -> $Version"

$cargoText = $cargoText -replace '(?m)^version = "[^"]+"', "version = `"$Version`""
Set-Content -Path $cargoToml -Value $cargoText -NoNewline

Run-Native cargo @("check")

$changelog = Join-Path $repoRoot "CHANGELOG.md"
if (-not (Test-Path $changelog)) {
    Fail "CHANGELOG.md not found"
}

$today = Get-Date -Format "yyyy-MM-dd"
$changelogText = Get-Content $changelog -Raw
if ($changelogText -notmatch '(?m)^## \[Unreleased\]\s*$') {
    Fail "CHANGELOG.md must contain '## [Unreleased]'"
}

$releasedHeading = "## [Unreleased]`r`n`r`n## [$Version] - $today"
$changelogText = $changelogText -replace '(?m)^## \[Unreleased\]\s*$', $releasedHeading
Set-Content -Path $changelog -Value $changelogText -NoNewline

Run-Native cargo @("fmt", "--check")
Run-Native cargo @("test")
Run-Native git @("diff", "--check")

if (-not $NoCommit) {
    Run-Native git @("add", "Cargo.toml", "Cargo.lock", "CHANGELOG.md")
    Run-Native git @("commit", "-m", "release $Version")
}

if (-not $NoTag) {
    Run-Native git @("tag", "-a", $tag, "-m", $tag)
}

if ($Push) {
    Run-Native git @("push", "origin", $branch)
    if (-not $NoTag) {
        Run-Native git @("push", "origin", $tag)
    }
}

Write-Host "Release prepared: $tag"
if (-not $Push) {
    Write-Host "Push with: git push origin $branch $tag"
}
