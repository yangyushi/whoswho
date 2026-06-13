# Changelog

All notable changes to this project are documented in this file.

The format follows Keep a Changelog, and this project uses Semantic Versioning.

## [Unreleased]

### Added
- `wsw list` shows note counts for each person.
- `wsw get` and quick lookup print all notes for the selected person.
- `wsw search` searches note content by default and supports `-f notes`.
- Release automation for version bumping, changelog extraction, CI, and GitHub Releases.

### Changed
- Adding a note updates the person's `updated_at`, so `wsw list --recent` reflects recent note activity.
- `wsw note` output distinguishes the note ID from the person ID.
