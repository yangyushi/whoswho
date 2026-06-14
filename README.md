# wsw (who is who)

A CLI tool to store and retrieve personal information for AI agents and humans.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
# Binary will be at target/release/wsw
```

## Uninstallation

If installed with cargo:

```bash
cargo uninstall wsw
```

To remove all data (database file):

```bash
rm ~/.wsw.db
```

## Commands

| Command | Description |
|---------|-------------|
| `wsw [NAME]` | Quick lookup by name |
| `wsw add <NAME> [FIELD=VALUE]...` | Add a person with optional fields |
| `wsw get <NAME>` | Show person's details and notes |
| `wsw set <NAME> <FIELD=VALUE>...` | Update/add fields |
| `wsw note <NAME> <CONTENT>` | Add a timestamped note |
| `wsw log <NAME>` | View notes/history |
| `wsw list` | List all people with note counts (supports `--recent`, `--limit`) |
| `wsw search <QUERY>` | Search names, fields, and notes (supports `-f <FIELD>` and `-f notes`) |
| `wsw rm <NAME>` | Remove person (or `--field <FIELD>` to remove field) |

## Options

All commands support:

- `--db <PATH>` or `WSW_DB` env var - Custom database path

Quick lookup supports:

- `--json` - JSON output, for example `wsw --json Alice`

Command-specific options:

- `--json` - JSON output for `get`, `list`, and `search`; it may appear before or after those subcommands
- `--id` - Use ID instead of name for `get`, `set`, `note`, `log`, and `rm`
- `-y, --yes` - Skip confirmation for `rm`

## Usage Examples

### Add a person

```bash
wsw add "Alice" email=alice@example.com role=Engineer github=alice
```

### Quick lookup

```bash
wsw Alice
```

Quick lookup also prints that person's notes.

### Add notes

```bash
wsw note Alice "Met at Rust meetup"
wsw note Alice "Follow up about project collaboration"
```

### View history

```bash
wsw log Alice
```

### Search

```bash
# Search all fields
wsw search Engineer

# Search notes too
wsw search "project collaboration"

# Search specific field
wsw search -f role Manager

# Search only notes
wsw search -f notes "follow up"
```

### Update

```bash
wsw set Alice twitter=@alice location=SF
```

### List as JSON

```bash
wsw list --json
```

### Remove

```bash
# Remove entire person
wsw rm "Alex Chen"
wsw rm --id 1 -y

# Remove just one field
wsw rm "Alex Chen" --field Location
wsw rm --id 1 --field Level -y
```

## Database

By default, data is stored in `~/.wsw.db`. Use `--db` to specify a custom path:

```bash
wsw add "Taylor Park" Job=Designer --db ./project-alpha.db
```

Or set the `WSW_DB` environment variable:

```bash
export WSW_DB=/path/to/contacts.db
wsw list
```

## Data Model

- SQLite database (default location)
- Each person: `id`, `name`, `created_at`, `updated_at`
- Dynamic key-value fields (any FIELD=VALUE pairs)
- Timestamped notes attached to each person
