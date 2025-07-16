use reqwest::{StatusCode, Url};

use crate::{
    error::WeaviateError,
    models::query::{AggregateQuery, ExploreQuery, GetQuery, RawQuery},
    ResponseExt, WeaviateClient,
};

/// All GraphQL related endpoints and functionality described in
/// [Weaviate GraphQL API documentation](https://weaviate.io/developers/weaviate/api/graphql)
#[derive(Debug)]
pub struct Query<'a> {
    client: &'a WeaviateClient,
}

impl<'a> Query<'a> {
    /// Create a new Query object. The query object is intended to like inside the WeaviateClient
    /// and be called through the WeaviateClient.
    pub(crate) fn new(client: &'a WeaviateClient) -> Self {
        Query { client }
    }

    /// Get the endpoint for graphql
    ///
    /// # Returns
    /// A `Result` containing the URL for the graphql endpoint or a `ParseError` if the URL is invalid.
    ///
    /// An `Err` variant should not occur as the `base_url` is validated during the `WeaviateClient` creation.
    fn endpoint(&self) -> Result<Url, url::ParseError> {
        self.client.base_url.join("/v1/graphql/")
    }

    /// Execute the Get{} GraphQL query
    ///
    /// # Parameters
    /// - query: the query to execute
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::models::query::GetBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let query = GetBuilder::new(
    ///         "JeopardyQuestion",
    ///         vec![
    ///             "question",
    ///             "answer",
    ///             "points",
    ///             "hasCategory { ... on JeopardyCategory { title }}"
    ///         ])
    ///         .with_limit(1)
    ///         .with_additional(vec!["id"])
    ///         .build();
    ///     let res = client.query().get(query).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, query: GetQuery) -> Result<serde_json::Value, WeaviateError> {
        let endpoint = self.endpoint()?;
        let payload = serde_json::to_value(query)?;
        let res: serde_json::Value = self
            .client
            .post(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    /// Execute the Aggregate{} GraphQL query
    ///
    ///
    /// # Parameters
    /// - query: the query to execute
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::models::query::AggregateBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let query = AggregateBuilder::new("Article")
    ///         .with_meta_count()
    ///         .with_fields(vec!["wordCount {count maximum mean median minimum mode sum type}"])
    ///         .build();
    ///     let res = client.query().aggregate(query).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn aggregate(
        &self,
        query: AggregateQuery,
    ) -> Result<serde_json::Value, WeaviateError> {
        let endpoint = self.endpoint()?;
        let payload = serde_json::to_value(query).unwrap();
        let res: serde_json::Value = self
            .client
            .post(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    /// Execute the Explore{} GraphQL query
    ///
    /// # Parameters
    /// - query: the query to execute
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::models::query::ExploreBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let query = ExploreBuilder::new()
    ///         .with_limit(1)
    ///         .with_near_vector("{vector: [-0.36840257,0.13973749,-0.28994447]}")
    ///         .with_fields(vec!["className"])
    ///         .build();
    ///     let res = client.query().explore(query).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn explore(&self, query: ExploreQuery) -> Result<serde_json::Value, WeaviateError> {
        let endpoint = self.endpoint()?;
        let payload = serde_json::to_value(query).unwrap();
        let res = self
            .client
            .post(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    /// Execute a raw GraphQL query.
    ///
    /// This method has been implemented to allow you to run your own query that doesn't fit in
    /// with the format that is set out in this crate.
    ///
    /// If there is a query that you think should be added, please open up a new feature request on
    /// GitHub.
    ///
    /// # Parameters
    /// - query: the query to execute
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::models::query::RawQuery;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let query = RawQuery::new("{Get{JeopardyQuestion{question answer points}}}");
    ///     let res = client.query().raw(query).await;
    ///     Ok(())
    ///
    /// }
    /// ```
    pub async fn raw(&self, query: RawQuery) -> Result<serde_json::Value, WeaviateError> {
        let endpoint = self.endpoint()?;
        let payload = serde_json::to_value(query).unwrap();
        let res: serde_json::Value = self
            .client
            .post(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::query::RawQuery;
    use crate::models::query::{AggregateBuilder, ExploreBuilder, GetBuilder};
    use crate::WeaviateClient;

    async fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new_async().await;
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    async fn mock_post(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str,
    ) -> mockito::Mock {
        server
            .mock("POST", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    async fn test_get_response() -> String {
        serde_json::to_string(&serde_json::json!({
            "data": {
                "Get": {
                    "JeopardyQuestion": [
                        {
                            "answer": "Jonah",
                            "points": 100,
                            "question": "This prophet passed the time he spent inside a fish offering up prayers"
                        },
                    ]
                }
            }
        })).unwrap()
    }

    fn test_aggregate_response() -> String {
        serde_json::to_string(&serde_json::json!(
        {
          "data": {
            "Aggregate": {
              "Article": [
                {
                  "inPublication": {
                    "pointingTo": [
                      "Publication"
                    ],
                    "type": "cref"
                  },
                  "meta": {
                    "count": 4403
                  },
                  "wordCount": {
                    "count": 4403,
                    "maximum": 16852,
                    "mean": 966.0113558937088,
                    "median": 680,
                    "minimum": 109,
                    "mode": 575,
                    "sum": 4253348,
                    "type": "int"
                  }
                }
              ]
            }
          }
        }))
        .unwrap()
    }

    async fn test_explore_response() -> String {
        serde_json::to_string(&serde_json::json!(
        {
          "data": {
            "Explore": [
              {
                "beacon": "weaviate://localhost/7e9b9ffe-e645-302d-9d94-517670623b35",
                "certainty": 0.975523,
                "className": "Publication"
              }
            ]
          },
          "errors": null
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn test_get_query_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let exp_res = test_get_response().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 200, &exp_res).await;
        let query = GetBuilder::new(
            "JeopardyQuestion",
            vec![
                "question",
                "answer",
                "points",
                "hasCategory { ... on JeopardyCategory { title }}",
            ],
        )
        .with_limit(1)
        .with_additional(vec!["id"])
        .build();
        let res = client.query().get(query).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap()["data"]["Get"]["JeopardyQuestion"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_get_query_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 422, "").await;
        let query = GetBuilder::new(
            "JeopardyQuestion",
            vec![
                "question",
                "answer",
                "points",
                "hasCategory { ... on JeopardyCategory { title }}",
            ],
        )
        .with_limit(1)
        .with_additional(vec!["id"])
        .build();
        let res = client.query().get(query).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_aggregate_query_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(
            &mut mock_server,
            "/v1/graphql/",
            200,
            &test_aggregate_response(),
        )
        .await;
        let query = AggregateBuilder::new("Article")
            .with_meta_count()
            .with_fields(vec![
                "wordCount {count maximum mean median minimum mode sum type}",
            ])
            .build();
        let res = client.query().aggregate(query).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap()["data"]["Aggregate"]["Article"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_aggregate_query_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 422, "").await;
        let query = AggregateBuilder::new("JeopardyQuestion").build();
        let res = client.query().aggregate(query).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_explore_query_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let exp_res = test_explore_response().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 200, &exp_res).await;
        let query = ExploreBuilder::new()
            .with_limit(1)
            .with_near_vector("{vector: [-0.36840257,0.13973749,-0.28994447]}")
            .with_fields(vec!["className"])
            .build();
        let res = client.query().explore(query).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_explore_query_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 422, "").await;
        let query = ExploreBuilder::new().build();
        let res = client.query().explore(query).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_raw_query_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let exp_res = test_get_response().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 200, &exp_res).await;
        let query = RawQuery::new("{ Get { JeopardyQuestion { question answer points } } }");
        let res = client.query().raw(query).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap()["data"]["Get"]["JeopardyQuestion"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_raw_query_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/graphql/", 422, "").await;
        let query = RawQuery::new("{ Get { JeopardyQuestion { question answer points } } }");
        let res = client.query().raw(query).await;
        mock.assert();
        assert!(res.is_err());
    }
}
