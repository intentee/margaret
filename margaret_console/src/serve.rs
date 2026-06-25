use clap::ArgMatches;

use margaret_http::server::Server;

use crate::command_outcome::CommandOutcome;

pub async fn serve(server: Server, matches: &ArgMatches) -> CommandOutcome {
    let address = matches
        .get_one::<String>("addr")
        .expect("addr is a required argument");

    match server.bind(address).await {
        Ok(bound) => bound.serve().await,
        Err(error) => {
            eprintln!("failed to bind to {address}: {error}");

            CommandOutcome::Failed
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Arg;
    use clap::Command;
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;

    use margaret_http::router::Router;
    use margaret_http::server::Server;

    use super::serve;
    use crate::command_outcome::CommandOutcome;

    fn matches_for(address: &str) -> clap::ArgMatches {
        Command::new("app")
            .arg(Arg::new("addr").long("addr").required(true))
            .get_matches_from(["app", "--addr", address])
    }

    #[tokio::test]
    async fn fails_to_bind_an_invalid_address() {
        let outcome = serve(
            Server::new(Router::empty()),
            &matches_for("this is not an address"),
        )
        .await;

        assert_eq!(outcome, CommandOutcome::Failed);
    }

    #[tokio::test]
    async fn serves_a_bound_address() {
        let probe = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("a probe port is reserved");
        let address = probe
            .local_addr()
            .expect("the probe address is known")
            .to_string();
        drop(probe);

        let matches = Box::leak(Box::new(matches_for(&address)));

        tokio::spawn(serve(Server::new(Router::empty()), matches));

        let mut connected = false;

        while !connected {
            tokio::task::yield_now().await;
            connected = TcpStream::connect(&address).await.is_ok();
        }
    }
}
