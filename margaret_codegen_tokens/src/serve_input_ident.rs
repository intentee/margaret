use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn serve_input_ident(slot: usize) -> Ident {
    format_ident!("serve_input_{slot}")
}

#[cfg(test)]
mod tests {
    use super::serve_input_ident;

    #[test]
    fn names_the_local_after_its_slot() {
        assert_eq!(serve_input_ident(3).to_string(), "serve_input_3");
    }
}
