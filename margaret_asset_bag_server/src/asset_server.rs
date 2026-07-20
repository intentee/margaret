use std::marker::PhantomData;

use rust_embed::RustEmbed;

use margaret_http::response::Response;

use crate::content_type::content_type;

const CACHE_CONTROL: &str = "public, max-age=31536000, immutable";

pub struct AssetServer<TEmbedded> {
    marker: PhantomData<TEmbedded>,
}

impl<TEmbedded> Default for AssetServer<TEmbedded> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<TEmbedded: RustEmbed> AssetServer<TEmbedded> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn respond(&self, asset_path: &str) -> Response {
        match TEmbedded::get(asset_path) {
            Some(embedded_file) => {
                Response::bytes(200, content_type(asset_path), embedded_file.data.into_owned())
                    .header("cache-control", CACHE_CONTROL)
            }
            None => Response::not_found(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_embed::RustEmbed;

    use super::AssetServer;

    #[derive(RustEmbed)]
    #[folder = "test_assets/"]
    struct TestAssets;

    #[test]
    fn serves_an_embedded_asset_with_an_ok_status() {
        assert_eq!(
            AssetServer::<TestAssets>::new()
                .respond("app_ABC12345.js")
                .status(),
            200
        );
    }

    #[test]
    fn responds_with_not_found_for_a_missing_asset() {
        assert_eq!(
            AssetServer::<TestAssets>::new()
                .respond("does-not-exist.js")
                .status(),
            404
        );
    }
}
