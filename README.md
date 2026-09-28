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
margaret = "0.4.0"

[build-dependencies]
margaret = { version = "0.4.0", default-features = false, features = ["codegen"] }
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

## Trusting OpenID Providers

A `#[singleton]` that implements `DeclaresTokenTrust` declares a trusted OpenID Provider with `#[trusts_oidc_issuer(tag)]`. Margaret finds the provider's signing keys through OpenID Connect Discovery, keeps them fresh in a background service, and verifies bearer tokens against the declared issuer and audience.

An `#[infers_authenticated_user]` provider receives the verification of the request's bearer token as an argument of its `#[infer_from_request]` method:

```rust
#[infer_from_request]
pub fn infer_service_account(
    &self,
    #[oidc_token(issuer = partner)] verification: OidcTokenVerification<PartnerClaims>,
) -> anyhow::Result<AuthenticatedUserOutcome<ServiceAccount>> {
    Ok(match verification {
        OidcTokenVerification::Absent | OidcTokenVerification::NotBearer => {
            AuthenticatedUserOutcome::Anonymous
        }
        OidcTokenVerification::Rejected(rejection) => AuthenticatedUserOutcome::Interrupted(
            ResponseContinuation::from(rejection.challenge().response()),
        ),
        OidcTokenVerification::Unavailable => AuthenticatedUserOutcome::Interrupted(
            ResponseContinuation::from(Response::text(503, "the signing keys are not available yet")),
        ),
        OidcTokenVerification::Verified(verified) => {
            let PartnerClaims { sub, tenant_id } = verified.claims;

            if tenant_id == self.tenant_id {
                AuthenticatedUserOutcome::Authenticated(ServiceAccount { subject: sub })
            } else {
                AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(
                    Response::forbidden(),
                ))
            }
        }
    })
}
```

A route that requires such a user answers an anonymous request with `401 Unauthorized` and `WWW-Authenticate: Bearer`. A malformed `Authorization` header is answered with `invalid_request`, and a token that fails verification with `invalid_token`.

The example's `GET /ci/runner` route demonstrates an OIDC-protected route.

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
