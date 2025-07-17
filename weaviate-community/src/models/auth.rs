use reqwest::header::{HeaderName, HeaderValue};
use secrecy::{ExposeSecret, SecretString};

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct AuthSecret {
    api_key: SecretString,
}

impl AuthSecret {
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = SecretString::from(api_key.into());
        AuthSecret { api_key }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> Result<HeaderValue, reqwest::header::InvalidHeaderValue> {
        let bearer = format!("Bearer {}", self.api_key.expose_secret());
        HeaderValue::from_str(&bearer)
    }
}

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct ApiKey {
    pub api_header: String,
    pub api_key: SecretString,
}

impl ApiKey {
    /// Construct a new `AuthApiKey`.
    pub fn new(api_header: impl Into<String>, api_key: impl Into<String>) -> Self {
        let api_key = SecretString::from(api_key.into());
        ApiKey {
            api_header: api_header.into(),
            api_key,
        }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_name(&self) -> Result<HeaderName, reqwest::header::InvalidHeaderName> {
        HeaderName::from_bytes(self.api_header.as_bytes())
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> Result<HeaderValue, reqwest::header::InvalidHeaderValue> {
        HeaderValue::from_str(self.api_key.expose_secret())
    }
}
