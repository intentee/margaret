use crate::asset_href::AssetHref;
use crate::has_stylesheet::HasStylesheet;
use crate::include::Include;
use crate::preload::Preload;
use crate::registers_includes::RegistersIncludes;

#[derive(Clone, Copy)]
pub struct StylesheetBundle {
    href: AssetHref,
    preloads: &'static [Preload],
}

impl StylesheetBundle {
    #[must_use]
    pub const fn new(href: AssetHref, preloads: &'static [Preload]) -> Self {
        Self { href, preloads }
    }
}

impl HasStylesheet for StylesheetBundle {
    fn stylesheet_include(&self) -> Include {
        Include::Stylesheet(self.href)
    }

    fn stylesheet_output_preload(&self) -> Preload {
        Preload::Style(self.href)
    }

    fn stylesheet_preloads(&self) -> &'static [Preload] {
        self.preloads
    }
}

impl RegistersIncludes for StylesheetBundle {
    fn includes(&self) -> Vec<Include> {
        vec![self.stylesheet_include()]
    }

    fn output_preloads(&self) -> Vec<Preload> {
        vec![self.stylesheet_output_preload()]
    }

    fn preloads(&self) -> &'static [Preload] {
        self.stylesheet_preloads()
    }
}
