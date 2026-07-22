---
paths:
  - "margaret_service/**"
  - "margaret_service_tests/**"
---

# `margaret_service` crate rules

- services need to reuse `#[runner]` attribute in the method that actually starts the service
- `cancellation_token` argument is optional; if a service does not need it it should not be injected
