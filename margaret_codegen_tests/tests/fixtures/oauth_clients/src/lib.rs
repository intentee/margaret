include!(concat!(env!("OUT_DIR"), "/margaret.rs"));

pub mod artifact_claims;
pub mod artifact_uploader;
pub mod artifact_uploader_provider;
pub mod partner_client;
pub mod partner_issuer;
pub mod token_refresher;
pub mod upload_page;
