use reqwest::header::{HeaderName, HeaderValue};

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub enum AuthSecret {
    ApiKey(String),
    BearerToken(String),
}

impl AuthSecret {
    pub fn api_key(api_key: impl Into<String>) -> Self {
        AuthSecret::ApiKey(api_key.into())
    }

    pub fn bearer_token(token: impl Into<String>) -> Self {
        AuthSecret::BearerToken(token.into())
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> Result<HeaderValue, reqwest::header::InvalidHeaderValue> {
        let bearer = match self {
            AuthSecret::ApiKey(api_key) => format!("ApiKey {api_key}"),
            AuthSecret::BearerToken(token) => format!("Bearer {token}"),
        };
        HeaderValue::from_str(&bearer)
    }
}

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct ApiKey {
    pub api_header: String,
    pub api_key: String,
}

impl ApiKey {
    /// Construct a new `AuthApiKey`.
    pub fn new(api_header: impl Into<String>, api_key: impl Into<String>) -> Self {
        ApiKey {
            api_header: api_header.into(),
            api_key: api_key.into(),
        }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_name(&self) -> Result<HeaderName, reqwest::header::InvalidHeaderName> {
        HeaderName::from_bytes(self.api_header.as_bytes())
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> Result<HeaderValue, reqwest::header::InvalidHeaderValue> {
        HeaderValue::from_str(&self.api_key)
    }
}
