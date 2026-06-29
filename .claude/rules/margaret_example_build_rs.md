---
paths:
  - "margaret_example/build.rs"
---

# `margaret_example` `buidl.rs` rules

The rules apply to all the consumer crates in principle.

`build.rs` must stay minimal at the consumer side; it must be only a single, simple function invocation with a list of consumer crates to scan, nothing else. 
The entire weight of scanning, constructing a module, checking the surrounding of the generated module must be put on the framework instead, so the boilerplate stays minimal.
