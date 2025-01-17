// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteServiceNetwork`](crate::operation::delete_service_network::builders::DeleteServiceNetworkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_network_identifier(impl ::std::convert::Into<String>)`](crate::operation::delete_service_network::builders::DeleteServiceNetworkFluentBuilder::service_network_identifier) / [`set_service_network_identifier(Option<String>)`](crate::operation::delete_service_network::builders::DeleteServiceNetworkFluentBuilder::set_service_network_identifier): <p>The Amazon Resource Name (ARN) or ID of the service network.</p>
    /// - On success, responds with [`DeleteServiceNetworkOutput`](crate::operation::delete_service_network::DeleteServiceNetworkOutput)
    /// - On failure, responds with [`SdkError<DeleteServiceNetworkError>`](crate::operation::delete_service_network::DeleteServiceNetworkError)
    pub fn delete_service_network(&self) -> crate::operation::delete_service_network::builders::DeleteServiceNetworkFluentBuilder {
        crate::operation::delete_service_network::builders::DeleteServiceNetworkFluentBuilder::new(self.handle.clone())
    }
}
