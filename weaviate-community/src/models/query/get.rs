use serde_json::json;
use uuid::Uuid;

/// GetQuery struct to hold a Get query.
#[derive(Debug)]
pub struct GetQuery {
    pub class_name: String,
    pub properties: Vec<String>,
    pub additional: Option<Vec<String>>,
    pub where_clause: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub after: Option<Uuid>, // cant use with where, near<media>, bm25, hybrid, etc
    pub near_text: Option<String>,
    pub near_vector: Option<String>,
    pub near_image: Option<String>,
    pub near_object: Option<String>,
    pub near_video: Option<String>,
    pub near_audio: Option<String>,
    pub near_thermal: Option<String>,
    pub near_imu: Option<String>,
    pub near_depth: Option<String>,
    pub sort: Option<String>,
    pub bm25: Option<String>,
    pub hybrid: Option<String>,
    pub group_by: Option<String>,
    pub tenant: Option<String>,
    pub autocut: Option<u32>,
    pub ask: Option<String>,
}

impl GetQuery {
    /// Create a new GetQuery item.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// );
    /// ```
    pub fn new(class_name: impl Into<String>, properties: Vec<&str>) -> Self {
        Self {
            class_name: class_name.into(),
            properties: properties.iter().map(|prop| prop.to_string()).collect(),
            limit: None,
            offset: None,
            additional: None,
            tenant: None,
            autocut: None,
            after: None,
            sort: None,
            where_clause: None,
            near_text: None,
            near_vector: None,
            near_image: None,
            near_object: None,
            near_video: None,
            near_audio: None,
            near_thermal: None,
            near_imu: None,
            near_depth: None,
            hybrid: None,
            bm25: None,
            ask: None,
            group_by: None,
        }
    }

    /// Sets the `limit` in the get query filters.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// ).with_limit(1);
    /// ```
    pub fn with_limit(mut self, limit: u32) -> GetQuery {
        self.limit = Some(limit);
        self
    }

    /// Set the `offset` in the get query filters.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// )
    ///     .with_limit(1)
    ///     .with_offset(1);
    /// ```
    pub fn with_offset(mut self, offset: u32) -> GetQuery {
        self.offset = Some(offset);
        self
    }

    /// Specify the `_additional` properties to retrieve in the query result.
    ///
    /// Note that the additional properties are properties that cannot be specified in the regular
    /// properties field, such as the `vector`, or the object UUID (`id`). More `_additional`
    /// properties are described [here](https://weaviate.io/developers/weaviate/api/graphql/additional-properties).
    ///
    /// Cross referenced properties should be specified in the regular properties field (in the
    /// `new` method).
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new("JeopardyQuestion", vec![])
    ///     .with_additional(vec!["vector"]);
    /// ```
    pub fn with_additional(mut self, additional: Vec<&str>) -> GetQuery {
        let additional = additional.iter().map(|item| item.to_string()).collect();
        self.additional = Some(additional);
        self
    }

    /// Specify the `tenant` in the get query filter.
    ///
    /// For classes that have multi-tenancy enabled, the tenant parameter must be specified in each
    /// query.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new("JeopardyQuestion", vec!["answer"])
    ///     .with_tenant("tenantA");
    /// ```
    pub fn with_tenant(mut self, tenant: impl Into<String>) -> GetQuery {
        self.tenant = Some(tenant.into());
        self
    }

    /// Specify the `autocut` search filter in the get query.
    ///
    /// The `autocut` filter is an argument that can be added to class objects retrieved by the
    /// `near<media>`, `bm25`, and `hybrid` operators.
    ///
    /// More information on `autocut` can be found [here](https://weaviate.io/developers/weaviate/api/graphql/additional-operators#autocut)
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new("JeopardyQuestion", vec!["question", "answer"])
    ///     .with_hybrid("{query: \"food\"}")
    ///     .with_autocut(1);
    /// ```
    pub fn with_autocut(mut self, autocut: u32) -> GetQuery {
        self.autocut = Some(autocut);
        self
    }

    /// Specify the `after` search filter in the get query.
    ///
    /// The `after` operator can be used to sequentially retrieve class objects from Weaviate.
    ///
    /// More information on `after` can be found [here](https://weaviate.io/developers/weaviate/api/graphql/additional-operators#cursor-with-after)
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_after(mut self, after: Uuid) -> GetQuery {
        self.after = Some(after);
        self
    }

