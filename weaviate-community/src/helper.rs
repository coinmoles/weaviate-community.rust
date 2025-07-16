use crate::error::WeaviateError;

pub(crate) trait ResponseExt {
    /// Checks the status code of the response against an expected status code.
    /// If the status code matches, it returns the response.
    /// If it does not match, it returns a `WeaviateError` with details about the mismatch.
    fn check_status(
        self,
        expected: reqwest::StatusCode,
    ) -> Result<reqwest::Response, WeaviateError>;
}

impl ResponseExt for reqwest::Response {
    fn check_status(
        self,
        expected: reqwest::StatusCode,
    ) -> Result<reqwest::Response, WeaviateError> {
        let actual = self.status();
        if actual == expected {
            return Ok(self);
        }
        let url = self.url().clone();
        let reason = self.extensions().get::<hyper::ext::ReasonPhrase>().cloned();
        Err(WeaviateError::UnexpectedStatusCode {
            url,
            expected,
            actual,
            reason,
        })
    }
}
