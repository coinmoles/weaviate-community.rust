use reqwest::header::{HeaderName, HeaderValue};

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct AuthApiKey {
    pub api_key: String,
}

impl AuthApiKey {
    /// Construct a new `AuthApiKey`.
    pub fn new(api_key: &str) -> Self {
        AuthApiKey {
            api_key: api_key.into(),
        }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> HeaderValue {
        let bearer = format!("ApiKey {}", self.api_key);
        HeaderValue::from_str(&bearer).unwrap()
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
    pub fn new(api_header: &str, api_key: &str) -> Self {
        ApiKey {
            api_header: api_header.into(),
            api_key: api_key.into(),
        }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_name(&self) -> HeaderName {
        HeaderName::from_bytes(self.api_header.as_bytes()).unwrap()
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> HeaderValue {
        HeaderValue::from_str(&self.api_key).unwrap()
    }
}
