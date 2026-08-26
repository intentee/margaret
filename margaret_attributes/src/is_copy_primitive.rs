#[must_use]
pub(crate) fn is_copy_primitive(name: &str) -> bool {
    matches!(
        name,
        "bool"
            | "char"
            | "f32"
            | "f64"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
    )
}

#[cfg(test)]
mod tests {
    use super::is_copy_primitive;

    #[test]
    fn recognizes_every_copy_primitive() {
        for primitive in [
            "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16",
            "u32", "u64", "u128", "usize",
        ] {
            assert!(is_copy_primitive(primitive));
        }
    }

    #[test]
    fn rejects_a_non_primitive_type() {
        assert!(!is_copy_primitive("String"));
    }
}
