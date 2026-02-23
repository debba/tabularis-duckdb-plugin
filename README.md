<div align="center">
  <img src="https://raw.githubusercontent.com/debba/tabularis/main/public/logo-sm.png" width="120" height="120" />
</div>

# tabularis-plugin-duckdb

<p align="center">

![](https://img.shields.io/github/release/debba/tabularis-plugin-duckdb.svg?style=flat)
![](https://img.shields.io/github/downloads/debba/tabularis-plugin-duckdb/total.svg?style=flat)
![Build & Release](https://github.com/debba/tabularis-plugin-duckdb/workflows/Release/badge.svg)
[![Discord](https://img.shields.io/discord/1470772941296894128?color=5865F2&logo=discord&logoColor=white)](https://discord.gg/YrZPHAwMSG)

</p>

A [DuckDB](https://duckdb.org/) plugin for [Tabularis](https://github.com/debba/tabularis), the lightweight database management tool.

This plugin enables Tabularis to work with DuckDB databases — both file-based and in-memory — providing full schema inspection, CRUD operations, view management, and DDL generation through a JSON-RPC 2.0 over stdio interface.

**Discord** - [Join our discord server](https://discord.gg/YrZPHAwMSG) and chat with the maintainers.

## Table of Contents

- [Features](#features)
- [Supported DuckDB Data Types](#supported-duckdb-data-types)
- [Installation](#installation)
  - [Automatic (via Tabularis)](#automatic-via-tabularis)
  - [Manual Installation](#manual-installation)
- [How It Works](#how-it-works)
- [Supported Operations](#supported-operations)
- [Building from Source](#building-from-source)
- [Development](#development)
- [Changelog](#changelog)
- [License](#license)

## Features

- **File-based & In-memory databases** — Open `.duckdb` files or work with ephemeral in-memory databases.
- **Schema Inspection** — Browse tables, columns, primary keys, foreign keys, and indexes.
- **View Management** — Create, alter, drop, and inspect views with full metadata.
- **SQL Execution** — Run any SQL query with automatic pagination for SELECT statements.
- **Inline Editing** — Insert, update, and delete records directly from the Tabularis data grid.
- **DDL Generation** — Generate CREATE TABLE, ALTER TABLE, CREATE INDEX, and FOREIGN KEY statements.
- **Schema Snapshot** — Fetch full schema metadata (tables, columns, foreign keys) in a single call.
- **BLOB Support** — Handle binary data with base64 encoding and file reference wire formats.
- **Complex Types** — Full support for DuckDB-specific types: LIST, MAP, STRUCT, UNION, ENUM, and more.
- **Bundled DuckDB** — Statically embeds DuckDB; no external dependencies required at runtime.
- **Cross-platform** — Pre-built binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x86_64).

## Supported DuckDB Data Types

| Category | Types |
|---|---|
| **Numeric** | TINYINT, SMALLINT, INTEGER, BIGINT, HUGEINT, UTINYINT, USMALLINT, UINTEGER, UBIGINT, UHUGEINT, FLOAT, DOUBLE, DECIMAL |
| **String** | VARCHAR, TEXT, UUID, ENUM |
| **Date/Time** | DATE, TIME, TIMESTAMP, TIMESTAMP WITH TIME ZONE, TIMESTAMP_NS, TIMESTAMP_MS, TIMESTAMP_S, TIME WITH TIME ZONE, INTERVAL |
| **Binary** | BLOB |
| **JSON** | JSON |
| **Composite** | LIST, MAP, STRUCT, UNION |
| **Other** | BOOLEAN, BIT |

## Installation

### Automatic (via Tabularis)

If your version of Tabularis supports plugin management, the DuckDB plugin can be installed directly from the application.

### Manual Installation

1. Download the latest release for your platform from the [Releases page](https://github.com/debba/tabularis-plugin-duckdb/releases).
2. Extract the archive.
3. Copy `tabularis-duckdb-plugin` (or `tabularis-duckdb-plugin.exe` on Windows) and `manifest.json` into the Tabularis plugins directory:

| OS | Plugins Directory |
|---|---|
| **Linux** | `~/.local/share/tabularis/plugins/duckdb/` |
| **macOS** | `~/Library/Application Support/com.debba.tabularis/plugins/duckdb/` |
| **Windows** | `%APPDATA%\com.debba.tabularis\plugins\duckdb\` |

4. Restart Tabularis.

## How It Works

The plugin is a standalone Rust binary that communicates with Tabularis through **JSON-RPC 2.0 over stdio**:

1. Tabularis spawns the plugin as a child process.
2. Requests are sent as newline-delimited JSON-RPC messages to the plugin's `stdin`.
3. Responses are written to `stdout` in the same format.

This architecture keeps the plugin fully isolated — no shared memory, no network ports, no external dependencies.

## Supported Operations

| Method | Description |
|---|---|
| `test_connection` | Verify database connectivity |
| `get_databases` | List databases |
| `get_schemas` | List schemas |
| `get_tables` | List tables in a schema |
| `get_columns` | Get column metadata for a table |
| `get_foreign_keys` | Get foreign key constraints |
| `get_indexes` | Get indexes for a table |
| `get_views` | List all views |
| `get_view_definition` | Get SQL definition of a view |
| `get_view_columns` | Get columns for a view |
| `create_view` | Create a new view |
| `alter_view` | Modify an existing view |
| `drop_view` | Drop a view |
| `execute_query` | Execute SQL with pagination support |
| `insert_record` | Insert a new row |
| `update_record` | Update a cell by primary key |
| `delete_record` | Delete a row by primary key |
| `get_schema_snapshot` | Full schema dump in one call |
| `get_all_columns_batch` | All columns for all tables |
| `get_all_foreign_keys_batch` | All foreign keys for all tables |
| `get_create_table_sql` | Generate CREATE TABLE DDL |
| `get_add_column_sql` | Generate ADD COLUMN DDL |
| `get_alter_column_sql` | Generate ALTER COLUMN DDL |
| `get_create_index_sql` | Generate CREATE INDEX DDL |
| `get_create_foreign_key_sql` | Generate FOREIGN KEY DDL |
| `drop_index` | Drop an index |
| `drop_foreign_key` | Drop a foreign key constraint |

## Building from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)

### Build

```bash
cargo build --release
```

The binary will be located at `target/release/tabularis-duckdb-plugin`.

### Install Locally

A convenience script is provided to build and copy the plugin to the Tabularis plugins directory:

```bash
./sync.sh
```

## Development

### Testing the Plugin

Two development binaries are included for testing:

**Direct DuckDB test:**

```bash
cargo run --bin test
```

**Simulated Tabularis integration test** (spawns the plugin and sends JSON-RPC requests via stdio):

```bash
cargo run --bin test_plugin
```

### Tech Stack

- **Language:** Rust (edition 2021)
- **Database:** [DuckDB](https://duckdb.org/) v1.1.1 (bundled/statically linked)
- **Serialization:** serde + serde_json
- **Protocol:** JSON-RPC 2.0 over stdio

## [Changelog](./CHANGELOG.md)

## License

Apache License 2.0
