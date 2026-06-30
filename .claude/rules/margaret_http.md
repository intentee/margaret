---
paths:
  - "margaret_http/**"
  - "margaret_macros/**"
---

# `margaret_http` crate rules

- each `http_responder` must respond to exactly one route
- globally defined middleware must not be supported; middleware must be explicitly added to responders
- responders need to support having multiple middlewares attached to them
- every item (http responders, http middlewares, etc) must be non-blocking, and async
- server must be able to process multiple connections in parallel

# `margaret_http` attributes

There must be no string heuristics, all the middleware handlers, route responders must attach themselves to an enum, or something verifiable at compile times.

Route patterns themselves, obviously, can be string patterns (compatible with `matchit`).

Specific attribute rules:

- `#[middleware(<name>)]` - `<name>` is the name of the actual middleware tag
- `#[responds_to_http(method = <method>, path = <path>, server = <marker>, symbol = <enum variant>)]` 
    * `<method>` must map directly into `Method` enum
    * `<path>` must be `matchit`-compatible string 
    * `<server>` is required and must be the full path to a declared `#[http_server]` marker
    * `<symbol>` is optional and must be the full path to a user-defined enum variant implementing `HttpRouteSymbol`
- `#[http_server(name = <name>)]` - attached to a marker struct that declares a named HTTP server; `<name>` is a required string literal, unique string literal (only used in CLI); routes target the marker by its full path through the required `server =` argument
- `#[provides_route_parameter]` - attached to a struct implementing `HttpRouteParameterBinder` with a specific type

# `margaret_http` servers

- an application may run multiple named HTTP servers in parallel (e.g. a public-facing one and an internal-facing one), each bound to its own address
- there is no implicit/default server; every server must be declared explicitly with `#[http_server(name = "<name>")]` and every route must name its server with `server = <full::path::to::marker>`
- the server name is the snake_case of the explicit `name` argument; server names must be globally unique — two markers resolving to the same name is a compile-time error
- a route references its server by the marker's full path (compile-time verifiable, never a string); a reference that resolves to more than one marker, to no marker, or that is missing entirely is a compile-time error (zero ambiguity)
- base and plugin crates may each declare a marker struct with the exact same type name (e.g. both `Internal`); the full-path reference and the explicit unique names keep resolution unambiguous
- a declared server with no routes is a compile-time error
- each declared server is generated as its own `server_<name>` entry point, registered as its own parallel `ServerService`, and reads its own `--<name>-addr` console argument
- forwarding is intra-server: each server's router only knows its own routes; trying to forward a route to a different server must be detectable at compile time

# `margaret_http` route symbols

- the framework never generates a route symbol enum; route symbols are defined by the user, in user-space, as their own enum implementing `HttpRouteSymbol`
- a route symbol is optional on a route; it is only declared (via `symbol = <full::path::to::Enum::Variant>`) when the user needs to point at that route from elsewhere in the codebase (e.g. as a forward target)
- a route without a `symbol =` is path-matched but cannot be forwarded to
- two routes declaring the same symbol is a compile-time error (each symbol identifies at most one responder)
- forwarding targets are referenced through the user's enum (`Forward::to(MyRouteSymbol::Variant)`), never a raw string

# `margaret_http` data flow

- each http request needs to through its entire middleware stack (first attribute in order becomes the first middleware to apply)
- http responder can forward requests to a different http responder; in that case the next responder's entire middleware stack must be reapplied
- all the interceptors must be applied only once - after the response is final (not forwarded to another responder)
- if an interceptor returns http responder, then the entire stack repeats (new middleware reapplied, response, interceptor) until we get the final response
- during the http request handling there must be no cycles (entering the same responder twice must result in a cycle error; same middleware or interceptors can be entered multiple times)
- interceptors must only attach themselves to marker traits, never specific types

# `margaret_http` lifecycle algorithm

Middleware -> responder -> interceptor loop must follow this algorithm. You need to assume that all the responders are stateless, so you need to add a safeguard against cycles to be safe.

```
INPUT:  request, response, currentNode
OUTPUT: a finalResponse

currentNode is always one of three categories:
    - Responder        (runs middleware, then emits a successor node)
    - Interceptable    (requires a registered interceptor to resolve)
    - FinalResponse    (terminal; nothing left to resolve)

PROCEDURE Resolve(request, response, currentNode):

    WHILE currentNode is not a FinalResponse:

        IF currentNode is a Responder:
            // Invariant: a Responder NEVER emits a successor
            // before its middleware has had a chance to redirect.
            redirect ← RunMiddleware(request, response, currentNode)
            IF redirect ≠ currentNode:
                currentNode ← redirect
            ELSE:
                currentNode ← currentNode.produceSuccessor(request, response)
            CONTINUE                          // re-categorize from the top

        IF currentNode is an Interceptable:
            currentNode ← RunInterceptor(request, response, currentNode)
            CONTINUE

    RETURN currentNode


PROCEDURE RunMiddleware(request, response, responderNode):
    middlewareList ← middleware registered for responderNode
    FOR EACH middleware IN middlewareList (in order):
        redirect ← middleware.preprocess(request, response, responderNode)
        IF redirect ≠ responderNode:
            RETURN redirect                   // first redirect wins
    RETURN responderNode                      // unchanged


PROCEDURE RunInterceptor(request, response, interceptableNode):
    interceptor ← interceptor registered for interceptableNode's type
    IF no interceptor exists:
        ERROR "no interceptor registered for this type"
    RETURN interceptor.intercept(request, response, interceptableNode)
```
