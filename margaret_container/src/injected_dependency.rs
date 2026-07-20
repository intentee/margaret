use margaret_attributes::canonical_path::CanonicalPath;

pub enum InjectedDependency {
    Collection {
        member_fields: Vec<String>,
        trait_path: CanonicalPath,
    },
    SingleConcrete {
        concrete: CanonicalPath,
        field: String,
    },
    SingleInterface {
        field: String,
        interface: CanonicalPath,
    },
}
