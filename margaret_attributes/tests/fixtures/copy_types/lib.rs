#[derive(Clone, Copy)]
struct DerivedCopy(u32);

#[derive(Clone, Copy)]
enum DerivedCopyEnum {
    First,
    Second,
}

struct ManuallyCopied(u32);

impl Clone for ManuallyCopied {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for ManuallyCopied {}

#[derive(Clone)]
struct ClonedOnly(String);
