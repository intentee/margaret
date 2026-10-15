# Margaret

An opinionated Rust application framework with compile-time verified architecture and dependency injection.

The [`margaret_example`](margaret_example) crate is a complete application built with it.

The example keeps all of its data in Postgres, so any number of its instances can serve the same blog. Start a local database, create the schema, and load the sample data with:

```sh
make example.database example.migrate example.seed
```

## Models

`#[model]` structs are active records generated at compile time. A query can only walk the declared primary key, unique constraints and indexes, every read is bounded by a compile-time limit or streamed in batches, and relations load eagerly through `#[eager_load]` shapes, so N+1 queries, unindexed filters and unbounded reads or writes do not compile. The framework keeps its own tables (signing keys, client assertions and authorization grants) with the same models.

## Signing keys

An application that holds signing keys (to issue tokens or sessions, or to publish its JWKS) stores them sealed with AES-256-GCM under a key it reads from the environment variable named by `#[encrypts_signing_keys(key_from = "...")]`, holding 32 bytes in standard padded base64 (for example from `openssl rand -base64 32`). Such an application does not compile without that declaration, and one that declares it without signing anything does not compile either. Give every deployment its own key, since a sealed key set opens wherever the same key is configured.

## Tests

`make test.unit` needs no services. `make test.postgres`, `make test.cluster` and `make test.conformance` start a disposable Postgres in Docker, and `make coverage` runs every suite.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
