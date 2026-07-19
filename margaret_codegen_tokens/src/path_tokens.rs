use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn path_tokens(path: &CanonicalPath) -> TokenStream {
    let mut segments = path.segments().iter();
    let Some(first) = segments.next() else {
        return TokenStream::new();
    };
    let rest = segments.map(|segment| format_ident!("{}", segment));

    if first == "crate" {
        quote! { crate #(:: #rest)* }
    } else {
        let crate_name = format_ident!("{}", first);

        quote! { #crate_name #(:: #rest)* }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::path_tokens;

    fn rendered(segments: &[&str]) -> String {
        let path = CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect());

        path_tokens(&path).to_string()
    }

    #[test]
    fn renders_the_host_crate_root_as_the_crate_keyword() {
        assert_eq!(
            rendered(&["crate", "routes", "GetGreeting"]),
            "crate :: routes :: GetGreeting"
        );
    }

    #[test]
    fn renders_a_foreign_crate_root_as_its_crate_name() {
        assert_eq!(
            rendered(&["margaret_http", "response", "Response"]),
            "margaret_http :: response :: Response"
        );
    }

    #[test]
    fn renders_a_single_segment_foreign_root() {
        assert_eq!(rendered(&["other"]), "other");
    }

    #[test]
    fn renders_a_path_without_segments_as_nothing() {
        assert_eq!(rendered(&[]), "");
    }
}
