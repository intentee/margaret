use crate::include::Include;
use crate::preload::Preload;

pub trait RegistersIncludes {
    fn includes(&self) -> Vec<Include>;
    fn output_preloads(&self) -> Vec<Preload>;
    fn preloads(&self) -> &'static [Preload];
}
