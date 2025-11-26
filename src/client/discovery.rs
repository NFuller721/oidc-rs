use url::Url;

#[derive(Deserialize)]
struct ProviderMetadata {
    issuer: Url,
    authorization_endpoint: Url,
    token_endpoint: Url,
    userinfo_endpoint: Option<Url>,
    jwks_uri: Url,
    registration_endpoint: Option<Url>,

    scopes_supported: Option<Vec<String>>,
    response_types_supported: Vec<String>,
    response_modes_supported: Option<Vec<String>>,
    grant_types_supported: Option<Vec<String>>,
    acr_values_supported: Option<Vec<String>>,

    subject_types_supported: Vec<SubjectType>,
    id_token_signing_alg_values_supported: Vec<SigningAlgorithm>,
    id_token_encryption_alg_values_supported: Option<Vec<String>>,
    id_token_encryption_enc_values_supported: Option<Vec<String>>,

    userinfo_signing_alg_values_supported: Option<Vec<String>>,
    userinfo_encryption_alg_values_supported: Option<Vec<String>>,
    userinfo_encryption_enc_values_supported: Option<Vec<String>>,

    request_object_signing_alg_values_supported: Option<Vec<String>>,
    request_object_encryption_alg_values_supported: Option<Vec<String>>,
    request_object_encryption_enc_values_supported: Option<Vec<String>>,

    token_endpoint_auth_methods_supported: Option<Vec<String>>,
    token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,

    display_values_supported: Option<Vec<String>>,
    claim_types_supported: Option<Vec<String>>,
    claims_supported: Option<Vec<String>>,

    service_documentation: Option<Url>,
    claims_locales_supported: Option<Vec<String>>,
    ui_locales_supported: Option<Vec<String>>,

    claims_parameter_supported: Option<bool>,
    request_parameter_supported: Option<bool>,
    request_uri_parameter_supported: Option<bool>,
    require_request_uri_registration: Option<bool>,

    op_policy_uri: Option<Url>,
    op_tos_uri: Option<Url>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SubjectType {
    PairWise,
    Public,

    #[serde(untagged)]
    Other(String),
}

#[derive(Deserialize)]
enum SigningAlgorithm {
    RS256,

    #[serde(untagged)]
    Other(String),
}
