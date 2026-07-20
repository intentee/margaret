use crate::asset_href::AssetHref;
use crate::has_script::HasScript;
use crate::has_stylesheet::HasStylesheet;
use crate::include::Include;
use crate::preload::Preload;
use crate::registers_includes::RegistersIncludes;

#[derive(Clone, Copy)]
pub struct ScriptStylesheetBundle {
    script_href: AssetHref,
    stylesheet_href: AssetHref,
    preloads: &'static [Preload],
}

impl ScriptStylesheetBundle {
    #[must_use]
    pub const fn new(
        script_href: AssetHref,
        stylesheet_href: AssetHref,
        preloads: &'static [Preload],
    ) -> Self {
        Self {
            script_href,
            stylesheet_href,
            preloads,
        }
    }
}

impl HasScript for ScriptStylesheetBundle {
    fn script_include(&self) -> Include {
        Include::Script(self.script_href)
    }

    fn script_output_preload(&self) -> Preload {
        Preload::Module(self.script_href)
    }

    fn script_preloads(&self) -> &'static [Preload] {
        self.preloads
    }
}

impl HasStylesheet for ScriptStylesheetBundle {
    fn stylesheet_include(&self) -> Include {
        Include::Stylesheet(self.stylesheet_href)
    }

    fn stylesheet_output_preload(&self) -> Preload {
        Preload::Style(self.stylesheet_href)
    }

    fn stylesheet_preloads(&self) -> &'static [Preload] {
        self.preloads
    }
}

impl RegistersIncludes for ScriptStylesheetBundle {
    fn includes(&self) -> Vec<Include> {
        vec![self.script_include(), self.stylesheet_include()]
    }

    fn output_preloads(&self) -> Vec<Preload> {
        vec![self.script_output_preload(), self.stylesheet_output_preload()]
    }

    fn preloads(&self) -> &'static [Preload] {
        self.preloads
    }
}
