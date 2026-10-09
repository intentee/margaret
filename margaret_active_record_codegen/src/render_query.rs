use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::branch_edge::BranchEdge;
use crate::field_identifier::field_identifier;
use crate::field_span_tokens::field_span_tokens;
use crate::field_type_tokens::field_type_tokens;
use crate::node_ending::NodeEnding;
use crate::node_location::NodeLocation;
use crate::node_ordering::NodeOrdering;
use crate::node_update_tokens::node_update_tokens;
use crate::query_edge::QueryEdge;
use crate::query_node::QueryNode;
use crate::trie_builder::TrieBuilder;

fn branches<'tree, 'model>(edges: &[&'tree QueryEdge<'model>]) -> Vec<BranchEdge<'tree, 'model>> {
    edges
        .iter()
        .filter_map(|edge| match &edge.node {
            QueryNode::Branch(edges) => Some(BranchEdge {
                edges,
                field: edge.field,
                node: &edge.node,
            }),
            QueryNode::Leaf(_) => None,
        })
        .collect()
}

fn continued_tokens(edge: &QueryEdge, location: &NodeLocation, model: &Model) -> TokenStream {
    let record = path_tokens(&model.path);

    match &edge.node {
        QueryNode::Branch(_) => {
            let path = location.child(&edge.field.name).path(model);

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

fn step_declaration(
    edges: &[&QueryEdge],
    location: &NodeLocation,
    model: &Model,
    state: &TokenStream,
) -> TokenStream {
    let record = path_tokens(&model.path);
    let then = if branches(edges).is_empty() {
        quote! {}
    } else {
        quote! { pub mod then; }
    };
    let fields = edges.iter().map(|edge| {
        let identifier = field_identifier(edge.field);
        let field_type = field_type_tokens(edge.field);
        let continued = continued_tokens(edge, location, model);

        quote! {
            pub #identifier: margaret::framework::active_record::next::Next<#record, #field_type, #continued, #state>
        }
    });

    quote! {
        #then

        pub struct Step {
            #(#fields),*
        }
    }
}

fn branch_modules(
    edges: &[&QueryEdge],
    location: &NodeLocation,
    model: &Model,
) -> Vec<GeneratedModuleTokens> {
    let branches = branches(edges);

    if branches.is_empty() {
        return Vec::new();
    }

    let declarations = branches.iter().map(|branch| {
        let identifier = field_identifier(branch.field);

        quote! { pub mod #identifier; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        location.then().module(model),
        quote! { #(#declarations)* },
    )];

    for branch in &branches {
        modules.extend(render_branch(
            branch,
            &location.child(&branch.field.name),
            model,
        ));
    }

    modules
}

fn render_branch(
    branch: &BranchEdge,
    location: &NodeLocation,
    model: &Model,
) -> Vec<GeneratedModuleTokens> {
    let record = path_tokens(&model.path);
    let edges: Vec<&QueryEdge> = branch.edges.all().collect();
    let step = step_declaration(
        &edges,
        location,
        model,
        &quote! { margaret::framework::active_record::narrowed::Narrowed<#record> },
    );
    let first = field_identifier(branch.edges.first.field);
    let first_start = branch.edges.first.field.start;
    let rest = branch.edges.rest.iter().map(|edge| {
        let identifier = field_identifier(edge.field);
        let start = edge.field.start;

        quote! {
            #identifier: margaret::framework::active_record::next::Next::new(
                ::std::clone::Clone::clone(&narrowed),
                #start,
            )
        }
    });
    let scan_order = match branch.node.ordering() {
        NodeOrdering::Total(rest_fields) => {
            let spans = rest_fields.into_iter().map(field_span_tokens);

            quote! {
                impl margaret::framework::active_record::scan_order::ScanOrder<#record> for Step {
                    const REST: &'static [margaret::framework::active_record::field_span::FieldSpan] =
                        &[#(#spans),*];
                }
            }
        }
        NodeOrdering::Partial => quote! {},
    };
    let update_impl = node_update_tokens(model, &first);
    let mut modules = vec![GeneratedModuleTokens::new(
        location.module(model),
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

            #scan_order
        },
    )];

    modules.extend(branch_modules(&edges, location, model));
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
    let mut modules = vec![GeneratedModuleTokens::new(
        location.module(model),
        quote! {
            #step

            impl margaret::framework::active_record::field_set::FieldSet for Step {
                const FIELDS: Self = Self { #(#constructions),* };
            }
        },
    )];

    modules.extend(branch_modules(&edges, &location, model));
    modules
}
