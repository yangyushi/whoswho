# wsw (who is who)

A CLI tool to store and retrieve personal information (who is who) for AI agents and humans.

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

## Usage

### Add a person

```bash
wsw add "Alex Chen" Job=Engineer Company=TechCorp
wsw add "Jordan Smith" Title="Product Manager" Department=Growth
```

### Query a person

```bash
# By name (partial match supported)
wsw "Alex Chen"

# Exact match by ID
wsw get --id 1
```

### Update fields

```bash
wsw set "Alex Chen" Level=L5 Team="Platform"
wsw set --id 1 Location="San Francisco"
```

### Add notes

```bash
wsw note "Alex Chen" "Discussed Q3 roadmap, interested in AI features"
wsw note --id 1 "Follow up on infrastructure proposal"
```

### Show notes/history

```bash
wsw log "Alex Chen"
wsw log --id 1 --limit 10
```

### List all people

```bash
wsw list
wsw list --recent
wsw list --limit 20
```

### Search

```bash
# Search all fields
wsw search "TechCorp"

# Search specific field
wsw search --field Company "TechCorp"
wsw search --field Job "Engineer"
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

### JSON output (for LLM agents)

```bash
wsw "Alex Chen" --json
wsw list --json
wsw search "Engineer" --json
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

## Global Options

All commands support these options:

- `--db <PATH>` - Use custom database file
- `--json` - Output as JSON
- `-y, --yes` - Skip confirmations

## Handling Duplicate Names

When multiple people match a name, use `--id` to specify:

```bash
wsw set --id 3 Title="Staff Engineer"
```

## License

MIT
