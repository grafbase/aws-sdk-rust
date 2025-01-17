// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAwsLogSource`](crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sources(Vec<AwsLogSourceConfiguration>)`](crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceFluentBuilder::sources) / [`set_sources(Option<Vec<AwsLogSourceConfiguration>>)`](crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceFluentBuilder::set_sources): <p>Specify the natively-supported Amazon Web Services service to remove as a source in Security Lake.</p>
    /// - On success, responds with [`DeleteAwsLogSourceOutput`](crate::operation::delete_aws_log_source::DeleteAwsLogSourceOutput) with field(s):
    ///   - [`failed(Option<Vec<String>>)`](crate::operation::delete_aws_log_source::DeleteAwsLogSourceOutput::failed): <p>Deletion of the Amazon Web Services sources failed as the account is not a part of the organization.</p>
    /// - On failure, responds with [`SdkError<DeleteAwsLogSourceError>`](crate::operation::delete_aws_log_source::DeleteAwsLogSourceError)
    pub fn delete_aws_log_source(&self) -> crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceFluentBuilder {
        crate::operation::delete_aws_log_source::builders::DeleteAwsLogSourceFluentBuilder::new(self.handle.clone())
    }
}
