---
paths:
  - "margaret_jwks_key_gen/**"
  - "margaret_jwks_key_gen_tests/**"
---

# `margaret_jwks_key_gen` crate rules

- make sure that the crate is cryptographically correct
- make sure that the token is generated only once during its lifecycle; make sure its never disassembled, or reassembled to provide crate's features
- make sure that it is structurally impossible to make bad security choices when using the crate, be opinionated
- make sure to not implement any cryptography by yourself, rely on extremely well vetted 3rd party crates (never introduce dependencies without discussing first)
