use serde_json::json;

/// RawQuery struct to hold a custom `raw` query.
#[derive(Debug)]
pub struct RawQuery {
    pub query: String,
}

impl RawQuery {
    /// Retrieve a raw GraphQL query.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::RawQuery;
    ///
    /// let my_query_str = "{
    ///   Get {
    ///     JeopardyQuestion {
    ///       question
    ///       answer
    ///       points
    ///     }
    ///   }
    /// }";
    ///
    /// let query = RawQuery::new(my_query_str);
    /// ```
    pub fn new(query: &str) -> Self {
        RawQuery {
            query: query.into(),
        }
    }

    /// Convert the `RawQuery` into a `serde_json::Value` payload.
    pub fn as_payload(&self) -> serde_json::Value {
        json!({ "query": self.query })
    }
}
