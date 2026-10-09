use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ext::IdentExt as _;

use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_field::ModelField;

use crate::field_identifier::field_identifier;
use crate::generated_model_module::generated_model_module;
use crate::generated_model_path::generated_model_path;

pub(crate) struct NodeLocation {
    segments: Vec<Ident>,
}

impl NodeLocation {
    pub(crate) fn query() -> Self {
        Self {
            segments: vec![format_ident!("query")],
        }
    }

    pub(crate) fn child(&self, field: &ModelField) -> Self {
        self.then().nested(field_identifier(field))
    }

    pub(crate) fn edge(&self) -> Self {
        self.nested(format_ident!("edge"))
    }

    pub(crate) fn edge_of(&self, field: &ModelField) -> Self {
        self.edge().nested(field_identifier(field))
    }

    pub(crate) fn module(&self, model: &Model) -> String {
        generated_model_module(
            model,
            &self
                .segments
                .iter()
                .map(|segment| segment.unraw().to_string())
                .collect::<Vec<String>>()
                .join("/"),
        )
    }

    pub(crate) fn path(&self, model: &Model) -> TokenStream {
        let generated = generated_model_path(model);
        let segments = &self.segments;

        quote! { #generated #(::#segments)* }
    }

    pub(crate) fn then(&self) -> Self {
        self.nested(format_ident!("then"))
    }

    fn nested(&self, segment: Ident) -> Self {
        let mut segments = self.segments.clone();

        segments.push(segment);

        Self { segments }
    }
}
