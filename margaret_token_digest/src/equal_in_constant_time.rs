use aws_lc_rs::constant_time::verify_slices_are_equal;

#[must_use]
pub fn equal_in_constant_time(presented: &[u8], expected: &[u8]) -> bool {
    verify_slices_are_equal(presented, expected).is_ok()
}
