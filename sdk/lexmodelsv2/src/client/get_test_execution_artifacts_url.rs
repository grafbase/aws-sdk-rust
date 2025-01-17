// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTestExecutionArtifactsUrl`](crate::operation::get_test_execution_artifacts_url::builders::GetTestExecutionArtifactsUrlFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`test_execution_id(impl ::std::convert::Into<String>)`](crate::operation::get_test_execution_artifacts_url::builders::GetTestExecutionArtifactsUrlFluentBuilder::test_execution_id) / [`set_test_execution_id(Option<String>)`](crate::operation::get_test_execution_artifacts_url::builders::GetTestExecutionArtifactsUrlFluentBuilder::set_test_execution_id): <p>The unique identifier of the completed test execution.</p>
    /// - On success, responds with [`GetTestExecutionArtifactsUrlOutput`](crate::operation::get_test_execution_artifacts_url::GetTestExecutionArtifactsUrlOutput) with field(s):
    ///   - [`test_execution_id(Option<String>)`](crate::operation::get_test_execution_artifacts_url::GetTestExecutionArtifactsUrlOutput::test_execution_id): <p>The unique identifier of the completed test execution.</p>
    ///   - [`download_artifacts_url(Option<String>)`](crate::operation::get_test_execution_artifacts_url::GetTestExecutionArtifactsUrlOutput::download_artifacts_url): <p>The pre-signed Amazon S3 URL to download completed test execution.</p>
    /// - On failure, responds with [`SdkError<GetTestExecutionArtifactsUrlError>`](crate::operation::get_test_execution_artifacts_url::GetTestExecutionArtifactsUrlError)
    pub fn get_test_execution_artifacts_url(
        &self,
    ) -> crate::operation::get_test_execution_artifacts_url::builders::GetTestExecutionArtifactsUrlFluentBuilder {
        crate::operation::get_test_execution_artifacts_url::builders::GetTestExecutionArtifactsUrlFluentBuilder::new(self.handle.clone())
    }
}
