pub mod auth;
pub mod commands;
pub mod config;
pub mod constants;
pub mod databases;
pub mod enums;
pub mod errors;
pub mod forms;
pub mod models;
pub mod repositories;
pub mod route_parameters;
pub mod routes;
pub mod services;
pub mod shapes;
pub mod singletons;
pub mod tickers;
pub mod views;
pub mod websocket;

#[rustfmt::skip]
#[path = "../margaret/mod.rs"]
pub mod margaret;
