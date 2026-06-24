---
paths:
  - "margaret_attributes/**"
---

# `margaret_attributes` crate rules

- `margaret_attributes` must be focused only on detecting, and interpreting attributes
- `margaret_attributes` must be the only, authoritative place in the entire project that reads and processes attributes
- `margaret_attributes` must not decide about anything related to business logic, lifetime of singletons, and it must not give any semantic meaning to attributes
- `margaret_attributes` must provide tools that allow other crates to interpret and process the attributes
