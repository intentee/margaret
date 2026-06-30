# Project Rules

- dependency injection container must be constructed only once during the entire application lifetime
- the entire generated code must be contained if the user's crate under the umbrella `margaret` module
- make sure that all the framework functions, traits, structs are single purpose, and focus on a single objective
- make sure that no attribute support "priority" of any kind; all the handlers, deciders etc must be unique (error otherwise); there must be zero ambiguity
- the framework itself must be opinionated, and take the heavy load onto itself; the applications that use it should not need to know the framework internals, everything needs to be managed by minimal, simple, declarative attributes and the DI container
