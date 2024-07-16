# SurrealDB Query Builder

A query builder for SurrealDB that generates DEFINE statements compatible with the [SurrealDB Rust Library](https://surrealdb.com/docs/sdk/rust).

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [SurrealDB](https://surrealdb.com/docs/)
- [SurrealDB Rust Crate](https://docs.rs/surrealdb/latest/surrealdb/index.html)


## References
- Statement References: [SurrealDB Doc](https://surrealdb.com/docs/surrealdb/surrealql/statements)
- Crate Documentation: [SurrealDB Crate Doc](https://docs.rs/surrealdb/latest/surrealdb/index.html)

## Supported Statements
- **Table Builder**: Generates `DEFINE TABLE` statement.
- **Field Builder**: Generates `DEFINE FIELD` statement.
- **Index Builder**: Generates `DEFINE INDEX` statement.
- **Param Builder**: Generates `DEFINE PARAM` statement.
- **Query Builder**: Generates multiple statements.

## To-Do Checklist

- [ ] **Function**: Implement `DEFINE FUNCTION` statements. [Documentation](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/function)
- [ ] **Event**: Implement `DEFINE EVENT` statements. [Documentation](https://surrealdb.com/docs/surrealdb/surrealql/statements/define/event)
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