use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Preparation {
    pub arguments: Vec<CanonicalPath>,
    pub function: CanonicalPath,
    pub prepared: CanonicalPath,
}

impl Preparation {
    #[must_use]
    pub fn call(&self) -> TokenStream {
        let function = path_tokens(&self.function);
        let arguments = self.arguments.iter().map(path_tokens);

        quote! { #function(#(#arguments),*) }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::Preparation;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    #[test]
    fn calls_its_function_with_every_constant_in_order() {
        assert_eq!(
            Preparation {
                arguments: vec![path(&["crate", "SUPPORT"]), path(&["crate", "ENDPOINTS"])],
                function: path(&["crate", "Handler", "create"]),
                prepared: path(&["crate", "Handler"]),
            }
            .call()
            .to_string()
            .split_whitespace()
            .collect::<String>(),
            "crate::Handler::create(crate::SUPPORT,crate::ENDPOINTS)"
        );
    }
}
