#[singleton]
struct RootStruct;

#[singleton]
struct WithProvides;

#[singleton]
enum RootEnum {}

#[responds_to_http(method = Get, pattern = "/home", count = 5, Marker, (grouped) = 1, qualified::path = 2)]
fn root_function() {}

trait RootTrait {}

#[doc = "name value form"]
struct DocCommented;

#[bad_args(= 5)]
struct BadArgs;

#[ns::tagged]
struct Qualified;

#[singleton]
#[singleton]
struct RepeatedAttrs;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Result as IoResult;

const ROOT_CONST: u8 = 0;
type RootAlias = u8;

mod inline_module {
    #[singleton]
    struct InsideInline;
}

mod file_module;
mod dir_module;

struct WithConstructor;

impl WithConstructor {
    #[constructor]
    fn new(root: Arc<RootStruct>) -> Self {}

    fn other(&self) {}

    const UNUSED: u8 = 0;
}

struct AnotherService;

impl AnotherService {
    fn build() {}
}

trait SomeTrait {}

impl SomeTrait for WithConstructor {
    fn skipped_trait_method(&self) {}
}

impl (RootStruct, RootEnum) {
    fn skipped_tuple_method() {}
}

impl Undeclared {
    fn orphaned_method() {}
}

impl HashMap {
    fn skipped_imported_method(&self) {}
}
