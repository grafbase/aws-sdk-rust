// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteNetworkProfile`](crate::operation::delete_network_profile::builders::DeleteNetworkProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_profile_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_network_profile::builders::DeleteNetworkProfileFluentBuilder::network_profile_arn) / [`set_network_profile_arn(Option<String>)`](crate::operation::delete_network_profile::builders::DeleteNetworkProfileFluentBuilder::set_network_profile_arn): <p>The ARN of the network profile associated with a device.</p>
    /// - On success, responds with [`DeleteNetworkProfileOutput`](crate::operation::delete_network_profile::DeleteNetworkProfileOutput)
    /// - On failure, responds with [`SdkError<DeleteNetworkProfileError>`](crate::operation::delete_network_profile::DeleteNetworkProfileError)
    #[deprecated(note = "Alexa For Business is no longer supported")]
    pub fn delete_network_profile(&self) -> crate::operation::delete_network_profile::builders::DeleteNetworkProfileFluentBuilder {
        crate::operation::delete_network_profile::builders::DeleteNetworkProfileFluentBuilder::new(self.handle.clone())
    }
}
