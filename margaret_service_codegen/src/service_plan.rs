use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_console_argument_codegen::owned_weave::owned_weave;
use margaret_container::container_bindings::ContainerBindings;
use proc_macro2::TokenStream;
use quote::quote;

use crate::framework_service::FrameworkService;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;

pub struct ServicePlan {
    pub(crate) construction_arguments: Vec<TokenStream>,
    pub(crate) has_spiffe_http_client: bool,
    pub(crate) prelude: TokenStream,
    pub(crate) units: Vec<ServiceUnit>,
    roots: Vec<CanonicalPath>,
}

impl ServicePlan {
    pub fn build(
        index: &AttributeIndex,
        framework_services: &[FrameworkService],
        bindings: &ContainerBindings,
        serve_arguments: &[ConsoleArgument],
    ) -> Result<Self, ServiceCodegenError> {
        let mut units = service_units(index)?;
        units.extend(framework_services.iter().map(ServiceUnit::from_framework));
        let roots = units
            .iter()
            .map(|unit| unit.concrete_path.clone())
            .collect();
        let (prelude, construction_arguments) = serve_inputs(serve_arguments, bindings)?;

        Ok(Self {
            construction_arguments,
            has_spiffe_http_client: has_spiffe_http_client(serve_arguments),
            prelude,
            roots,
            units,
        })
    }

    #[must_use]
    pub fn roots(&self) -> &[CanonicalPath] {
        &self.roots
    }
}

fn serve_inputs(
    serve_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> Result<(TokenStream, Vec<TokenStream>), ServiceCodegenError> {
    let mut resolutions = Vec::with_capacity(serve_arguments.len());
    let mut construction_arguments = Vec::with_capacity(serve_arguments.len());

    for argument in serve_arguments {
        let slot = bindings.console_slot(&argument.slot_key())?;
        let ident = console_argument_ident(slot);
        let value = argument_value(argument);

        resolutions.push(quote! { let #ident = #value; });
        construction_arguments.push(owned_weave(argument, slot, true));
    }

    Ok((quote! { #(#resolutions)* }, construction_arguments))
}
