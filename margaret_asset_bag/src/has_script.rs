use crate::include::Include;
use crate::preload::Preload;

pub trait HasScript {
    fn script_include(&self) -> Include;
    fn script_output_preload(&self) -> Preload;
    fn script_preloads(&self) -> &'static [Preload];
}
