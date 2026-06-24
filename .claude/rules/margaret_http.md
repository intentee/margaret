---
paths:
  - "margaret_http/**"
---

# `margaret_http` crate rules

- each `http_responder` must respond to exactly one route
- globally defined middleware must not be supported; middleware must be explicitly added to responders
- responders need to support having multiple middlewares attached to them
- every item (http responders, http middlewares, etc) must be non-blocking, and async
- server must be able to process multiple connections in parallel
