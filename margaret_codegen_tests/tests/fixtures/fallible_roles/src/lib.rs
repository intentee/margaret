include!(concat!(env!("OUT_DIR"), "/margaret.rs"));

pub mod boot;
pub mod chat;
pub mod get_home;
pub mod get_profile;
pub mod get_user;
pub mod guard;
pub mod home_view;
pub mod home_view_props;
pub mod interval;
pub mod pulse;
pub mod result;
pub mod secrets;
pub mod session_user_provider;
pub mod user;
pub mod user_binder;
pub mod worker;
