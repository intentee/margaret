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

# `margaret_http` data flow

- each http request needs to through its entire middleware stack (first attribute in order becomes the first middleware to apply)
- http responder can forward requests to a different http responder; in that case the next responder's entire middleware stack must be reapplied
- all the interceptors must be applied only once - after the response is final (not forwarded to another responder)
