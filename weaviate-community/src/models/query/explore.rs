use serde_json::json;

use crate::error::QueryError;

/// ExploreQuery struct to hold an Explore query.
#[derive(Debug)]
pub struct ExploreQuery {
    limit: Option<u32>,
    near_text: Option<String>,
    near_vector: Option<String>,
    fields: Option<Vec<String>>,
}

impl ExploreQuery {
    /// Create a new ExploreQuery item.
    ///
    /// This is the same as `ExploreQuery::builder()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::ExploreQuery;
    ///
    /// let query_builder = ExploreQuery::new();
    /// ```
    pub fn new() -> Self {
        Self {
            limit: None,
            near_text: None,
            near_vector: None,
            fields: None,
        }
    }

    /// Appends the specified fields in the explore query body.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::ExploreQuery;
    ///
    /// let query_builder = ExploreQuery::new()
    ///     .with_fields(vec!["beacon", "certainty", "className"]);
    /// ```
    pub fn with_fields(mut self, fields: Vec<&str>) -> ExploreQuery {
        let fields = fields.iter().map(|field| field.to_string()).collect();
        self.fields = Some(fields);
        self
    }

    /// Sets the `limit` in the explore query filters.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::ExploreQuery;
    ///
    /// let query_builder = ExploreQuery::new()
    ///     .with_limit(1);
    /// ```
    pub fn with_limit(mut self, limit: u32) -> ExploreQuery {
        self.limit = Some(limit);
        self
    }

    /// Sets the `nearText` value in the explore query filters.
    ///
    /// One of either `with_near_text` or `with_near_vector` must be set in the query at point of
    /// build.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_text(mut self, near_text: impl Into<String>) -> ExploreQuery {
        self.near_text = Some(near_text.into());
        self
    }

    /// Sets the `nearVector` value in the explore query filters.
    ///
    /// One of either `with_near_text` or `with_near_vector` must be set in the query at point of
    /// build.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_vector(mut self, near_vector: impl Into<String>) -> ExploreQuery {
        self.near_vector = Some(near_vector.into());
        self
    }

    fn contains_filters(&self) -> bool {
        self.near_text.is_some() || self.near_vector.is_some()
    }

    /// Convert the `ExploreQuery` into a `serde_json::Value` payload.
    pub fn as_payload(&self) -> Result<serde_json::Value, QueryError> {
        if !self.contains_filters() {
            return Err(QueryError::MissingField(
                "ExploreQuery must contain either nearText or nearVector".into(),
            ));
        }

        Ok(json!({ "query": self.to_string() }))
    }
}

impl Default for ExploreQuery {
    fn default() -> Self {
        ExploreQuery::new()
    }
}

impl std::fmt::Display for ExploreQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Path
        writeln!(f, "{{")?;
        writeln!(f, "  Explore")?;

        // Filters
        if self.contains_filters() {
            writeln!(f, "  (")?;
            if let Some(limit) = &self.limit {
                writeln!(f, "    limit: {limit}")?;
            }
            if let Some(near_text) = &self.near_text {
                writeln!(f, "    nearText: {near_text}")?;
            }
            if let Some(near_vector) = &self.near_vector {
                writeln!(f, "    nearVector: {near_vector}")?;
            }
            writeln!(f, "  )")?;
        }

        // Body
        writeln!(f, "  {{")?;
        if let Some(fields) = &self.fields {
            writeln!(f, "    {}", fields.join(" "))?;
        }
        writeln!(f, "  }}")?;
        write!(f, "}}")
    }
}
