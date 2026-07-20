#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AssetHref {
    Absolute(&'static str),
    Local(&'static str),
}

impl AssetHref {
    #[must_use]
    pub fn url(&self) -> String {
        match self {
            AssetHref::Absolute(url) => (*url).to_string(),
            AssetHref::Local(path) => format!("/{path}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AssetHref;

    #[test]
    fn prefixes_a_local_path_with_a_root_slash() {
        assert_eq!(
            AssetHref::Local("assets/app_ABC12345.js").url(),
            "/assets/app_ABC12345.js"
        );
    }

    #[test]
    fn passes_an_absolute_url_through_unchanged() {
        assert_eq!(
            AssetHref::Absolute("https://fonts.example/font.woff2").url(),
            "https://fonts.example/font.woff2"
        );
    }
}
