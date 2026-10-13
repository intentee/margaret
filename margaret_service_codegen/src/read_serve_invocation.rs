use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::routes_ident::routes_ident;
use margaret_codegen_tokens::server_origin_ident::server_origin_ident;
use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;
use margaret_codegen_tokens::spiffe_issuer_request_client_ident::spiffe_issuer_request_client_ident;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_origin_source::ServerOriginSource;
use margaret_http_codegen::server_uploads::ServerUploads;
use margaret_serve_input_codegen::chained_read::ChainedRead;
use margaret_serve_input_codegen::chained_reads::chained_reads;
use margaret_serve_input_codegen::read_expression::ReadExpression;
use margaret_serve_input_codegen::spiffe_client_requirements::SpiffeClientRequirements;

use crate::serve_invocation_field::ServeInvocationField;
use crate::serve_invocation_fields::serve_invocation_fields;
use crate::serve_invocation_read::ServeInvocationRead;
use crate::server_bindings::ServerBindings;
use crate::service_plan::ServicePlan;
use crate::spiffe_activation::SpiffeActivation;
use crate::svid_bundle_type::svid_bundle_type;

fn required_argument(value_type: &TokenStream, id: &str) -> TokenStream {
    quote! {
        margaret::framework::console::required_argument::required_argument::<#value_type>(matches, #id)
    }
}

