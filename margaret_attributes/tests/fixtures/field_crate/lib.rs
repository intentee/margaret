struct NamedFields {
    #[column(name = "first_column")]
    first: String,
    second: u32,
}

struct TupleFields(String, u32);

struct UnitStruct;

enum Variants {
    Unit,
    Tuple(String, u32),
    Named { first: String },
}
