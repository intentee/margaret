use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn container_field_ident(position: usize) -> Ident {
    format_ident!("provider{position}")
}

#[cfg(test)]
mod tests {
    use super::container_field_ident;

    #[test]
    fn names_a_provider_after_its_construction_position() {
        assert_eq!(container_field_ident(3).to_string(), "provider3");
    }
}
