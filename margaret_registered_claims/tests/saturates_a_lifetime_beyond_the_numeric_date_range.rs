use std::time::Duration;

use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn saturates_a_lifetime_beyond_the_numeric_date_range() {
    assert_eq!(
        NumericDate::new(1_000).after(Duration::MAX),
        NumericDate::new(i64::MAX)
    );
}
