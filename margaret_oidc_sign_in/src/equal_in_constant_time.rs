use aws_lc_rs::constant_time::verify_slices_are_equal;

pub(crate) fn equal_in_constant_time(presented: &str, expected: &str) -> bool {
    verify_slices_are_equal(presented.as_bytes(), expected.as_bytes()).is_ok()
}
