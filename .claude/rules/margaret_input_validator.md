---
paths:
  - "margaret_input_validator/**"
---

# `margaret_input_validator` crate rules

- crate provides the base for validation, and error handling to be reused over the entire framework
- crate must be based on serde, and provide a universal validator
- validator must be usable internally by `margaret_console`, and `margaret_http` to validate the input data
- validator must only receive a ready to use data source
- invalid input data must not be treated as error (validation is successful, data might be incorrect); only fail when there are actual errors during validation process

