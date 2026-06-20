# Releasing

`wsw` uses Semantic Versioning. `Cargo.toml` is the source of truth for the current package version, and GitHub Releases are created from annotated Git tags.

## Branch Policy

- `main` contains released code only.
- `dev` is the integration branch for reviewed changes before release.
- Day-to-day work should happen on short-lived branches created from `dev`.
- Use branch prefixes that describe the intent, such as `feature/`, `bugfix/`, and `refactor/`.
- Finished work should be rebased onto the latest `dev`, then proposed as a PR targeting `dev`.
- When enough changes have accumulated, merge `dev` into `main`, then prepare and publish the release from `main`.
- Tags are created only from `main`.

## Version Rules

- Patch: bug fixes and documentation-only changes.
- Minor: backward-compatible CLI features, output additions, and search behavior improvements.
- Major: incompatible CLI behavior, incompatible JSON output, or database migrations that require user action.

## Development Flow

1. Start from an up-to-date `dev`:

```powershell
git switch dev
git pull --ff-only origin dev
```

2. Create a short-lived work branch:

```powershell
git switch -c bugfix/describe-the-fix
```

Use `feature/`, `bugfix/`, or `refactor/` as the prefix.

3. Keep user-facing changes documented under `## [Unreleased]` in `CHANGELOG.md`.
4. Commit the work branch and run the relevant checks.
5. Rebase onto the latest `dev` before opening a PR:

```powershell
git fetch origin
git rebase origin/dev
```

6. Push the branch and open a PR targeting `dev`.
7. When preparing a release, make sure `dev` has all intended changes and passes tests.
8. Merge `dev` into `main`:

```powershell
git switch main
git pull --ff-only origin main
git merge --no-ff dev
```

## Normal Release Flow

1. Make sure all release-bound changes have already been merged into `main`.
2. Make sure the working tree is clean.
3. Run the release script:

```powershell
.\scripts\release.ps1 -Version 0.2.0
```

The script will:

- require a clean working tree on `main`;
- update `Cargo.toml`;
- move the `CHANGELOG.md` unreleased notes under the target version;
- run `cargo fmt --check`, `cargo test`, and `git diff --check`;
- commit `release 0.2.0`;
- create annotated tag `v0.2.0`.

4. Push the release commit and tag:

```powershell
git push origin main v0.2.0
```

Pushing the tag starts `.github/workflows/release.yml`, which builds Linux, macOS, and Windows archives and creates the GitHub Release.

## One-Step Push

To let the script push after the local checks pass:

```powershell
.\scripts\release.ps1 -Version 0.2.0 -Push
```

## Manual GitHub Release Retry

If packaging succeeds but publishing needs to be retried, run the `release` workflow manually in GitHub Actions and provide the existing tag, for example:

```text
v0.2.0
```

Manual runs check out that tag, rebuild artifacts, extract release notes from `CHANGELOG.md`, and recreate the GitHub Release.
