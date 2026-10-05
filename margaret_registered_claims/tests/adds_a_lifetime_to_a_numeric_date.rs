use std::time::Duration;

use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn adds_a_lifetime_to_a_numeric_date() {
    assert_eq!(
        NumericDate::new(1_000).after(Duration::from_mins(1)),
        NumericDate::new(1_060)
    );
}
