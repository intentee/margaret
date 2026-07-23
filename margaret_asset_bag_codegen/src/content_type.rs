use std::path::Path;

pub(crate) fn content_type(output_path: &str) -> &'static str {
    let extension = Path::new(output_path)
        .extension()
        .and_then(|extension| extension.to_str());

    match extension {
        Some("avif") => "image/avif",
        Some("css") => "text/css",
        Some("gif") => "image/gif",
        Some("glb") => "model/gltf-binary",
        Some("ico") => "image/x-icon",
        Some("jpeg" | "jpg") => "image/jpeg",
        Some("js" | "mjs") => "text/javascript",
        Some("json") => "application/json",
        Some("otf") => "font/otf",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ttf") => "font/ttf",
        Some("wasm") => "application/wasm",
        Some("webp") => "image/webp",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::content_type;

    #[test]
    fn maps_each_supported_output_extension_to_its_content_type() {
        let cases = [
            ("assets/hero_HASH.avif", "image/avif"),
            ("assets/app_HASH.css", "text/css"),
            ("assets/anim_HASH.gif", "image/gif"),
            ("assets/scene_HASH.glb", "model/gltf-binary"),
            ("assets/favicon_HASH.ico", "image/x-icon"),
            ("assets/photo_HASH.jpeg", "image/jpeg"),
            ("assets/photo_HASH.jpg", "image/jpeg"),
            ("assets/app_HASH.js", "text/javascript"),
            ("assets/data_HASH.json", "application/json"),
            ("assets/worker_HASH.mjs", "text/javascript"),
            ("assets/inter_HASH.otf", "font/otf"),
            ("assets/logo_HASH.png", "image/png"),
            ("assets/icon_HASH.svg", "image/svg+xml"),
            ("assets/inter_HASH.ttf", "font/ttf"),
            ("assets/module_HASH.wasm", "application/wasm"),
            ("assets/hero_HASH.webp", "image/webp"),
            ("assets/inter_HASH.woff", "font/woff"),
            ("assets/inter_HASH.woff2", "font/woff2"),
        ];

        for (output, expected) in cases {
            assert_eq!(
                content_type(output),
                expected,
                "unexpected content type for `{output}`"
            );
        }
    }

    #[test]
    fn falls_back_to_octet_stream_for_unrecognized_outputs() {
        let cases = ["assets/model_HASH.bin", "assets/LICENSE"];

        for output in cases {
            assert_eq!(
                content_type(output),
                "application/octet-stream",
                "unexpected content type for `{output}`"
            );
        }
    }
}
