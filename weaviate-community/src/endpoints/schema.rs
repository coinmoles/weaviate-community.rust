use reqwest::{StatusCode, Url};

use crate::error::WeaviateError;
use crate::models::schema::{
    Class, Classes, Property, Shard, ShardStatus, Shards, Tenant, Tenants,
};
use crate::{ResponseExt, WeaviateClient};

/// All schema related endpoints and functionality described in
/// [Weaviate schema API documentation](https://weaviate.io/developers/weaviate/api/rest/schema)
#[derive(Debug)]
pub struct Schema<'a> {
    client: &'a WeaviateClient,
}

impl<'a> Schema<'a> {
    /// Create a new Schema object. The schema object is intended to like inside the WeaviateClient
    /// and be called through the WeaviateClient.
    pub(crate) fn new(client: &'a WeaviateClient) -> Self {
        Schema { client }
    }

    /// Get the endpoint for schema
    ///
    /// # Returns
    /// A `Result` containing the URL for the schema endpoint or a `ParseError` if the URL is invalid.
    ///
    /// An `Err` variant should not occur as the `base_url` is validated during the `WeaviateClient` creation.
    fn endpoint(&self) -> Result<Url, url::ParseError> {
        self.client.base_url.join("/v1/schema/")
    }

    /// Facilitates the retrieval of the configuration for a single class in the schema.
    ///
    /// GET /v1/schema/{class_name}
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let response = client.schema().get_class("Library").await;
    ///     assert!(response.is_err());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_class(&self, class_name: &str) -> Result<Class, WeaviateError> {
        let endpoint = self.endpoint()?.join(class_name)?;
        let res: Class = self
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

    /// Facilitates the retrieval of the full Weaviate schema.
    ///
    /// GET /v1/schema
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let schema = client.schema().get().await?;
    ///     println!("{:#?}", &schema);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self) -> Result<Classes, WeaviateError> {
        let endpoint = self.endpoint()?;
        let res: Classes = self
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

