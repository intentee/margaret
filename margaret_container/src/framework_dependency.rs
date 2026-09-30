use margaret_attributes::canonical_path::CanonicalPath;

pub enum FrameworkDependency {
    BorrowedProvider(CanonicalPath),
    BorrowedTokenIssuance,
    Constant(CanonicalPath),
    Provider(CanonicalPath),
    Providers(Vec<CanonicalPath>),
    SingletonView(CanonicalPath),
    SingletonViews {
        singletons: Vec<CanonicalPath>,
        view: CanonicalPath,
    },
    TokenIssuance,
}
