use crate::include::Include;
use crate::preload::Preload;

pub trait HasStylesheet {
    fn stylesheet_include(&self) -> Include;
    fn stylesheet_output_preload(&self) -> Preload;
    fn stylesheet_preloads(&self) -> &'static [Preload];
}
