use serde_json::json;

/// AggregatorQuery struct to hold an Aggregate query.
#[derive(Debug)]
pub struct AggregateQuery {
    pub class_name: String,
    pub object_limit: Option<u32>,
    pub meta_count: Option<bool>,
    pub fields: Option<Vec<String>>,
    pub where_clause: Option<String>,
    pub group_by: Option<String>,
    pub near: Option<String>,
    pub tenant: Option<String>,
    pub limit: Option<u32>,
}

impl AggregateQuery {
    /// Create a new AggregateQuery item.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::AggregateQuery;
    ///
    /// let query_builder = AggregateQuery::new("Article");
    /// ```
    pub fn new(class_name: &str) -> Self {
        Self {
            class_name: class_name.into(),
            object_limit: None,
            meta_count: None,
            fields: None,
            where_clause: None,
            group_by: None,
            near: None,
            tenant: None,
            limit: None,
        }
    }

    /// Set the `objectLimit: <value>` as a filter to the query to limit the vector search results
    /// used within the aggregation.
    ///
    /// Should only be set in conjunction when used in conjunction with a `near` filter (for
    /// example, `with_near_text()`
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::AggregateQuery;
    ///
    /// let query_builder = AggregateQuery::new("Article")
    ///     .with_object_limit(1);
    /// ```
    pub fn with_object_limit(mut self, value: u32) -> Self {
        self.object_limit = Some(value);
        self
    }

    /// Add `meta{count}` to the body of the query when called.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::AggregateQuery;
    ///
    /// let query_builder = AggregateQuery::new("Article")
    ///     .with_meta_count();
    /// ```
    pub fn with_meta_count(mut self) -> Self {
        self.meta_count = Some(true);
        self
    }

    /// Appends the specified fields in the aggregate query body.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::AggregateQuery;
    ///
    /// let query_builder = AggregateQuery::new("Article")
    ///     .with_fields(vec!["wordCount { mean }"]);
    /// ```
    pub fn with_fields(mut self, fields: Vec<&str>) -> Self {
        let fields = fields.iter().map(|field| field.to_string()).collect();
        self.fields = Some(fields);
        self
    }

    /// Set the `where` filter in the aggregate query.
    ///
    /// # Example -> todo
    /// ```
    /// ```
    pub fn with_where(mut self, where_clause: &str) -> Self {
        self.where_clause = Some(where_clause.into());
        self
    }

    /// Set the `group_by` filter in the aggregate query.
    ///
    /// This may also require `groupedBy {...}` to be specified in with_fields().
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::AggregateQuery;
    ///
    /// let query_builder = AggregateQuery::new("Article")
    ///     .with_group_by_filter("[\"inPublication\"]")
    ///     .with_fields(vec!["groupedBy {value path}"]);
    /// ```
    pub fn with_group_by_filter(mut self, group_by: impl Into<String>) -> Self {
        self.group_by = Some(group_by.into());
        self
    }

    /// Set the `nearText` filter in the aggregate query. This filter can be used with text modules
    /// (text2vec).
    ///
    /// Note that the `autocorrect` field is only available with the `text-spellcheck` Weaviate
    /// module.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_text(mut self, near_text: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_text.into());
        self
    }

    /// Set the `nearVector` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_vector(mut self, near_vector: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_vector.into());
        self
    }

    /// Set the `nearObject` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_object(mut self, near_object: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_object.into());
        self
    }

    /// Set the `nearImage` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_image(mut self, near_image: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_image.into());
        self
    }

    /// Set the `nearAudio` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_audio(mut self, near_audio: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_audio.into());
        self
    }

    /// Set the `nearVideo` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_video(mut self, near_video: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_video.into());
        self
    }

    /// Set the `nearDepth` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_depth(mut self, near_depth: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_depth.into());
        self
    }

    /// Set the `nearThermal` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_thermal(mut self, near_thermal: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_thermal.into());
        self
    }

    /// Set the `nearIMU` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_imu(mut self, near_imu: impl Into<String>) -> Self {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_imu.into());
        self
    }

    /// Set the `tenant` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_tenant(mut self, tenant: impl Into<String>) -> Self {
        self.tenant = Some(tenant.into());
        self
    }

    /// Set the `limit` filter in the aggregate query.
    ///
    /// Limits the number of results that are returned.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::AggregateQuery;
    ///
    /// let query_builder = AggregateQuery::new("Article")
    ///     .with_limit(1);
    /// ```
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Check if the query contains a filter.
    fn contains_filter(&self) -> bool {
        self.where_clause.is_some()
            || self.group_by.is_some()
            || self.near.is_some()
            || self.object_limit.is_some()
            || self.tenant.is_some()
            || self.limit.is_some()
    }

    /// Convert the `AggregateQuery` into a `serde_json::Value` payload.
    pub fn as_payload(&self) -> serde_json::Value {
        json!({ "query": self.to_string() })
    }
}

impl std::fmt::Display for AggregateQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "  Aggregate {{")?;
        writeln!(f, "    {}", self.class_name)?;

        if self.contains_filter() {
            writeln!(f, "    (")?;
            if let Some(where_clause) = &self.where_clause {
                writeln!(f, "      where: {where_clause}")?;
            }
            if let Some(group_by) = &self.group_by {
                writeln!(f, "      groupBy: {group_by}")?;
            }
            if let Some(near) = &self.near {
                writeln!(f, "      near: {near}")?;
            }
            if let Some(object_limit) = &self.object_limit {
                writeln!(f, "      objectLimit: {object_limit}")?;
            }
            if let Some(tenant) = &self.tenant {
                writeln!(f, "      tenant: {tenant}")?;
            }
            if let Some(limit) = &self.limit {
                writeln!(f, "      limit: {limit}")?;
            }
            writeln!(f, "    )")?;
        }

        writeln!(f, "    {{")?;
        if self.meta_count.is_some() {
            writeln!(f, "      meta {{ count }}")?;
        }
        if let Some(fields) = &self.fields {
            writeln!(f, "      {}", fields.join(" "))?;
        }
        writeln!(f, "    }}")?;
        writeln!(f, "  }}")?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_aggregate_query() {
        let query = AggregateQuery::new("Article")
            .with_meta_count()
            .with_fields(vec!["wordCount { count }"])
            .with_limit(10);

        let expected = indoc! {"
            {
              Aggregate {
                Article
                (
                  limit: 10
                )
                {
                  meta { count }
                  wordCount { count }
                }
              }
            }"
        };

        assert_eq!(query.to_string(), expected);
    }
}
