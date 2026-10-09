use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;

const CRATE_ROOT: &str = "crate";

fn segment_identifiers(segments: &[String]) -> impl Iterator<Item = Ident> {
    segments.iter().map(|segment| format_ident!("{}", segment))
}

#[must_use]
pub fn path_tokens(path: &CanonicalPath) -> TokenStream {
    match path.segments() {
        [first, rest @ ..] if first == CRATE_ROOT => {
            let rest = segment_identifiers(rest);

            quote! { crate #(:: #rest)* }
        }
        foreign @ [_, _, ..] => {
            let foreign = segment_identifiers(foreign);

            quote! { #(:: #foreign)* }
        }
        primitive => {
            let primitive = segment_identifiers(primitive);

            quote! { #(#primitive)* }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::path_tokens;

    fn rendered(segments: &[&str]) -> String {
        let path = CanonicalPath::new(segments.iter().map(ToString::to_string).collect());

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
    fn renders_a_foreign_crate_root_as_an_absolute_path() {
        assert_eq!(
            rendered(&["margaret_http", "response", "Response"]),
            ":: margaret_http :: response :: Response"
        );
    }

    #[test]
    fn renders_a_primitive_type_bare() {
        assert_eq!(rendered(&["u8"]), "u8");
    }

    #[test]
    fn renders_a_path_without_segments_as_nothing() {
        assert_eq!(rendered(&[]), "");
    }
}
