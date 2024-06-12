# Tusk

## Config

```toml
# ~/.tusk/config.toml
default = "local"

[[connection]]
name = "local"
user = "postgres"
password = "password"
host = "locahost"
port = 5432
database = "test"
```

## Usage

Each subcommand comes with a `-h` flag.

```console
❯ tusk -h
Postgres tuning and utility cli

Usage: tusk [OPTIONS] <COMMAND>

Commands:
  ls
  query
  help   Print this message or the help of the given subcommand(s)

Options:
  -p, --profile <PROFILE>
  -h, --help               Print help
  -V, --version            Print version
❯ tusk query -h
Usage: tusk query [OPTIONS] <COMMAND>

Commands:
  older-than
  kill
  help        Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>  [default: table] [possible values: json, table, yaml]
  -h, --help             Print help
```

Check for long running queries

```console
❯ tusk query older-than 5min
+--------+-------------------------+-----------------------------------------------+-------+
| pid    | duration                | query                                         | state |
+--------+-------------------------+-----------------------------------------------+-------+
| 118207 | 39 days 10:42:24.541683 | SELECT pg_catalog.pg_postmaster_start_time... | idle  |
+--------+-------------------------+-----------------------------------------------+-------+
```

Optionally, output as `json`...

```bash
❯ tusk query older-than 5min -o json | jq
```
```json
[
  {
    "pid": 118207,
    "duration": "39 days 10:44:54.355862",
    "query": "SELECT pg_catalog.pg_postmaster_start_time...",
    "state": "idle"
  }
]
```


or `yaml`...

```bash
❯ tusk query older-than 5min -o yaml | yq
```

```yaml
- pid: 57114
  duration: 39 days 10:45:35.301343
  query: SELECT pg_catalog.pg_postmaster_start_time...
  state: idle
```

These will output the entire query field. They have been truncated in the example for readability.