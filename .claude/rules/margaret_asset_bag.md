---
paths:
  - "margaret_asset_bag/**"
  - "margaret_asset_bag_codegen/**"
---

# `margaret_asset_bag` rules

The goal of `margaret_asset_bag` crates is to expose front-end assets in a way that:
- they must be embeddable entirely in the back-end binary
- they can ONLY be accessed through macros that validate assets at compile time
- invalid paths must be detected at compile time
- content types must be derived from the files themselves, and `esbuild-meta.json` itself (never just from the filename)
- both content type and the content itself must be resolved compile-time

