use reqwest::{StatusCode, Url};

use crate::error::WeaviateError;
use crate::models::nodes::MultiNodes;
use crate::{ResponseExt, WeaviateClient};

/// All nodes related endpoints and functionality described in
/// [Weaviate nodes API documentation](https://weaviate.io/developers/weaviate/api/rest/nodes)
#[derive(Debug)]
pub struct Nodes<'a> {
    client: &'a WeaviateClient,
}

impl<'a> Nodes<'a> {
    /// Create a new instance of the Nodes endpoint struct. Should only be done by the parent client.
    pub(crate) fn new(client: &'a WeaviateClient) -> Self {
        Nodes { client }
    }

    /// Get the endpoint for nodes
    ///
    /// # Returns
    /// A `Result` containing the URL for the nodes endpoint or a `ParseError` if the URL is invalid.
    ///
    /// An `Err` variant should not occur as the `base_url` is validated during the `WeaviateClient` creation.
    fn endpoint(&self) -> Result<Url, url::ParseError> {
        self.client.base_url.join("/v1/nodes/")
    }

    /// Get the node status for all nodes in the Weaviate instance.
    ///
    /// # Examples
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let res = client.nodes().get_nodes_status().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_nodes_status(&self) -> Result<MultiNodes, WeaviateError> {
        let endpoint = self.endpoint()?;
        let res = self
            .client
            .get(endpoint)
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
    use crate::{models::nodes::MultiNodes, WeaviateClient};

    async fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new_async().await;
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn test_nodes() -> MultiNodes {
        let nodes: MultiNodes = serde_json::from_value(serde_json::json!(
        {
            "nodes": [
              {
                "batchStats": {
                  "ratePerSecond": 0
                },
                "gitHash": "e6b37ce",
                "name": "weaviate-0",
                "shards": [
                  {
                    "class": "TestArticle",
                    "name": "nq1Bg9Q5lxxP",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  },
                  {
                    "class": "TestAuthor",
                    "name": "MINLtCghkdG8",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  }
                ],
                "stats": {
                  "objectCount": 0,
                  "shardCount": 2
                },
                "status": "HEALTHY",
                "version": "1.22.1"
              },
              {
                "batchStats": {
                  "ratePerSecond": 0
                },
                "gitHash": "e6b37ce",
                "name": "weaviate-1",
                "shards": [
                  {
                    "class": "TestArticle",
                    "name": "HuPocHE5w2LP",
                    "objectCount": 1,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  },
                  {
                    "class": "TestAuthor",
                    "name": "PeQjZRmK0xNB",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  }
                ],
                "stats": {
                  "objectCount": 1,
                  "shardCount": 2
                },
                "status": "HEALTHY",
                "version": "1.22.1"
              },
              {
                "batchStats": {
                  "ratePerSecond": 0
                },
                "gitHash": "e6b37ce",
                "name": "weaviate-2",
                "shards": [
                  {
                    "class": "TestArticle",
                    "name": "JTg39c7ZlFUX",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  },
                  {
                    "class": "TestAuthor",
                    "name": "W5ulmuJGDTxj",
                    "objectCount": 1,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  }
                ],
                "stats": {
                  "objectCount": 1,
                  "shardCount": 2
                },
                "status": "HEALTHY",
                "version": "1.22.1"
              }
            ]
          }))
        .unwrap();
        nodes
    }

    async fn mock_get(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str,
    ) -> mockito::Mock {
        server
            .mock("GET", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    #[tokio::test]
    async fn test_get_nodes_status_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let nodes = test_nodes();
        let nodes_str = serde_json::to_string(&nodes).unwrap();
        let mock = mock_get(&mut mock_server, "/v1/nodes/", 200, &nodes_str).await;
        let res = client.nodes().get_nodes_status().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(res.unwrap().nodes.len(), nodes.nodes.len());
    }

    #[tokio::test]
    async fn test_get_nodes_status_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/nodes/", 404, "").await;
        let res = client.nodes().get_nodes_status().await;
        mock.assert();
        assert!(res.is_err());
    }
}
