use std::collections::BTreeSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::generated_feature::GeneratedFeature;

#[derive(Default)]
pub(crate) struct GeneratedFeatures {
    present: BTreeSet<GeneratedFeature>,
}

impl GeneratedFeatures {
    pub(crate) fn from_index(index: &AttributeIndex) -> Self {
        let mut features = Self::default();

        features.enable_if(
            GeneratedFeature::AuthenticatedUsers,
            index.has_framework_attribute(FrameworkAttribute::InfersAuthenticatedUser),
        );
        features.enable_if(
            GeneratedFeature::Http,
            index.has_framework_attribute(FrameworkAttribute::RespondsToHttp),
        );
        features.enable_if(
            GeneratedFeature::Middleware,
            index.has_framework_attribute(FrameworkAttribute::HandlesMiddlewareAttribute),
        );
        features.enable_if(
            GeneratedFeature::Schema,
            index.has_framework_attribute(FrameworkAttribute::Model),
        );
        features.enable_if(
            GeneratedFeature::Websockets,
            index.has_framework_attribute(FrameworkAttribute::WebsocketSession),
        );
        features.enable_if(
            GeneratedFeature::Views,
            index.has_framework_attribute(FrameworkAttribute::RendersView)
                && features.contains(GeneratedFeature::Http),
        );

        let has_services = index.has_framework_attribute(FrameworkAttribute::Service)
            || index.has_framework_attribute(FrameworkAttribute::ScheduledWithTickTimer);

        features.enable_if(
            GeneratedFeature::Serves,
            features.contains(GeneratedFeature::Http)
                || has_services
                || features.contains(GeneratedFeature::Websockets),
        );
        features.enable_if(
            GeneratedFeature::Console,
            index.has_framework_attribute(FrameworkAttribute::ConsoleCommand)
                || features.contains(GeneratedFeature::Serves)
                || features.contains(GeneratedFeature::Schema),
        );

        features
    }

    pub(crate) fn contains(&self, feature: GeneratedFeature) -> bool {
        self.present.contains(&feature)
    }

    pub(crate) fn enable(&mut self, feature: GeneratedFeature) {
        self.present.insert(feature);
    }

    pub(crate) fn enable_if(&mut self, feature: GeneratedFeature, present: bool) {
        if present {
            self.enable(feature);
        }
    }

    pub(crate) fn serves_http(&self) -> bool {
        self.contains(GeneratedFeature::Http) || self.contains(GeneratedFeature::Websockets)
    }
}
