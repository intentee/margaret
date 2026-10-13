use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::field_identifier::field_identifier;
use crate::field_span_tokens::field_span_tokens;
use crate::field_type_tokens::field_type_tokens;
use crate::node_ending::NodeEnding;
use crate::node_location::NodeLocation;
use crate::node_ordering::NodeOrdering;
use crate::node_update_tokens::node_update_tokens;
use crate::query_edge::QueryEdge;
use crate::query_edges::QueryEdges;
use crate::query_node::QueryNode;
use crate::trie_builder::TrieBuilder;

fn branch_step_module(
    edges: &QueryEdges,
    location: &NodeLocation,
    model: &Model,
) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let all: Vec<&QueryEdge> = edges.all().collect();
    let step = step_declaration(
        &all,
        location,
        model,
        &quote! { margaret::framework::active_record::narrowed::Narrowed<#record> },
    );
    let first = field_identifier(edges.first.field);
    let first_start = edges.first.field.start;
    let rest = edges.rest.iter().map(|edge| {
        let identifier = field_identifier(edge.field);
        let start = edge.field.start;

        quote! {
            #identifier: margaret::framework::active_record::next::Next::new(
                ::std::clone::Clone::clone(&narrowed),
                #start,
            )
        }
    });
    let update_impl = node_update_tokens(model, &first);

    GeneratedModuleTokens::new(
        location.step().module(model),
        quote! {
            #step

            impl Step {
                #[doc = " # Errors"]
                #[doc = ""]
                #[doc = " Returns `ActiveRecordError` when the rows cannot be deleted."]
                pub async fn delete<Executing: margaret::framework::database::executor::Executor>(
                    self,
                    executor: &Executing,
                ) -> ::std::result::Result<
                    u64,
                    margaret::framework::active_record::active_record_error::ActiveRecordError,
                > {
                    margaret::framework::active_record::prefix::Prefix::new(self.#first.into_narrowed())
                        .delete(executor)
                        .await
                }
            }

            #update_impl

            impl margaret::framework::active_record::continuation::Continuation<#record> for Step {
                fn continued(
                    narrowed: margaret::framework::active_record::narrowed::Narrowed<#record>,
                ) -> Self {
                    Self {
                        #(#rest,)*
                        #first: margaret::framework::active_record::next::Next::new(narrowed, #first_start),
                    }
                }
            }
        },
    )
}

fn continuation_modules(
    edge: &QueryEdge,
    location: &NodeLocation,
    model: &Model,
) -> Vec<GeneratedModuleTokens> {
    match &edge.node {
        QueryNode::Branch(edges) => {
            let continued: Vec<&QueryEdge> = edges.all().collect();
            let mut modules = vec![
                GeneratedModuleTokens::new(
                    location.module(model),
                    quote! {
                        pub mod edge;
                        pub mod step;
                        pub mod then;
                    },
                ),
                edge_module(edge, location, model),
                branch_step_module(edges, location, model),
            ];

            modules.extend(then_modules(&continued, location, model));
            modules
        }
        QueryNode::Leaf(_) => vec![
            GeneratedModuleTokens::new(location.module(model), quote! { pub mod edge; }),
            edge_module(edge, location, model),
        ],
    }
}

fn continued_tokens(edge: &QueryEdge, location: &NodeLocation, model: &Model) -> TokenStream {
    let record = path_tokens(&model.path);

    match &edge.node {
        QueryNode::Branch(_) => {
            let path = location.step().path(model);

            quote! { #path::Step }
        }
        QueryNode::Leaf(NodeEnding::UniqueEnd) => quote! {
            margaret::framework::active_record::unique::Unique<
                #record,
                margaret::framework::active_record::unguarded::Unguarded,
            >
        },
        QueryNode::Leaf(NodeEnding::PlainEnd) => {
            quote! { margaret::framework::active_record::prefix::Prefix<#record> }
        }
    }
}

fn edge_module(edge: &QueryEdge, location: &NodeLocation, model: &Model) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let continued = continued_tokens(edge, location, model);
    let scan_order = match edge.node.ordering() {
        NodeOrdering::Total(rest) => {
            let spans = [edge.field].into_iter().chain(rest).map(field_span_tokens);

            quote! {
                impl margaret::framework::active_record::scan_order::ScanOrder<#record> for Edge {
                    const SPANS: &'static [margaret::framework::active_record::field_span::FieldSpan] =
                        &[#(#spans),*];
                }
            }
        }
        NodeOrdering::Partial => quote! {},
    };

    GeneratedModuleTokens::new(
        location.edge().module(model),
        quote! {
            pub struct Edge;

            impl margaret::framework::active_record::query_edge::QueryEdge<#record> for Edge {
                type Continued = #continued;
            }

            #scan_order
        },
    )
}

fn step_declaration(
    edges: &[&QueryEdge],
    location: &NodeLocation,
    model: &Model,
    state: &TokenStream,
) -> TokenStream {
    let record = path_tokens(&model.path);
    let fields = edges.iter().map(|edge| {
        let identifier = field_identifier(edge.field);
        let field_type = field_type_tokens(edge.field);
        let edge_path = location.child(edge.field).edge().path(model);

        quote! {
            pub #identifier: margaret::framework::active_record::next::Next<#record, #field_type, #edge_path::Edge, #state>
        }
    });

    quote! {
        pub struct Step {
            #(#fields),*
        }
    }
}

fn then_modules(
    edges: &[&QueryEdge],
    location: &NodeLocation,
    model: &Model,
) -> Vec<GeneratedModuleTokens> {
    let declarations = edges.iter().map(|edge| {
        let identifier = field_identifier(edge.field);

        quote! { pub mod #identifier; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        location.then().module(model),
        quote! { #(#declarations)* },
    )];

    for edge in edges {
        modules.extend(continuation_modules(
            edge,
            &location.child(edge.field),
            model,
        ));
    }

    modules
}

pub(crate) fn render_query(model: &Model) -> Vec<GeneratedModuleTokens> {
    let root_edges = TrieBuilder::of(model).root_edges();
    let edges: Vec<&QueryEdge> = root_edges.iter().collect();
    let location = NodeLocation::query();
    let constructions = edges.iter().map(|edge| {
        let identifier = field_identifier(edge.field);
        let start = edge.field.start;

        quote! { #identifier: margaret::framework::active_record::next::Next::new(margaret::framework::active_record::root::Root, #start) }
    });
    let step = step_declaration(
        &edges,
        &location,
        model,
        &quote! { margaret::framework::active_record::root::Root },
    );
    let mut modules = vec![
        GeneratedModuleTokens::new(
            location.module(model),
            quote! {
                pub mod step;
                pub mod then;
            },
        ),
        GeneratedModuleTokens::new(
            location.step().module(model),
            quote! {
                #step

                impl margaret::framework::active_record::field_set::FieldSet for Step {
                    const FIELDS: Self = Self { #(#constructions),* };
                }
            },
        ),
    ];

    modules.extend(then_modules(&edges, &location, model));
    modules
}
