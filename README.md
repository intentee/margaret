# Margaret

An opinionated Rust application framework with compile-time verified architecture and dependency injection.

The [`margaret_example`](margaret_example) crate is a complete application built with it.

The example keeps all of its data in Postgres, so any number of its instances can serve the same blog. Start a local database, create the schema, and load the sample data with:

```sh
make example.database example.migrate example.seed
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
