use syn::Fields;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructShape {
    Named { field_count: usize },
    Unit,
    Unnamed { field_count: usize },
}

impl From<&Fields> for StructShape {
    fn from(fields: &Fields) -> Self {
        match fields {
            Fields::Named(named) => StructShape::Named {
                field_count: named.named.len(),
            },
            Fields::Unit => StructShape::Unit,
            Fields::Unnamed(unnamed) => StructShape::Unnamed {
                field_count: unnamed.unnamed.len(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::ItemStruct;
    use syn::parse_str;

    use super::StructShape;

    fn shape_of(source: &str) -> StructShape {
        let item: ItemStruct = parse_str(source).expect("the struct fixture parses");

        StructShape::from(&item.fields)
    }

    #[test]
    fn reads_a_unit_struct() {
        assert_eq!(shape_of("struct Marker;"), StructShape::Unit);
    }

    #[test]
    fn reads_a_named_struct() {
        assert_eq!(
            shape_of("struct Pair { left: u8, right: u8 }"),
            StructShape::Named { field_count: 2 }
        );
    }

    #[test]
    fn reads_a_tuple_struct() {
        assert_eq!(
            shape_of("struct Wrapper(u8);"),
            StructShape::Unnamed { field_count: 1 }
        );
    }
}
