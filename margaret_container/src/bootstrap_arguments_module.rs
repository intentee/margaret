use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn bootstrap_arguments_module(function: &Ident) -> Ident {
    format_ident!("{function}_arguments")
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use super::bootstrap_arguments_module;

    #[test]
    fn names_the_module_after_its_bootstrap_function() {
        assert_eq!(
            bootstrap_arguments_module(&format_ident!("serve")).to_string(),
            "serve_arguments"
        );
    }
}
