use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn container_field_ident(position: usize, retained: bool) -> Ident {
    if retained {
        format_ident!("provider{position}")
    } else {
        format_ident!("_provider{position}")
    }
}

#[cfg(test)]
mod tests {
    use super::container_field_ident;

    #[test]
    fn names_a_retained_provider_after_its_construction_position() {
        assert_eq!(container_field_ident(3, true).to_string(), "provider3");
    }

    #[test]
    fn hides_a_provider_that_no_accessor_reads() {
        assert_eq!(container_field_ident(3, false).to_string(), "_provider3");
    }
}