    /// Specify the `sort` search filter in the get query.
    ///
    /// Any primitive property types can be sorted, such as `text`, `string`, `number`, or `int`.
    ///
    /// When a query has a natural order (e.g. because of a near<media> vector search), adding a
    /// sort operator will override that order.
    ///
    /// More on sorting in Weaviate can be found [here](https://weaviate.io/developers/weaviate/api/graphql/additional-operators#sorting)
    pub fn with_sort(mut self, sort: impl Into<String>) -> GetQuery {
        self.sort = Some(sort.into());
        self
    }

    /// Specify conditionals to add to the `where` search filter in the get query.
    ///
    /// More information on conditionals can be found [here](https://weaviate.io/developers/weaviate/api/graphql/filters)
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_where(mut self, where_clause: impl Into<String>) -> GetQuery {
        self.where_clause = Some(where_clause.into());
        self
    }

    /// Set the `nearText` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_text(mut self, near_text: impl Into<String>) -> GetQuery {
        self.near_text = Some(near_text.into());
        self
    }

    /// Set the `nearVector` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_vector(mut self, near_vector: impl Into<String>) -> GetQuery {
        self.near_vector = Some(near_vector.into());
        self
    }

    /// Set the `nearObject` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_object(mut self, near_object: impl Into<String>) -> GetQuery {
        self.near_object = Some(near_object.into());
        self
    }

    /// Set the `nearImage` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_image(mut self, near_image: impl Into<String>) -> GetQuery {
        self.near_image = Some(near_image.into());
        self
    }

    /// Set the `nearVideo` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_video(mut self, near_video: impl Into<String>) -> GetQuery {
        self.near_video = Some(near_video.into());
        self
    }

    /// Set the `nearAudio` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_audio(mut self, near_audio: impl Into<String>) -> GetQuery {
        self.near_audio = Some(near_audio.into());
        self
    }

    /// Set the `nearThermal` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_thermal(mut self, near_thermal: impl Into<String>) -> GetQuery {
        self.near_thermal = Some(near_thermal.into());
        self
    }

    /// Set the `nearIMU` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_imu(mut self, near_imu: impl Into<String>) -> GetQuery {
        self.near_imu = Some(near_imu.into());
        self
    }

    /// Set the `nearDepth` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_depth(mut self, near_depth: impl Into<String>) -> GetQuery {
        self.near_depth = Some(near_depth.into());
        self
    }

    /// Specify the `hybrid` search filter in the get query.
    ///
    /// The `hybrid` operator produces results based on a weighted combination of results from a
    /// keyword (bm25) search and a vector (near<media>) search.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new("JeopardyQuestion", vec!["question", "answer"])
    ///     .with_hybrid("{query: \"food\"}")
    ///     .with_limit(3);
    /// ```
    ///
    /// This will generate the following GetQuery:
    /// ```text
    /// GetQuery {
    ///   query: "{
    ///     Get {
    ///       JeopardyQuestion
    ///       (
    ///         limit: 3
    ///         hybrid: {query: "food"]
    ///       )
    ///       {
    ///         question
    ///         answer
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    pub fn with_hybrid(mut self, hybrid: impl Into<String>) -> GetQuery {
        self.hybrid = Some(hybrid.into());
        self
    }

    /// Specify the `bm25` search filter in the get query.
    ///
    /// To use BM25 search, you must provide a search string as a minimum.
    ///
    /// More on Keyword (BM25) search can be found [here](https://weaviate.io/developers/weaviate/search/bm25)
    ///
    /// # Example
    /// ```
    /// use weaviate_community::models::query::GetQuery;
    ///
    /// let query_builder = GetQuery::new("JeopardyQuestion", vec!["question", "answer"])
    ///     .with_bm25("{query: \"food\"}")
    ///     .with_limit(3);
    /// ```
    ///
    /// This will generate the following GetQuery:
    /// ```text
    /// GetQuery {
    ///   query: "{
    ///     Get {
    ///       JeopardyQuestion
    ///       (
    ///         limit: 3
    ///         bm25: {query: "food"]
    ///       )
    ///       {
    ///         question
    ///         answer
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    /// and would look for objects containing the keyword `food` anywhere in the object if ran.
    pub fn with_bm25(mut self, bm25: impl Into<String>) -> GetQuery {
        self.bm25 = Some(bm25.into());
        self
    }

    /// Specify the `groupBy` in the get query filters.
    ///
    /// To use `groupBy`:
    /// - Provide the property by which the results should be grouped,
    /// - The maximum number of groups, and
    /// - The maximum number of objects per group
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_group_by(mut self, group_by: impl Into<String>) -> GetQuery {
        self.group_by = Some(group_by.into());
        self
    }

    pub fn with_ask(mut self, ask: impl Into<String>) -> GetQuery {
        self.ask = Some(ask.into());
        self
    }

    /// Check if the query contains a filter.
    fn contains_filter(&self) -> bool {
        self.limit.is_some()
            || self.offset.is_some()
            || self.after.is_some()
            || self.autocut.is_some()
            || self.tenant.is_some()
            || self.where_clause.is_some()
            || self.near_text.is_some()
            || self.near_vector.is_some()
            || self.near_image.is_some()
            || self.near_object.is_some()
            || self.hybrid.is_some()
            || self.bm25.is_some()
            || self.sort.is_some()
            || self.ask.is_some()
    }

    /// Convert the `GetQuery` into a `serde_json::Value` payload.
    pub fn as_payload(&self) -> serde_json::Value {
        json!({ "query": self.to_string() })
    }
}

