---
paths:
  - "margaret_console/**"
---

# `margaret_console` crate rules

- commands need to be declarative (input arguments need to be specified in command's arguments)
- all the input arguments, and configuration must be specifiable through declarative attributes
- when no subcommand is specified, `margaret_console` must default to `clap`'s default help view
- command handlers must be async
