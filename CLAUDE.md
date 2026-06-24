# Project Rules

- dependency injection container must be constructed only once during the entire application lifetime
- `#[singleton]` attribute means that a given item is managed through DI container, and is constructed AT MOST once
- the entire generated code must be contained if the user's crate under the umbrella `margaret` module
