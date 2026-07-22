---
paths:
  - "margaret_asset_bag_codegen/**"
  - "margaret_codegen/**"
  - "margaret_codegen_tokens/**"
  - "margaret_console_argument_codegen/**"
  - "margaret_console_codegen/**"
  - "margaret_http_codegen/**"
  - "margaret_injection_codegen/**"
  - "margaret_middleware_codegen/**"
  - "margaret_model_codegen/**"
  - "margaret_request_binding_codegen/**"
  - "margaret_route_parameter_codegen/**"
  - "margaret_service_codegen/**"
  - "margaret_views_codegen/**"
  - "margaret_websocket_codegen/**"
---

# `margaret_codegen` crate rules

- `margaret_codegen` must generate minimal amount of features to satisfy the attributes (for example if there are no `http` server attributes, it must not generate server entry point)
- all the generated code must be generated under umbrella `margaret` crate, so the framework users can import it as a `main` entry point
- `prettyplease` or any kind of another code formatter can be applied AT MOST ONCE PER FILE during the entire code generation phase
- make sure that the input codebase is converted to AST exactly ONCE during the building phase, then reused (to optimize the speed of the generation)
