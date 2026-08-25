#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckPredicate {
    ByteLength { length: u32 },
    Minimum { minimum: u32 },
}
