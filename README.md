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
| `wsw get <NAME>` | Show person's details |
| `wsw set <NAME> <FIELD=VALUE>...` | Update/add fields |
| `wsw note <NAME> <CONTENT>` | Add a timestamped note |
| `wsw log <NAME>` | View notes/history |
| `wsw list` | List all people (supports `--recent`, `--limit`) |
| `wsw search <QUERY>` | Search (supports `-f <FIELD>` for field-specific) |
| `wsw rm <NAME>` | Remove person (or `--field <FIELD>` to remove field) |

## Global Options

All commands support these options:

- `--db <PATH>` or `WSW_DB` env var - Custom database path
- `--json` - JSON output
- `-y, --yes` - Skip confirmations
- `--id` - Use ID instead of name (for get/set/note/log/rm)

## Usage Examples

### Add a person

```bash
wsw add "Alice" email=alice@example.com role=Engineer github=alice
```

### Quick lookup

```bash
wsw Alice
```

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

# Search specific field
wsw search -f role Manager
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
