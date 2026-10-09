use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_model_codegen::model::Model;

use crate::generated_model_module::generated_model_module;
use crate::generated_model_path::generated_model_path;

pub(crate) struct NodeLocation {
    segments: Vec<String>,
}

impl NodeLocation {
    pub(crate) fn query() -> Self {
        Self {
            segments: vec!["query".to_string()],
        }
    }

    pub(crate) fn child(&self, field: &str) -> Self {
        let mut location = self.then();

        location.segments.push(field.to_string());
        location
    }

    pub(crate) fn edge(&self) -> Self {
        let mut segments = self.segments.clone();

        segments.push("edge".to_string());

        Self { segments }
    }

    pub(crate) fn edge_of(&self, field: &str) -> Self {
        let mut location = self.edge();

        location.segments.push(field.to_string());
        location
    }

    pub(crate) fn module(&self, model: &Model) -> String {
        generated_model_module(model, &self.segments.join("/"))
    }

    pub(crate) fn path(&self, model: &Model) -> TokenStream {
        let generated = generated_model_path(model);
        let segments = self
            .segments
            .iter()
            .map(|segment| format_ident!("{segment}"));

        quote! { #generated #(::#segments)* }
    }

    pub(crate) fn then(&self) -> Self {
        let mut segments = self.segments.clone();

        segments.push("then".to_string());

        Self { segments }
    }
}
