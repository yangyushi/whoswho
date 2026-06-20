# Changelog

All notable changes to this project are documented in this file.

The format follows Keep a Changelog, and this project uses Semantic Versioning.

## [Unreleased]

### Added
- Quick lookup now accepts a numeric person ID, for example `wsw 58`.

### Changed
- `wsw list` now aligns ID, name, and note-count columns based on the current result set.
- `wsw list` now fetches note counts with the people list instead of running one count query per person.
- `wsw list` now sorts by recently updated people by default and no longer supports `--recent`.

## [0.2.1] - 2026-06-14

### Fixed
- CLI help and README option documentation now only advertise options that each command actually uses.

## [0.2.0] - 2026-06-13

### Added
- `wsw list` shows note counts for each person.
- `wsw get` and quick lookup print all notes for the selected person.
- `wsw search` searches note content by default and supports `-f notes`.
- Release automation for version bumping, changelog extraction, CI, and GitHub Releases.

### Changed
- Adding a note updates the person's `updated_at`.
- `wsw note` output distinguishes the note ID from the person ID.
