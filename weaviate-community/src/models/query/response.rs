use serde::Deserialize;

use crate::error::WeaviateError;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MaybeError<T> {
    Error(Errors),
    Ok(T),
}

impl<T> MaybeError<T> {
    pub fn error_for_error(self) -> Result<T, WeaviateError> {
        match self {
            MaybeError::Ok(data) => Ok(data),
            MaybeError::Error(err) => Err(WeaviateError::Other(err.errors.to_string())),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Errors {
    pub errors: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLGetResponse<T> {
    pub data: GetData<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetData<T> {
    #[serde(rename = "Get")]
    pub get: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLAggregateResponse<T> {
    pub data: AggregateData<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AggregateData<T> {
    #[serde(rename = "Aggregate")]
    pub aggregate: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLExploreResponse<T> {
    pub data: ExploreData<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExploreData<T> {
    #[serde(rename = "Explore")]
    pub explore: T,
}
