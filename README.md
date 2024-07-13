# SurrealDB Query Builder

A query builder for SurrealDB that generates DEFINE statements compatible with the [SurrealDB Rust Library](https://surrealdb.com/docs/sdk/rust).

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [SurrealDB](https://surrealdb.com/docs/)

## Features

- **Table Builder**: Generates `DEFINE TABLE` statements.
- **Field Builder**: Generates `DEFINE FIELD` statements.
- **Fields Builder**: Generates multiple `DEFINE FIELD` statements.
- **Indexes Builder**: Generates multiple `DEFINE INDEX` statements.

## To-Do Checklist

- [ ] **Function**: Implement `DEFINE FUNCTION` statements. [Documentation](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/function)
- [ ] **Event**: Implement `DEFINE EVENT` statements. [Documentation](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/event)
- [ ] **Params**: Implement `DEFINE PARAM` statements. [Documentation](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/param)
- [ ] **Analyzer**: Implement `DEFINE ANALYZER` statements. [Documentation](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/analyzer)

## Installation

To add the SurrealDB Query Builder to your project, run:

```bash
cargo add surrealqb
```

## Building the Project

To build the project, use:

```bash
cargo build
```

## Running Tests

To run tests, use:

```bash
cargo test -- --nocapture
```


## Developer
- JP Mateo (jpmateo022@gmail.com)