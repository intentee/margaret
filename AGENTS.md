# Project Rules

## General Principles

- make sure that all the framework functions, traits, structs are single purpose, and focus on a single objective
- make sure that no attribute support "priority" of any kind; all the handlers, deciders etc must be unique (error otherwise); there must be zero ambiguity
- the framework itself must be opinionated, and take the heavy load onto itself; the applications that use it should not need to know the framework internals, everything needs to be managed by minimal, simple, declarative attributes and the DI container
- nothing in the consumer project can be implicit, everything must be explicitly bound together for attributes
- if an enum is used, or any struct is referenced, the entire path to it must be present in the file that uses it (through an attribute or not)
- never use string heuristics in the attributes, all the connections, bindings, architecture, attributes must be verifiable at compile time
- the framework must never rely on purely the name of a struct, or a function (for example it always must support two route handlers from different modules, that are named exactly same, etc)
- every inconsistency, or ambiguity (like invalid route, invalid view name, invalid redirect etc) must be detected at compile time
- the entire project needs 100% code coverage (`margaret_example` must not be instrumented or included in the coverage)
- never edit the rules, never edit your own settings, never edit CLAUDE.md, never edit anything under .claude
- the attributes must be read only once during the entire application lifecycle, then reused from memory through attribute selector
- never create aliases, make sure that a specific information is only achievable in a single way (and code is never duplicated)
- there must always be exactly as single correct, single available way to do any specific thing
- never add syntax sugar of any kind; optimize the base architecture instead
- the codebase must never panic (panics are only allowed in the tests)
- generated code must never panic
- generated code must also follow all the local, and global rules (including module organization, rust code structure, and such)
- generate code must be minimal; if anything can be a part of a framework (and unit tested), it must be
- framework must never limit the user with naming their components, routes, and services (besides forcing conventions like snake case or such, but there must never be a list of reserved words that users can't choose)

## Dependency Injection Container

- dependency injection container must be constructed only once during the entire application lifetime

## Generated Code

- the entire generated code must be contained if the user's crate under the umbrella `margaret` module

