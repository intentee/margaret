use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn console_argument_field_ident(slot: usize) -> Ident {
    format_ident!("argument{slot}")
}

#[cfg(test)]
mod tests {
    use super::console_argument_field_ident;

    #[test]
    fn names_the_field_after_its_slot() {
        assert_eq!(console_argument_field_ident(3).to_string(), "argument3");
    }
}
