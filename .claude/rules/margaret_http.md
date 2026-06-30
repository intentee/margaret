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

# `margaret_http` data flow

- each http request needs to through its entire middleware stack (first attribute in order becomes the first middleware to apply)
- http responder can forward requests to a different http responder; in that case the next responder's entire middleware stack must be reapplied
- all the interceptors must be applied only once - after the response is final (not forwarded to another responder)
- if an interceptor returns http responder, then the entire stack repeats (new middleware reapplied, response, interceptor) until we get the final response
- during the http request handling there must be no cycles (entering the same responder twice must result in a cycle error; same middleware or interceptors can be entered multiple times)
- interceptors must only attach themselves to marker traits, never specific types

# `margaret_http` attributes

There must be no string heuristics, all the middleware handlers, route responders must attach themselves to an enum, or something verifiable at compile times.

Route patterns themselves, obviously, can be string patterns (compatible with `matchit`).

Specific attribute rules:

- `#[middleware(<name>)]` - `<name>` is the name of the actual middleware tag
- `#[responds_to_http(method = <method>, path = <path>)]` - `<method>` must map directly into `Method` enum, `<path>` must be `matchit`-compatible string
- `#[provides_route_parameter]` - attached to a struct implementing `HttpRouteParameterBinder` with a specific type

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
