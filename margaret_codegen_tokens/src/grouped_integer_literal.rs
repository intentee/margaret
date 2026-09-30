use proc_macro2::Span;
use syn::LitInt;

const DIGIT_GROUP_WIDTH: usize = 3;
const DIGIT_GROUP_SEPARATOR: char = '_';

#[must_use]
pub fn grouped_integer_literal(value: u64) -> LitInt {
    let digits = value.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / DIGIT_GROUP_WIDTH);

    for (position, digit) in digits.chars().enumerate() {
        if position > 0 && (digits.len() - position).is_multiple_of(DIGIT_GROUP_WIDTH) {
            grouped.push(DIGIT_GROUP_SEPARATOR);
        }

        grouped.push(digit);
    }

    LitInt::new(&grouped, Span::call_site())
}

#[cfg(test)]
mod tests {
    use super::grouped_integer_literal;

    #[test]
    fn groups_the_digits_of_a_large_integer_in_threes() {
        assert_eq!(grouped_integer_literal(8_388_608).to_string(), "8_388_608");
    }

    #[test]
    fn leaves_a_short_integer_ungrouped() {
        assert_eq!(grouped_integer_literal(512).to_string(), "512");
    }

    #[test]
    fn keeps_the_value_of_the_grouped_literal() {
        assert_eq!(
            grouped_integer_literal(1_073_741_824)
                .base10_parse::<u64>()
                .expect("the grouped literal is an integer"),
            1_073_741_824
        );
    }
}
