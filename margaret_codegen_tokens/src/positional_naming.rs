use proc_macro2::Ident;
use quote::format_ident;

#[derive(Clone, Copy)]
pub struct PositionalNaming {
    digits: usize,
    prefix: &'static str,
}

impl PositionalNaming {
    #[must_use]
    pub fn route_parameters(parameter_count: usize) -> Self {
        Self::of("parameter", parameter_count)
    }

    #[must_use]
    pub fn serve_inputs(slot_count: usize) -> Self {
        Self::of("serve_input", slot_count)
    }

    #[must_use]
    pub fn ident(self, position: usize) -> Ident {
        let Self { digits, prefix } = self;

        format_ident!("{prefix}_{position:0digits$}")
    }

    fn of(prefix: &'static str, count: usize) -> Self {
        Self {
            digits: count.to_string().len(),
            prefix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PositionalNaming;

    #[test]
    fn names_the_serve_input_local_after_its_slot_so_each_input_binding_is_distinct() {
        assert_eq!(
            PositionalNaming::serve_inputs(4).ident(3).to_string(),
            "serve_input_3"
        );
    }

    #[test]
    fn names_a_route_parameter_after_its_position_in_the_path() {
        assert_eq!(
            PositionalNaming::route_parameters(2).ident(1).to_string(),
            "parameter_1"
        );
    }

    #[test]
    fn pads_every_position_to_the_digits_of_the_count() {
        let naming = PositionalNaming::serve_inputs(11);

        assert_eq!(naming.ident(1).to_string(), "serve_input_01");
        assert_eq!(naming.ident(10).to_string(), "serve_input_10");
    }
}
