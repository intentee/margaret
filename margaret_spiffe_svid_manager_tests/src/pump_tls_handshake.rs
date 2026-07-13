use rustls::ClientConnection;
use rustls::ServerConnection;

use crate::handshake_error::HandshakeError;

const ROUNDS_BEFORE_THE_HANDSHAKE_IS_CONSIDERED_STUCK: usize = 100;

pub fn pump_tls_handshake(
    server_connection: &mut ServerConnection,
    client_connection: &mut ClientConnection,
) -> Result<(), HandshakeError> {
    for _ in 0..ROUNDS_BEFORE_THE_HANDSHAKE_IS_CONSIDERED_STUCK {
        let mut server_to_client = Vec::new();
        let mut client_to_server = Vec::new();

        client_connection
            .write_tls(&mut client_to_server)
            .expect("writing TLS to a Vec cannot fail");
        server_connection
            .read_tls(&mut client_to_server.as_slice())
            .expect("reading TLS from a slice cannot fail");
        server_connection.process_new_packets()?;

        server_connection
            .write_tls(&mut server_to_client)
            .expect("writing TLS to a Vec cannot fail");
        client_connection
            .read_tls(&mut server_to_client.as_slice())
            .expect("reading TLS from a slice cannot fail");
        client_connection.process_new_packets()?;

        if !client_connection.is_handshaking() && !server_connection.is_handshaking() {
            return Ok(());
        }
    }

    Err(HandshakeError::ExceededMaxRounds)
}
