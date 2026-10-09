use margaret_registered_claims::numeric_date::NumericDate;

#[must_use]
pub fn fixture_now() -> NumericDate {
    NumericDate::new(1_700_000_000)
}