impl std::fmt::Display for GetQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "  Get {{")?;
        writeln!(f, "    {}", self.class_name)?;

        if self.contains_filter() {
            writeln!(f, "    (")?;
            if let Some(where_clause) = &self.where_clause {
                writeln!(f, "      where: {where_clause}")?;
            }
            if let Some(limit) = &self.limit {
                writeln!(f, "      limit: {limit}")?;
            }
            if let Some(offset) = &self.offset {
                writeln!(f, "      offset: {offset}")?;
            }
            if let Some(near_text) = &self.near_text {
                writeln!(f, "      nearText: {near_text}")?;
            }
            if let Some(near_vector) = &self.near_vector {
                writeln!(f, "      nearVector: {near_vector}")?;
            }
            if let Some(near_object) = &self.near_object {
                writeln!(f, "      nearObject: {near_object}")?;
            }
            if let Some(near_image) = &self.near_image {
                writeln!(f, "      nearImage: {near_image}")?;
            }
            if let Some(near_video) = &self.near_video {
                writeln!(f, "      nearVideo: {near_video}")?;
            }
            if let Some(near_audio) = &self.near_audio {
                writeln!(f, "      nearAudio: {near_audio}")?;
            }
            if let Some(near_thermal) = &self.near_thermal {
                writeln!(f, "      nearThermal: {near_thermal}")?;
            }
            if let Some(near_imu) = &self.near_imu {
                writeln!(f, "      nearIMU: {near_imu}")?;
            }
            if let Some(near_depth) = &self.near_depth {
                writeln!(f, "      nearDepth: {near_depth}")?;
            }
            if let Some(bm25) = &self.bm25 {
                writeln!(f, "      bm25: {bm25}")?;
            }
            if let Some(hybrid) = &self.hybrid {
                writeln!(f, "      hybrid: {hybrid}")?;
            }
            if let Some(group_by) = &self.group_by {
                writeln!(f, "      groupBy: {group_by}")?;
            }
            if let Some(tenant) = &self.after {
                writeln!(f, "      after: \"{tenant}\"")?;
            }
            if let Some(autocut) = &self.autocut {
                writeln!(f, "      autocut: {autocut}")?;
            }
            if let Some(sort) = &self.sort {
                writeln!(f, "      sort: {sort}")?;
            }
            if let Some(ask) = &self.ask {
                writeln!(f, "      ask: {ask}")?;
            }
            writeln!(f, "    )")?;
        }

        // Body
        writeln!(f, "    {{")?;
        for property in &self.properties {
            writeln!(f, "      {property}")?;
        }

        if let Some(additional) = &self.additional {
            writeln!(f, "      _additional {{")?;
            for item in additional {
                writeln!(f, "        {item}")?;
            }
            writeln!(f, "      }}")?;
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
    fn test_get_query() {
        let query = GetQuery::new(
            "JeopardyQuestion",
            vec![
                "question",
                "answer",
                "points",
                "hasCategory { ... on JeopardyCategory { title }}",
            ],
        )
        .with_limit(1)
        .with_offset(1);

        let expected = indoc! {"
            {
              Get {
                JeopardyQuestion
                (
                  limit: 1
                  offset: 1
                )
                {
                  question
                  answer
                  points
                  hasCategory { ... on JeopardyCategory { title }}
                }
              }
            }"
        };

        assert_eq!(query.to_string(), expected);
    }
}