fn reported(fallible: &TokenStream) -> TokenStream {
    quote! { #fallible.map_err(margaret::framework::console::report_failure::report_failure) }
}

fn spiffe_reads(
    activation: SpiffeActivation,
    SpiffeClientRequirements {
        http_client,
        issuer_request_client,
    }: SpiffeClientRequirements,
) -> Vec<ChainedRead> {
    if !activation.any() {
        return Vec::new();
    }

    let bundle_type = svid_bundle_type(activation);
    let string_type = quote! { ::std::string::String };
    let bundle = reported(&quote! {
        #bundle_type::new(
            margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams {
                spiffe_trust_domain,
                spire_agent_addr,
            },
        )
    });
    let mut reads = vec![
        ChainedRead {
            binding: quote! { spiffe_trust_domain },
            read: ReadExpression::Fallible(required_argument(&string_type, "spiffe-trust-domain")),
        },
        ChainedRead {
            binding: quote! { spire_agent_addr },
            read: ReadExpression::Fallible(required_argument(&string_type, "spire-agent-addr")),
        },
        ChainedRead {
            binding: quote! { spiffe_bundle },
            read: ReadExpression::Fallible(bundle),
        },
    ];

    if activation.server_active {
        reads.push(ChainedRead {
            binding: quote! { spiffe_server_config },
            read: ReadExpression::Infallible(quote! {
                ::std::sync::Arc::new(spiffe_bundle.server_config())
            }),
        });
    }

    if http_client {
        let binding = spiffe_http_client_ident();

        reads.push(ChainedRead {
            binding: quote! { #binding },
            read: ReadExpression::Fallible(reported(&quote! { spiffe_bundle.reqwest_client() })),
        });
    }

    if issuer_request_client {
        let binding = spiffe_issuer_request_client_ident();

        reads.push(ChainedRead {
            binding: quote! { #binding },
            read: ReadExpression::Fallible(reported(&quote! {
                margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient::preconfigured(
                    spiffe_bundle.reqwest_client_builder(),
                )
                .map(::std::sync::Arc::new)
            })),
        });
    }

    reads
}

fn origin_read(server: &HttpServer) -> ChainedRead {
    let binding = server_origin_ident(server.name());

    ChainedRead {
        binding: quote! { #binding },
        read: match server.origin() {
            ServerOriginSource::Argument => {
                let read = required_argument(
                    &quote! { margaret::framework::server_origin::server_origin::ServerOrigin },
                    &server.url_argument(),
                );

                ReadExpression::Fallible(quote! {
                    #read.map(|server_origin| {
                        ::std::sync::Arc::<str>::from(server_origin.origin.ascii_serialization())
                    })
                })
            }
            ServerOriginSource::Issuer { endpoints } => {
                let endpoints = path_tokens(endpoints);

                ReadExpression::Infallible(quote! {
                    ::std::sync::Arc::<str>::from(#endpoints.issuer_origin)
                })
            }
        },
    }
}

fn listener_reads(server: &HttpServer, activation: SpiffeActivation) -> Vec<ChainedRead> {
    let ServerBindings {
        address,
        listener,
        transport,
        upload_directory,
    } = ServerBindings::of(server);
    let mut reads = vec![ChainedRead {
        binding: quote! { #address },
        read: ReadExpression::Fallible(required_argument(
            &quote! { ::std::string::String },
            &server.address_argument(),
        )),
    }];
    let uploads = match server.uploads() {
        ServerUploads::Accepted => {
            reads.push(ChainedRead {
                binding: quote! { #upload_directory },
                read: ReadExpression::Fallible(required_argument(
                    &quote! { ::std::path::PathBuf },
                    &server.upload_dir_argument(),
                )),
            });

            quote! {
                margaret::framework::http_uploaded_file::upload_config::UploadConfig::enabled(
                    #upload_directory,
                )
            }
        }
        ServerUploads::Refused => {
            quote! { margaret::framework::http_uploaded_file::upload_config::UploadConfig::Disabled }
        }
    };
    let transport_config = if activation.server_active {
        reads.push(ChainedRead {
            binding: quote! { #transport },
            read: ReadExpression::Fallible(required_argument(
                &quote! { margaret::framework::service::transport_choice::TransportChoice },
                &server.transport_argument(),
            )),
        });

        quote! { #transport.config(&spiffe_server_config) }
    } else {
        quote! { margaret::framework::http::transport_config::TransportConfig::Plain }
    };

    reads.push(ChainedRead {
        binding: quote! { #listener },
        read: ReadExpression::Infallible(quote! {
            margaret::framework::service::server_listener::ServerListener {
                address: #address,
                transport: #transport_config,
                uploads: #uploads,
            }
        }),
    });

    reads
}

fn server_reads(servers: &[HttpServer], activation: SpiffeActivation) -> Vec<ChainedRead> {
    if servers.is_empty() {
        return Vec::new();
    }

    let origins = servers.iter().map(|server| {
        let origin = server_origin_ident(server.name());

        quote! { &#origin }
    });
    let routes = routes_ident();
    let routes_read = ChainedRead {
        binding: quote! { #routes },
        read: ReadExpression::Fallible(reported(&quote! {
            super::routes::Routes::from_origins(#(#origins),*).map(::std::sync::Arc::new)
        })),
    };

    servers
        .iter()
        .map(origin_read)
        .chain([routes_read])
        .chain(
            servers
                .iter()
                .flat_map(|server| listener_reads(server, activation)),
        )
        .collect()
}

#[must_use]
pub fn read_serve_invocation(
    plan: &ServicePlan,
    servers: &[HttpServer],
    bindings: &ContainerBindings,
) -> ServeInvocationRead {
    let activation = SpiffeActivation::of(plan, servers);
    let reads: Vec<ChainedRead> = spiffe_reads(activation, plan.spiffe_clients)
        .into_iter()
        .chain(server_reads(servers, activation))
        .chain(plan.reads.iter().cloned())
        .collect();
    let initializations = serve_invocation_fields(plan, servers, bindings)
        .iter()
        .map(ServeInvocationField::initialization)
        .collect::<Vec<TokenStream>>();
    let finish = quote! {
        super::serve::serve_invocation::ServeInvocation {
            #(#initializations)*
        }
    };

    ServeInvocationRead {
        binding: if activation.any() || !servers.is_empty() || plan.reads_clap_matches {
            quote! { matches }
        } else {
            quote! { _matches }
        },
        chain: chained_reads(&reads, &finish),
    }
}
