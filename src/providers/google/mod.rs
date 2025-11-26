use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct GoogleClient {
    client_id: SecretString,
    client_secret: SecretString,
}

impl GoogleClient {
    pub fn builder(
        client_id: impl Into<SecretString>,
        client_secret: impl Into<SecretString>,
    ) -> GoogleClientBuilder {
        GoogleClientBuilder {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GoogleClientBuilder {
    client_id: SecretString,
    client_secret: SecretString,
}

impl GoogleClientBuilder {
    pub fn build(self) -> GoogleClient {
        GoogleClient {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}