    /// Create a new data object class in the schema.
    ///
    /// Note that from 1.5.0, creating a schema is optional, as Auto Schema is available. See for
    /// more info:
    /// [Weaviate auto-schema documentation](https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema)
    ///
    /// POST /v1/schema
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::models::schema::Class;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let class = Class::builder("Library").build();
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let res = client.schema().create_class(&class).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_class(&self, class: &Class) -> Result<Class, WeaviateError> {
        let endpoint = self.endpoint()?;
        let payload = serde_json::to_value(class).unwrap();
        let res: Class = self
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

    ///
    /// Remove a class (and all data in the instances) from the schema.
    ///
    /// DELETE v1/schema/{class_name}
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let response = client.schema().delete("Library").await;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn delete(&self, class_name: &str) -> Result<bool, WeaviateError> {
        let endpoint = self.endpoint()?.join(class_name)?;
        let _res = self
            .client
            .delete(endpoint)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?;
        Ok(true)
    }

    /// Update settings of an existing schema class.
    ///
    /// Use this endpoint to alter an existing class in the schema. Note that not all settings are
    /// mutable. If an error about immutable fields is returned and you still need to update this
    /// particular setting, you will have to delete the class (and the underlying data) and
    /// recreate. This endpoint cannot be used to modify properties.
    //  Instead, use POST /v1/schema/{ClassName}/properties (add_property method).
    //
    /// A typical use case for this endpoint is to update configuration, such as the
    /// vectorIndexConfig. Note that even in mutable sections, such as vectorIndexConfig,
    /// some fields may be immutable.
    ///
    /// You should attach a body to this PUT request with the entire new configuration of the class
    pub async fn update(&self, class: &Class) -> Result<Class, WeaviateError> {
        let endpoint = self.endpoint()?.join(&class.class)?;
        let payload = serde_json::to_value(class)?;
        let res: Class = self
            .client
            .put(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    ///
    /// Add a property to an existing class in the schema.
    ///
    pub async fn add_property(
        &self,
        class_name: &str,
        property: &Property,
    ) -> Result<Property, WeaviateError> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/properties");
        let endpoint = self.endpoint()?.join(&endpoint)?;
        let payload = serde_json::to_value(property)?;
        let res: Property = self
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

    ///
    /// View all of the shards for a particular class.
    ///
    pub async fn get_shards(&self, class_name: &str) -> Result<Shards, WeaviateError> {
        let path = format!("{class_name}/shards");
        let endpoint = self.endpoint()?.join(&path)?;
        let res: Vec<Shard> = self
            .client
            .get(endpoint)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(Shards { shards: res })
    }

    ///
    /// Update shard status
    ///
    pub async fn update_class_shard(
        &self,
        class_name: &str,
        shard_name: &str,
        status: ShardStatus,
    ) -> Result<Shard, WeaviateError> {
        let path = format!("{class_name}/shards/{shard_name}");
        let endpoint = self.endpoint()?.join(&path)?;
        let payload = serde_json::json!({ "status": status });
        let _res = self
            .client
            .put(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?;
        Ok(Shard {
            name: shard_name.into(),
            status,
        })
    }

    ///
    /// List tenants
    ///
    pub async fn list_tenants(&self, class_name: &str) -> Result<Tenants, WeaviateError> {
        let path = format!("{class_name}/tenants");
        let endpoint = self.endpoint()?.join(&path)?;
        let res: Vec<Tenant> = self
            .client
            .get(endpoint)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(Tenants { tenants: res })
    }

    ///
    /// Add tenant
    ///
    pub async fn add_tenants(
        &self,
        class_name: &str,
        tenants: &Tenants,
    ) -> Result<Tenants, WeaviateError> {
        let path = format!("{class_name}/tenants");
        let endpoint = self.endpoint()?.join(&path)?;
        let payload = serde_json::to_value(&tenants.tenants)?;
        let res: Vec<Tenant> = self
            .client
            .post(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(Tenants { tenants: res })
    }

    ///
    /// Remove tenants
    ///
    pub async fn remove_tenants(
        &self,
        class_name: &str,
        tenants: &Vec<&str>,
    ) -> Result<bool, WeaviateError> {
        let path = format!("{class_name}/tenants");
        let endpoint = self.endpoint()?.join(&path)?;
        let payload = serde_json::to_value(tenants)?;
        let _res = self
            .client
            .delete(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?;
        Ok(true)
    }

    ///
    /// Update tenants
    ///
    /// For updating tenants, both `name` and `activity_status` are required.
    ///
    /// Note that tenant activity status setting is only available from Weaviate v1.21
    ///
    pub async fn update_tenants(
        &self,
        class_name: &str,
        tenants: &Tenants,
    ) -> Result<Tenants, WeaviateError> {
        let path = format!("{class_name}/tenants");
        let endpoint = self.endpoint()?.join(&path)?;
        let payload = serde_json::to_value(&tenants.tenants)?;
        let res: Vec<Tenant> = self
            .client
            .put(endpoint)
            .json(&payload)
            .send()
            .await?
            .check_status(StatusCode::OK)
            .await?
            .json()
            .await?;
        Ok(Tenants { tenants: res })
    }
}

#[cfg(test)]
mod tests {
    // Tests currently require a weaviate instance to be running on localhost, as I have not yet
    // implemented anything to mock the database. In future, actual tests will run as integration
    // tests in a container as part of the CICD process.
    use crate::models::schema::{
        ActivityStatus, Class, ClassBuilder, Classes, Property, Shard, ShardStatus, Shards, Tenant,
        Tenants,
    };
    use crate::WeaviateClient;

    /// Helper function for generating a testing class
    fn test_class(class_name: &str) -> Class {
        ClassBuilder::new(class_name)
            .with_description("Test")
            .build()
    }

    fn test_classes() -> Classes {
        let class_a = test_class("Test1");
        let class_b = test_class("Test1");
        Classes::new(vec![class_a, class_b])
    }

    fn test_shard() -> Shard {
        Shard::new("abcd", ShardStatus::READY)
    }

    /// Helper function for generating a testing property
    fn test_property(property_name: &str) -> Property {
        Property::builder(property_name, vec!["boolean"])
            .with_description("test property")
            .build()
    }

    /// Helper function for generating some test tenants, as shown on the weaviate API webpage.
    fn test_tenants() -> Tenants {
        Tenants::new(vec![
            Tenant::builder("TENANT_A").build(),
            Tenant::builder("TENANT_B")
                .with_activity_status(ActivityStatus::COLD)
                .build(),
        ])
    }

    fn test_shards() -> Shards {
        Shards::new(vec![Shard::new("1D3PBjtz9W7r", ShardStatus::READY)])
    }

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

    async fn mock_put(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str,
    ) -> mockito::Mock {
        server
            .mock("PUT", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
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

    async fn mock_delete(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
    ) -> mockito::Mock {
        server
            .mock("DELETE", endpoint)
            .with_status(status_code)
            .create()
    }

    #[tokio::test]
    async fn test_create_class_ok() {
        let class = test_class("UnitClass");
        let class_str = serde_json::to_string(&class).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/schema/", 200, &class_str).await;
        let res = client.schema().create_class(&class).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(class.class, res.unwrap().class);
    }

    #[tokio::test]
    async fn test_create_class_err() {
        let class = test_class("UnitClass");
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/schema/", 401, "").await;
        let res = client.schema().create_class(&class).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_get_all_classes_ok() {
        let classes = test_classes();
        let class_str = serde_json::to_string(&classes).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/", 200, &class_str).await;
        let res = client.schema().get().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(classes.classes[0].class, res.unwrap().classes[0].class);
    }

    #[tokio::test]
    async fn test_get_all_classes_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/", 401, "").await;
        let class = client.schema().get().await;
        mock.assert();
        assert!(class.is_err());
    }

    #[tokio::test]
    async fn test_get_single_class_ok() {
        let class = test_class("Test");
        let class_str = serde_json::to_string(&class).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/Test", 200, &class_str).await;
        let res = client.schema().get_class("Test").await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(class.class, res.unwrap().class);
    }

    #[tokio::test]
    async fn test_get_single_class_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/Test", 401, "").await;
        let class = client.schema().get_class("Test").await;
        mock.assert();
        assert!(class.is_err());
    }

    #[tokio::test]
    async fn test_get_delete_class_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_delete(&mut mock_server, "/v1/schema/Test", 200).await;
        let res = client.schema().delete("Test").await;
        mock.assert();
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn test_get_delete_class_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_delete(&mut mock_server, "/v1/schema/Test", 401).await;
        let class = client.schema().delete("Test").await;
        mock.assert();
        assert!(class.is_err());
    }

    #[tokio::test]
    async fn test_update_class_ok() {
        let class = test_class("Test");
        let class_str = serde_json::to_string(&class).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_put(&mut mock_server, "/v1/schema/Test", 200, &class_str).await;
        let res = client.schema().update(&class).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(class.class, res.unwrap().class);
    }

    #[tokio::test]
    async fn test_update_class_err() {
        let class = test_class("Test");
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_put(&mut mock_server, "/v1/schema/Test", 401, "").await;
        let res = client.schema().update(&class).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_add_property_ok() {
        let property = test_property("Test");
        let property_str = serde_json::to_string(&property).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(
            &mut mock_server,
            "/v1/schema/TestClass/properties",
            200,
            &property_str,
        )
        .await;
        let res = client.schema().add_property("TestClass", &property).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(property.name, res.unwrap().name);
    }

    #[tokio::test]
    async fn test_add_property_err() {
        let property = test_property("Test");
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/schema/TestClass/properties", 401, "").await;
        let res = client.schema().add_property("TestClass", &property).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_get_shards_ok() {
        let shards = test_shards();
        let shards_str = serde_json::to_string(&shards.shards).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/Test/shards", 200, &shards_str).await;
        let res = client.schema().get_shards("Test").await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(shards.shards[0].name, res.unwrap().shards[0].name);
    }

    #[tokio::test]
    async fn test_get_shards_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/Test/shards", 401, "").await;
        let res = client.schema().get_shards("Test").await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_update_class_shard_ok() {
        let shard = test_shard();
        let shard_str = serde_json::to_string(&shard).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_put(
            &mut mock_server,
            "/v1/schema/Test/shards/abcd",
            200,
            &shard_str,
        )
        .await;
        let res = client
            .schema()
            .update_class_shard("Test", "abcd", ShardStatus::READONLY)
            .await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(shard.name, res.unwrap().name);
    }

    #[tokio::test]
    async fn test_update_class_shard_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_put(&mut mock_server, "/v1/schema/Test/shards/abcd", 401, "").await;
        let res = client
            .schema()
            .update_class_shard("Test", "abcd", ShardStatus::READONLY)
            .await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_list_tenants_ok() {
        let tenants = test_tenants();
        let tenants_str = serde_json::to_string(&tenants.tenants).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(
            &mut mock_server,
            "/v1/schema/Test/tenants",
            200,
            &tenants_str,
        )
        .await;
        let res = client.schema().list_tenants("Test").await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(tenants.tenants[0].name, res.unwrap().tenants[0].name);
    }

    #[tokio::test]
    async fn test_list_tenants_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/schema/Test/tenants", 422, "").await;
        let res = client.schema().list_tenants("Test").await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_add_tenants_ok() {
        let tenants = test_tenants();
        let tenants_str = serde_json::to_string(&tenants.tenants).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(
            &mut mock_server,
            "/v1/schema/Test/tenants",
            200,
            &tenants_str,
        )
        .await;
        let res = client.schema().add_tenants("Test", &tenants).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(tenants.tenants[0].name, res.unwrap().tenants[0].name);
    }

    #[tokio::test]
    async fn test_add_tenants_err() {
        let tenants = test_tenants();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/schema/Test/tenants", 422, "").await;
        let res = client.schema().add_tenants("Test", &tenants).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_remove_tenants_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_delete(&mut mock_server, "/v1/schema/Test/tenants", 200).await;
        let res = client
            .schema()
            .remove_tenants("Test", &vec!["TestTenant"])
            .await;
        mock.assert();
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn test_remove_tenants_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_delete(&mut mock_server, "/v1/schema/Test/tenants", 422).await;
        let res = client
            .schema()
            .remove_tenants("Test", &vec!["TestTenant"])
            .await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_update_tenants_ok() {
        let tenants = test_tenants();
        let tenants_str = serde_json::to_string(&tenants.tenants).unwrap();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_put(
            &mut mock_server,
            "/v1/schema/Test/tenants",
            200,
            &tenants_str,
        )
        .await;
        let res = client.schema().update_tenants("Test", &tenants).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(tenants.tenants[0].name, res.unwrap().tenants[0].name);
    }

    #[tokio::test]
    async fn test_update_tenants_err() {
        let tenants = test_tenants();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_put(&mut mock_server, "/v1/schema/Test/tenants", 422, "").await;
        let res = client.schema().update_tenants("Test", &tenants).await;
        mock.assert();
        assert!(res.is_err());
    }
}
