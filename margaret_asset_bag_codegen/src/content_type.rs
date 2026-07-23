use std::path::Path;

use crate::asset_bag_codegen_error::AssetBagCodegenError;

pub(crate) fn content_type(output_path: &str) -> Result<&'static str, AssetBagCodegenError> {
    let extension = Path::new(output_path)
        .extension()
        .and_then(|extension| extension.to_str());

    match extension {
        Some("js" | "mjs") => Ok("text/javascript"),
        Some("css") => Ok("text/css"),
        Some("json") => Ok("application/json"),
        Some("wasm") => Ok("application/wasm"),
        Some("png") => Ok("image/png"),
        Some("jpg" | "jpeg") => Ok("image/jpeg"),
        Some("gif") => Ok("image/gif"),
        Some("webp") => Ok("image/webp"),
        Some("avif") => Ok("image/avif"),
        Some("svg") => Ok("image/svg+xml"),
        Some("ico") => Ok("image/x-icon"),
        Some("woff") => Ok("font/woff"),
        Some("woff2") => Ok("font/woff2"),
        Some("ttf") => Ok("font/ttf"),
        Some("otf") => Ok("font/otf"),
        _ => Err(AssetBagCodegenError::UnsupportedAssetContentType {
            output: output_path.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::content_type;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    #[test]
    fn maps_each_supported_output_extension_to_its_content_type() {
        let cases = [
            ("assets/app_HASH.js", "text/javascript"),
            ("assets/worker_HASH.mjs", "text/javascript"),
            ("assets/app_HASH.css", "text/css"),
            ("assets/data_HASH.json", "application/json"),
            ("assets/module_HASH.wasm", "application/wasm"),
            ("assets/logo_HASH.png", "image/png"),
            ("assets/photo_HASH.jpg", "image/jpeg"),
            ("assets/photo_HASH.jpeg", "image/jpeg"),
            ("assets/anim_HASH.gif", "image/gif"),
            ("assets/hero_HASH.webp", "image/webp"),
            ("assets/hero_HASH.avif", "image/avif"),
            ("assets/icon_HASH.svg", "image/svg+xml"),
            ("assets/favicon_HASH.ico", "image/x-icon"),
            ("assets/inter_HASH.woff", "font/woff"),
            ("assets/inter_HASH.woff2", "font/woff2"),
            ("assets/inter_HASH.ttf", "font/ttf"),
            ("assets/inter_HASH.otf", "font/otf"),
        ];

        for (output, expected) in cases {
            assert_eq!(
                content_type(output).expect("the extension is supported"),
                expected,
                "unexpected content type for `{output}`"
            );
        }
    }

    #[test]
    fn rejects_an_output_with_an_unrecognized_extension() {
        assert!(matches!(
            content_type("assets/model_HASH.bin"),
            Err(AssetBagCodegenError::UnsupportedAssetContentType { output })
                if output == "assets/model_HASH.bin"
        ));
    }

    #[test]
    fn rejects_an_output_without_an_extension() {
        assert!(matches!(
            content_type("assets/LICENSE"),
            Err(AssetBagCodegenError::UnsupportedAssetContentType { output })
                if output == "assets/LICENSE"
        ));
    }
}
