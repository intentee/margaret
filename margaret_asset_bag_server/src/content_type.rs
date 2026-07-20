use mime_guess::from_path;

#[must_use]
pub fn content_type(asset_path: &str) -> String {
    from_path(asset_path).first_or_octet_stream().to_string()
}

#[cfg(test)]
mod tests {
    use super::content_type;

    #[test]
    fn resolves_a_png_content_type_from_the_extension() {
        assert_eq!(content_type("assets/logo_ABC12345.png"), "image/png");
    }

    #[test]
    fn resolves_a_javascript_content_type_from_the_extension() {
        assert_eq!(content_type("assets/app_ABC12345.js"), "text/javascript");
    }

    #[test]
    fn falls_back_to_octet_stream_for_an_unknown_extension() {
        assert_eq!(
            content_type("assets/model_ABC12345.unknownext"),
            "application/octet-stream"
        );
    }
}
