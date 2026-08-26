pub mod app_name;
pub mod auth;
pub mod commands;
pub mod deployment_environment;
pub mod english_greeter;
pub mod forms;
pub mod jwks_endpoint;
pub mod metrics;
pub mod models;
pub mod routes;
pub mod services;
pub mod stores;
pub mod sweep_interval;
pub mod system_clock;
pub mod tickers;
pub mod views;

#[rustfmt::skip]
#[path = "../margaret/mod.rs"]
pub mod margaret;
