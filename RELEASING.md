# Releasing

`wsw` uses Semantic Versioning. `Cargo.toml` is the source of truth for the current package version, and GitHub Releases are created from annotated Git tags.

## Version Rules

- Patch: bug fixes and documentation-only changes.
- Minor: backward-compatible CLI features, output additions, and search behavior improvements.
- Major: incompatible CLI behavior, incompatible JSON output, or database migrations that require user action.

## Normal Release Flow

1. Keep user-facing changes documented under `## [Unreleased]` in `CHANGELOG.md`.
2. Make sure feature and fix commits are already committed on `main`.
3. Run the release script:

```powershell
.\scripts\release.ps1 -Version 0.2.0
```

The script will:

- require a clean working tree on `main`;
- update `Cargo.toml`;
- refresh `Cargo.lock` with `cargo check`;
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
