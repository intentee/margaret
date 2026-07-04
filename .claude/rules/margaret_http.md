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
- `#[responds_to_http(method = <method>, name = <name>, path = <path>, server = <marker>)]` 
    * `<method>` is a compile-time validate HTTP verb (lowercase): "get", "post", "put", "delete", "patch", "query"
    * `<name>` is optional and must be a globally unique route name string (validated at compile time)
    * `<path>` must be `matchit`-compatible string 
    * `<server>`  must be a string with a server name
- `#[provides_route_parameter]` - attached to a struct implementing `HttpRouteParameterBinder` with a specific type
- `#[route_parameter(from = <from>)]` 
    * used as a part of `#[responder]` method
    * `<from>` must be the name of a route parameter that is mapped into the argument (must result in compile time error if there is no such parameter)

# `margaret_http` data flow

- each http request needs to through its entire middleware stack (first attribute in order becomes the first middleware to apply)
- http responder can forward requests to a different http responder; in that case the next responder's entire middleware stack must be reapplied
- all the interceptors must be applied only once - after the response is final (not forwarded to another responder)
- if an interceptor returns http responder, then the entire stack repeats (new middleware reapplied, response, interceptor) until we get the final response
- during the http request handling there must be no cycles (entering the same responder twice must result in a cycle error; same middleware or interceptors can be entered multiple times)
- interceptors must only attach themselves to marker traits, never specific types

# `margaret_http` request object

- request's form data, or any kind of input data must be parsed exactly once during the entire request's lifecycle
- request object must be similar in structure to what PHP provides globally ($_POST, $_GET, $_SERVER, $_FILES) - this kind of information must not be global, it must be scoped to the request
- uploaded files must be streamed into temporary directory, and only stored as a pointer (similar to PHP)
- temporary directory for uploads must be configurable per-server through CLI server flag (defaults to system TMP dir)
- request object must not be clone-able, and most not be cloned, or copied at any point; its properties must not be cloned or copied at any point in the framework
- request object must not be mutable in any way
- request object must carry the `Server` struct with the information about the server (`margaret_http/src/server.rs`)
- all the request fields that are keyed (equivalents to $_POST, $_GET, $_COOKIE, $_SERVER, $_FILES, path params etc), must be accessible with O(1) complexity
- make sure there are no aliases; specific information needs to be reachable in exactly one way (for example if $_SERVER has 'remote_addr' do not expose the same field again in Request)

# `margaret_http` routes object

Routes object must be generated so linking between routes must be validated during compile time.

It must be usable with this API: `router.<server_name>.<route_name>.<action>()`. Notice that only `action` is a method, prior chain members are regular properties.

- supported `<action>`:
    - `forward_to(<route_name>)`, for example `router.public.get_article.forward_to(GetArticleRouteProps{ ... })` (`GetArticleRouteProps` also must be generated at compile time)
    - `forward_to` should only work for GET routes (because it doesn't carry over any data for different kinds of routes)
- routes object is generated out of all the named routes at compile time
- cross-server redirects and references must be supported
- routes object must be constructed exactly once during the entire application lifecycle
- routes, and servers within routes object must be accessible with O(1) complexity


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
