use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::canonical_path::CanonicalPath;

pub struct FrameworkVocabulary<Variant: 'static> {
    pub enum_path: &'static [&'static str],
    pub name: fn(Variant) -> &'static str,
    pub variants: &'static [Variant],
}

impl<Variant: Copy> FrameworkVocabulary<Variant> {
    #[must_use]
    pub fn tokens(&self, variant: Variant) -> TokenStream {
        let segments = self
            .enum_path
            .iter()
            .copied()
            .chain([(self.name)(variant)])
            .map(|segment| format_ident!("{segment}"));

        quote! { #(#segments)::* }
    }

    #[must_use]
    pub fn variant(&self, resolved: &CanonicalPath) -> Option<Variant> {
        let (name, enum_path) = resolved.segments().split_last()?;

        if !enum_path
            .iter()
            .map(String::as_str)
            .eq(self.enum_path.iter().copied())
        {
            return None;
        }

        self.variants
            .iter()
            .copied()
            .find(|candidate| (self.name)(*candidate) == name)
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::FrameworkVocabulary;
    use crate::canonical_path::CanonicalPath;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Weather {
        Rain,
        Sun,
    }

    fn weather_name(weather: Weather) -> &'static str {
        match weather {
            Weather::Rain => "Rain",
            Weather::Sun => "Sun",
        }
    }

    const WEATHER: FrameworkVocabulary<Weather> = FrameworkVocabulary {
        enum_path: &["margaret", "framework", "weather", "Weather"],
        name: weather_name,
        variants: &[Weather::Rain, Weather::Sun],
    };

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    #[test]
    fn recognises_a_variant_of_the_framework_enum() {
        assert_eq!(
            WEATHER.variant(&path(&[
                "margaret",
                "framework",
                "weather",
                "Weather",
                "Sun"
            ])),
            Some(Weather::Sun)
        );
    }

    #[test]
    fn rejects_a_same_named_variant_of_a_foreign_enum() {
        assert_eq!(
            WEATHER.variant(&path(&["crate", "weather", "Weather", "Sun"])),
            None
        );
    }

    #[test]
    fn rejects_a_variant_the_framework_enum_does_not_declare() {
        assert_eq!(
            WEATHER.variant(&path(&[
                "margaret",
                "framework",
                "weather",
                "Weather",
                "Snow"
            ])),
            None
        );
    }

    #[test]
    fn rejects_an_empty_path() {
        assert_eq!(WEATHER.variant(&path(&[])), None);
    }

    #[test]
    fn renders_the_framework_path_of_a_variant() {
        assert_eq!(
            WEATHER.tokens(Weather::Rain).to_string(),
            quote! { margaret::framework::weather::Weather::Rain }.to_string()
        );
    }
}
