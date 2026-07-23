use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn console_argument_ident(slot: usize) -> Ident {
    format_ident!("console_argument_{slot}")
}

#[cfg(test)]
mod tests {
    use super::console_argument_ident;

    #[test]
    fn names_the_local_after_its_slot() {
        assert_eq!(console_argument_ident(3).to_string(), "console_argument_3");
    }
}
