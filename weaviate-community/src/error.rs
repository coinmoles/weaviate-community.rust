/// Error for Weaviate operations.
#[derive(Debug)]
pub enum WeaviateError {
    UrlParseError(url::ParseError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    QueryError(QueryError),
    UnexpectedStatusCode {
        url: reqwest::Url,
        expected: reqwest::StatusCode,
        actual: reqwest::StatusCode,
        reason: Option<String>,
    },
    BackupFailed,
}

impl std::fmt::Display for WeaviateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaviateError::UrlParseError(e) => write!(f, "URL parse error: {e}"),
            WeaviateError::ReqwestError(e) => write!(f, "Reqwest error: {e}"),
            WeaviateError::SerdeJsonError(e) => write!(f, "Serde JSON error: {e}"),
            WeaviateError::InvalidHeaderName(e) => write!(f, "Invalid header name: {e}"),
            WeaviateError::InvalidHeaderValue(e) => write!(f, "Invalid header value: {e}"),
            WeaviateError::QueryError(e) => write!(f, "Query error: {e}"),
            WeaviateError::UnexpectedStatusCode {
                url,
                expected,
                actual,
                reason,
            } => {
                write!(
                    f,
                    "Unexpected status code from URL {url}: expected {expected}, got {actual}."
                )?;
                if let Some(reason) = reason {
                    write!(f, " Reason: {reason:?}")?;
                }
                Ok(())
            }
            WeaviateError::BackupFailed => write!(f, "Backup operation failed"),
        }
    }
}

impl std::error::Error for WeaviateError {}

impl From<url::ParseError> for WeaviateError {
    fn from(err: url::ParseError) -> Self {
        WeaviateError::UrlParseError(err)
    }
}

impl From<reqwest::Error> for WeaviateError {
    fn from(err: reqwest::Error) -> Self {
        WeaviateError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for WeaviateError {
    fn from(err: serde_json::Error) -> Self {
        WeaviateError::SerdeJsonError(err)
    }
}

impl From<reqwest::header::InvalidHeaderName> for WeaviateError {
    fn from(err: reqwest::header::InvalidHeaderName) -> Self {
        WeaviateError::InvalidHeaderName(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for WeaviateError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        WeaviateError::InvalidHeaderValue(err)
    }
}

impl From<QueryError> for WeaviateError {
    fn from(err: QueryError) -> Self {
        WeaviateError::QueryError(err)
    }
}

#[derive(Debug)]
pub enum QueryError {
    InvalidCombination(&'static [&'static str]),
    InconsistentLength(usize, usize),
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::InvalidCombination(combination) => {
                write!(f, "These query parameters cannot all be `Some`:[",)?;
                for (i, param) in combination.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{param}",)?;
                }
                write!(f, " ]")
            }
            QueryError::InconsistentLength(expected, actual) => {
                write!(
                    f,
                    "`to_class_names` and `to_uuid` must have the same length. `to_class` has length {expected}, but `to_uuid` has length {actual}."
                )
            }
        }
    }
}

impl std::error::Error for QueryError {}
