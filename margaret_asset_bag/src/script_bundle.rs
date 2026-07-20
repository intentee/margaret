use crate::asset_href::AssetHref;
use crate::has_script::HasScript;
use crate::include::Include;
use crate::preload::Preload;
use crate::registers_includes::RegistersIncludes;

#[derive(Clone, Copy)]
pub struct ScriptBundle {
    href: AssetHref,
    preloads: &'static [Preload],
}

impl ScriptBundle {
    #[must_use]
    pub const fn new(href: AssetHref, preloads: &'static [Preload]) -> Self {
        Self { href, preloads }
    }
}

impl HasScript for ScriptBundle {
    fn script_include(&self) -> Include {
        Include::Script(self.href)
    }

    fn script_output_preload(&self) -> Preload {
        Preload::Module(self.href)
    }

    fn script_preloads(&self) -> &'static [Preload] {
        self.preloads
    }
}

impl RegistersIncludes for ScriptBundle {
    fn includes(&self) -> Vec<Include> {
        vec![self.script_include()]
    }

    fn output_preloads(&self) -> Vec<Preload> {
        vec![self.script_output_preload()]
    }

    fn preloads(&self) -> &'static [Preload] {
        self.script_preloads()
    }
}
