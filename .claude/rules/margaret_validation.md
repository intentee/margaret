---
paths:
  - "margaret_validation/**"
---

# `margaret_validation` crate rules

- crate provides the base for validation, and error handling to be reused over the entire framework
- invalid input data must not be treated as error (validation is successful, data might be incorrect); only fail when there are actual errors during validation process

