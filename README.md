# Margaret

An opinionated Rust application framework with compile-time verified architecture and dependency injection.

The [`margaret_example`](margaret_example) crate is a complete application built with it.

The example keeps all of its data in Postgres, so any number of its instances can serve the same blog. Start a local database, create the schema, and load the sample data with:

```sh
make example.database example.migrate example.seed
```

## Models

`#[model]` structs are active records generated at compile time. A query can only walk the declared primary key, unique constraints and indexes, every read is bounded by a compile-time limit or streamed in batches, and relations load eagerly through `#[eager_load]` shapes, so N+1 queries, unindexed filters and unbounded reads or writes do not compile. The framework keeps its own tables (signing keys, client assertions and authorization grants) with the same models.

## Tests

`make test.unit` needs no services. `make test.postgres`, `make test.cluster` and `make test.conformance` start a disposable Postgres in Docker, and `make coverage` runs every suite.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
