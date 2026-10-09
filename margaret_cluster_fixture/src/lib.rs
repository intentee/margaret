pub mod alice;
pub mod auth;
pub mod cluster_database;
pub mod commands;
pub mod forms;
pub mod models;
pub mod routes;
pub mod services;
pub mod stream_batch;
pub mod system_clock;
pub mod tickers;
pub mod views;

#[rustfmt::skip]
#[path = "../margaret/mod.rs"]
pub mod margaret;
