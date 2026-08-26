# Margaret

Margaret is an opinionated Rust application framework with compile-time verified architecture and dependency injection.

Margaret moves application wiring, dependency construction, routes, services, models, validation, views, WebSockets, identity, and supporting infrastructure into declarative attributes. The framework validates connections during code generation and exposes the generated application under the consumer crate's `margaret` module.

> Margaret 0.1 is an early release. Its APIs may change between minor versions.

## Requirements

- Rust 1.95.0 or newer
- Cargo with edition 2024 support

## Installation

Margaret has separate runtime and code-generation feature sets. Add the same crate version as both a normal dependency and a build dependency:

```toml
[dependencies]
margaret = "0.3.0"

[build-dependencies]
margaret = { version = "0.3.0", default-features = false, features = ["codegen"] }
```

Run Margaret's generator from the consumer's build script:

```rust
fn main() -> Result<(), margaret::framework::codegen::codegen_error::CodegenError> {
    margaret::framework::codegen::generate::generate()
}
```

Application code uses the runtime facade through `margaret::framework`. Generated code is kept inside the consumer crate under its generated `margaret` module.

## Features

- `runtime` is enabled by default and exposes Margaret's application runtime through `margaret::framework`.
- `codegen` exposes the compile-time application generator and is intended for build dependencies.

The [`margaret_example`](margaret_example) workspace crate demonstrates a complete application with commands, HTTP routes, views, models, services, identity, tokens, uploaded files, assets, validation, and WebSocket sessions.

## Development

Run the unit test and lint gates with:

```console
make test.unit
make clippy
```

The complete integration and coverage gates require Docker and the project development dependencies:

```console
make test.integration
make coverage
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
