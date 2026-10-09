use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::bound_read::BoundRead;
use crate::chained_construction::chained_construction;
use crate::key_block_tokens::KeyBlockTokens;
use crate::loaded_field_tokens::loaded_field_tokens;
use crate::loaded_target::LoadedTarget;
use crate::shape_declaration::ShapeDeclaration;
use crate::shape_read::ShapeRead;
use crate::shape_relation::ShapeRelation;
use crate::shape_relation_kind::ShapeRelationKind;
use crate::shape_relation_tokens::ShapeRelationTokens;
use crate::shapes_module_name::SHAPES_MODULE_NAME;
use crate::value_identifier::value_identifier;

fn preloaded_type(relation: &ShapeRelation) -> TokenStream {
    let loaded = path_tokens(&relation.loaded);
    let field_type = loaded_field_tokens(relation);

    match relation.kind {
        ShapeRelationKind::BelongsTo { .. } => quote! {
            <#field_type as margaret::framework::active_record::loadable::Loadable>::Preloaded
        },
        ShapeRelationKind::HasMany { .. } | ShapeRelationKind::HasOne { .. } => {
            quote! { margaret::framework::active_record::child_groups::ChildGroups<#loaded> }
        }
    }
}

fn preload_parameters(shape: &ShapeDeclaration, loads_children: bool) -> TokenStream {
    if loads_children
        || shape
            .relations
            .iter()
            .any(|relation| relation.target == LoadedTarget::Shape)
    {
        quote! {
            rows: &margaret::framework::active_record::loaded_rows::LoadedRows<'_>,
            offset: usize,
            executor: &Executing,
        }
    } else {
        quote! {
            _: &margaret::framework::active_record::loaded_rows::LoadedRows<'_>,
            _: usize,
            _: &Executing,
        }
    }
}

fn shape_read(base: &Ident, read_base: TokenStream, reads: Vec<ShapeRead>) -> TokenStream {
    let base_value = value_identifier(0);
    let mut bound = vec![BoundRead {
        read: read_base,
        value: base_value.clone(),
    }];
    let mut assignments = vec![quote! { #base: #base_value }];

    for shape_read in reads {
        match shape_read {
            ShapeRead::Fallible { field, read } => {
                let value = value_identifier(bound.len());

                assignments.push(quote! { #field: #value });
                bound.push(BoundRead { read, value });
            }
            ShapeRead::Infallible { field, value } => {
                assignments.push(quote! { #field: #value });
            }
        }
    }

    chained_construction(&bound, &quote! { Self { #(#assignments),* } })
}

pub(crate) fn render_shape(shape: &ShapeDeclaration) -> GeneratedModuleTokens {
    let shape_type = path_tokens(&shape.path);
    let model = path_tokens(&shape.model.path);
    let base = format_ident!("{}", shape.base);
    let module = format_ident!("{}", shape.module);
    let shapes = format_ident!("{SHAPES_MODULE_NAME}");
    let loads_children = shape.relations.iter().any(|relation| {
        matches!(
            relation.kind,
            ShapeRelationKind::HasMany { .. } | ShapeRelationKind::HasOne { .. }
        )
    });
    let preload_parameters = preload_parameters(shape, loads_children);
    let base_width = quote! {
        <#model as margaret::framework::active_record::loadable::Loadable>::WIDTH
    };
    let KeyBlockTokens {
        read_base,
        select: select_key,
        width: key_width,
    } = KeyBlockTokens::of(&model, loads_children);
    let preloaded_fields = shape.relations.iter().map(|relation| {
        let field = format_ident!("{}", relation.field);
        let preloaded = preloaded_type(relation);

        quote! { pub #field: #preloaded }
    });
    let ShapeRelationTokens {
        joined_width,
        preloads,
        reads,
        selects,
    } = ShapeRelationTokens::of(
        &shape.relations,
        &model,
        &base_width,
        quote! { #base_width #key_width },
    );
    let read = shape_read(&base, read_base, reads);

    GeneratedModuleTokens::new(
        format!("{SHAPES_MODULE_NAME}/{}", shape.module),
        quote! {
            pub struct Preloaded {
                #(#preloaded_fields),*
            }

            impl margaret::framework::active_record::loadable::Loadable for #shape_type {
                type Preloaded = crate::margaret::#shapes::#module::Preloaded;
                type Root = #model;

                const JOIN: margaret::framework::active_record::join_context::JoinContext =
                    margaret::framework::active_record::join_context::JoinContext::Required;

                const WIDTH: usize = #joined_width;

                async fn preload<Executing: margaret::framework::database::executor::Executor>(
                    #preload_parameters
                ) -> ::std::result::Result<
                    Self::Preloaded,
                    margaret::framework::active_record::active_record_error::ActiveRecordError,
                > {
                    ::std::result::Result::Ok(crate::margaret::#shapes::#module::Preloaded {
                        #(#preloads),*
                    })
                }

                fn read(
                    cursor: &mut margaret::framework::active_record::load_cursor::LoadCursor<'_>,
                    preloaded: &mut Self::Preloaded,
                ) -> ::std::result::Result<
                    Self,
                    margaret::framework::active_record::active_record_error::ActiveRecordError,
                > {
                    #read
                }

                fn select(
                    builder: &mut margaret::framework::active_record::select_builder::SelectBuilder,
                    source: margaret::framework::active_record::joined_source::JoinedSource,
                ) {
                    builder.record::<#model>(source);
                    #select_key
                    #(#selects)*
                }
            }

            impl margaret::framework::active_record::shape::Shape for #shape_type {}
        },
    )
}
