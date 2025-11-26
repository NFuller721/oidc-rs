use oidc::providers::google::GoogleClient;
use secrecy::SecretString;

fn main() {
    let builder = GoogleClient::builder(
        SecretString::from("client-id"),
        SecretString::from("client-secret"),
    )
    .build();

    dbg!(builder);
}
