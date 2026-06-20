# Agent Notes

## Branch Policy

- `main` contains released code only.
- `dev` is the integration branch for reviewed changes before release.
- Day-to-day work should happen on short-lived branches created from `dev`.
- Use branch prefixes that describe the intent, such as `feature/`, `bugfix/` and `refactor/`.
- Finished work should be rebased onto the latest `dev`, then proposed as a PR targeting `dev`.
- Accumulated changes are merged from `dev` into `main` when preparing a release.
- Releases are created from `main` by bumping the version, updating the changelog, creating an annotated tag, and letting GitHub Actions build the release artifacts.

## Release Notes

- Keep user-facing changes under `## [Unreleased]` in `CHANGELOG.md` while work is happening on work branches or `dev`.
- Do not move unreleased notes into a numbered changelog section until the release is being prepared on `main`.
- See `docs/RELEASING.md` for the full release flow.

## Git Handling

- Before making changes, check the current branch.
- If the requested work is a small fix and the repo is on `main`, switch to `dev`, update it, and create a short-lived branch before editing when possible.
- Rebase the work branch onto the latest `dev` before opening or updating a PR.
- PRs for feature, bugfix, and refactor branches should target `dev`.
- Do not rewrite history or reset user changes unless explicitly requested.
