# Project Rules

- never edit .claude/rules or CLAUDE.md
- dependency injection container must be constructed only once during the entire application lifetime
- the entire generated code must be contained if the user's crate under the umbrella `margaret` module
- make sure that all the framework functions, traits, structs are single purpose, and focus on a single objective
- make sure that no attribute support "priority" of any kind; all the handlers, deciders etc must be unique (error otherwise); there must be zero ambiguity
- the framework itself must be opinionated, and take the heavy load onto itself; the applications that use it should not need to know the framework internals, everything needs to be managed by minimal, simple, declarative attributes and the DI container
- nothing in the consumer project can be implicit, everything must be explicitly bound together for attributes
- if an enum is used, or any struct is referenced, the entire path to it must be present in the file that uses it (through an attribute or not)
- never use string heuristics in the attributes, all the connections, bindings, architecture, attributes must be verifiable at compile time
- the framework must never rely on purely the name of a struct, or a function (for example it always must support two route handlers from different modules, that are named exactly same, etc)
- every inconsistency, or ambiguity (like invalid route, invalid view name, invalid redirect etc) must be detected at compile time
