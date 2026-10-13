use margaret_oauth_vocabulary::client_secret::ClientSecret;

#[test]
fn keeps_the_client_secret_out_of_its_debug_output() {
    let secret = "7Fjfp0ZBr1KtDRbnfVdmIw"
        .parse::<ClientSecret>()
        .expect("the client secret is visible");

    assert_eq!(format!("{secret:?}"), "ClientSecret { .. }");
    assert_eq!(secret.expose(), "7Fjfp0ZBr1KtDRbnfVdmIw");
}
