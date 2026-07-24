---
paths:
  - "margaret_container/**"
  - "margaret_container_tests/**"
  - "margaret_macros/**"
---

# `margaret_container` crate rules

- `margaret_container` must be the only, and authoritative place that manages the dependency injection margaret_container
- `#[singleton]` attribute means that an item is managed by the container, and must only be constructed at most once
- dependency injection container must be constructed at most once over the entire application lifecycle
- all the dependencies must be constructed lazily - exactly when they are needed, but at most once over the entire application's lifecycle
- injected dependencies must not put any overhead on runtime (there must be no dependency on the container from the point of view of a `#[singleton]`, once the `#[singleton]` is constructed)
- container needs to be async by default; sync initialization must not be supported
