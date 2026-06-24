---
paths:
  - "margaret_container/**"
---

# `margaret_container` crate rules

- `margaret_container` must be the only, and authoritative place that manages the dependency injection margaret_container
- `#[singleton]` attribute means that an item is managed by the container, and must only be constructed at most once
- dependency injection container must be constructed at most once over the entire application lifecycle
