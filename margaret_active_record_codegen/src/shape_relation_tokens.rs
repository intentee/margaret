use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::field_span_tokens::field_span_tokens;
use crate::loaded_field_tokens::loaded_field_tokens;
use crate::loaded_target::LoadedTarget;
use crate::shape_read::ShapeRead;
use crate::shape_relation::ShapeRelation;
use crate::shape_relation_kind::ShapeRelationKind;

pub(crate) struct ShapeRelationTokens {
    pub(crate) joined_width: TokenStream,
    pub(crate) preloads: Vec<TokenStream>,
    pub(crate) reads: Vec<ShapeRead>,
    pub(crate) selects: Vec<TokenStream>,
}

impl ShapeRelationTokens {
    pub(crate) fn of(
        relations: &[ShapeRelation],
        model: &TokenStream,
        base_width: &TokenStream,
        joined_width: TokenStream,
    ) -> Self {
        let mut tokens = Self {
            joined_width,
            preloads: Vec::new(),
            reads: Vec::new(),
            selects: Vec::new(),
        };

        for relation in relations {
            tokens.push(relation, model, base_width);
        }

        tokens
    }

    fn push(&mut self, relation: &ShapeRelation, model: &TokenStream, base_width: &TokenStream) {
        let field = format_ident!("{}", relation.field);
        let field_type = loaded_field_tokens(relation);

        match relation.kind {
            ShapeRelationKind::BelongsTo { key } => {
                let span = field_span_tokens(key);
                let joined_width = &self.joined_width;

                self.preloads.push(match relation.target {
                    LoadedTarget::Model => quote! {
                        #field: margaret::framework::active_record::nothing_preloaded::NothingPreloaded
                    },
                    LoadedTarget::Shape => quote! {
                        #field: <#field_type as margaret::framework::active_record::loadable::Loadable>::preload(
                            rows,
                            offset + #joined_width,
                            executor,
                        )
                        .await?
                    },
                });
                self.reads.push(ShapeRead::Fallible {
                    read: quote! {
                        <#field_type as margaret::framework::active_record::loadable::Loadable>::read(
                            cursor,
                            &mut preloaded.#field,
                        )
                    },
                    field,
                });
                self.selects.push(quote! {
                    builder.belongs_to::<#field_type, #model>(source, #span);
                });
                self.joined_width = quote! {
                    #joined_width
                        + <#field_type as margaret::framework::active_record::loadable::Loadable>::WIDTH
                };
            }
            ShapeRelationKind::HasMany {
                limit,
                relation: declared,
            } => {
                let key = field_span_tokens(&declared.key);
                let order = declared
                    .covering_index
                    .fields
                    .iter()
                    .skip(1)
                    .map(field_span_tokens);

                self.preloads.push(quote! {
                    #field: margaret::framework::active_record::child_groups::ChildGroups::many(
                        rows,
                        offset + #base_width,
                        &margaret::framework::active_record::many_relation::ManyRelation {
                            key: #key,
                            limit: #limit,
                            order: &[#(#order),*],
                        },
                        executor,
                    )
                    .await?
                });
                self.reads.push(ShapeRead::Infallible {
                    value: quote! { preloaded.#field.children(cursor.index()) },
                    field,
                });
            }
            ShapeRelationKind::HasOne { relation: declared } => {
                let key = field_span_tokens(&declared.key);

                self.preloads.push(quote! {
                    #field: margaret::framework::active_record::child_groups::ChildGroups::one(
                        rows,
                        offset + #base_width,
                        &margaret::framework::active_record::one_relation::OneRelation { key: #key },
                        executor,
                    )
                    .await?
                });
                self.reads.push(ShapeRead::Infallible {
                    value: quote! { preloaded.#field.child(cursor.index()) },
                    field,
                });
            }
        }
    }
}
