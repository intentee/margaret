---
paths:
  - "margaret_codegen/**"
---

# `margaret_codegen` crate rules

- `margaret_codegen` must genrate minimal amount of features to satisfy the attributes (for example if there are no `http` server attributes, it must not generate server entry point)
- all the generated code must be generated under umbrella `margaret` crate, so the framework users can import it as a `main` entry point
