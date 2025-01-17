// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TestConnection`](crate::operation::test_connection::builders::TestConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_instance_arn(impl ::std::convert::Into<String>)`](crate::operation::test_connection::builders::TestConnectionFluentBuilder::replication_instance_arn) / [`set_replication_instance_arn(Option<String>)`](crate::operation::test_connection::builders::TestConnectionFluentBuilder::set_replication_instance_arn): <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    ///   - [`endpoint_arn(impl ::std::convert::Into<String>)`](crate::operation::test_connection::builders::TestConnectionFluentBuilder::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::operation::test_connection::builders::TestConnectionFluentBuilder::set_endpoint_arn): <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    /// - On success, responds with [`TestConnectionOutput`](crate::operation::test_connection::TestConnectionOutput) with field(s):
    ///   - [`connection(Option<Connection>)`](crate::operation::test_connection::TestConnectionOutput::connection): <p>The connection tested.</p>
    /// - On failure, responds with [`SdkError<TestConnectionError>`](crate::operation::test_connection::TestConnectionError)
    pub fn test_connection(&self) -> crate::operation::test_connection::builders::TestConnectionFluentBuilder {
        crate::operation::test_connection::builders::TestConnectionFluentBuilder::new(self.handle.clone())
    }
}
