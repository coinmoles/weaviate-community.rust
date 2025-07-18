use serde::Deserialize;

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
