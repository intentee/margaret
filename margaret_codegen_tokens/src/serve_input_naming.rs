use proc_macro2::Ident;
use quote::format_ident;

#[derive(Clone, Copy)]
pub struct ServeInputNaming {
    slot_digits: usize,
}

impl ServeInputNaming {
    #[must_use]
    pub fn for_slot_count(slot_count: usize) -> Self {
        Self {
            slot_digits: slot_count.to_string().len(),
        }
    }

    #[must_use]
    pub fn ident(self, slot: usize) -> Ident {
        let slot_digits = self.slot_digits;

        format_ident!("serve_input_{slot:0slot_digits$}")
    }
}

#[cfg(test)]
mod tests {
    use super::ServeInputNaming;

    #[test]
    fn names_the_local_after_its_slot() {
        assert_eq!(
            ServeInputNaming::for_slot_count(4).ident(3).to_string(),
            "serve_input_3"
        );
    }

    #[test]
    fn pads_every_slot_to_the_digits_of_the_slot_count() {
        let naming = ServeInputNaming::for_slot_count(11);

        assert_eq!(naming.ident(1).to_string(), "serve_input_01");
        assert_eq!(naming.ident(10).to_string(), "serve_input_10");
    }
}
