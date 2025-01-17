// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCustomRoutingListener`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`accelerator_arn(impl ::std::convert::Into<String>)`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::accelerator_arn) / [`set_accelerator_arn(Option<String>)`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::set_accelerator_arn): <p>The Amazon Resource Name (ARN) of the accelerator for a custom routing listener.</p>
    ///   - [`port_ranges(Vec<PortRange>)`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::port_ranges) / [`set_port_ranges(Option<Vec<PortRange>>)`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::set_port_ranges): <p>The port range to support for connections from clients to your accelerator.</p>  <p>Separately, you set port ranges for endpoints. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-custom-routing-endpoints.html">About endpoints for custom routing accelerators</a>.</p>
    ///   - [`idempotency_token(impl ::std::convert::Into<String>)`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::set_idempotency_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    /// - On success, responds with [`CreateCustomRoutingListenerOutput`](crate::operation::create_custom_routing_listener::CreateCustomRoutingListenerOutput) with field(s):
    ///   - [`listener(Option<CustomRoutingListener>)`](crate::operation::create_custom_routing_listener::CreateCustomRoutingListenerOutput::listener): <p>The listener that you've created for a custom routing accelerator.</p>
    /// - On failure, responds with [`SdkError<CreateCustomRoutingListenerError>`](crate::operation::create_custom_routing_listener::CreateCustomRoutingListenerError)
    pub fn create_custom_routing_listener(
        &self,
    ) -> crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder {
        crate::operation::create_custom_routing_listener::builders::CreateCustomRoutingListenerFluentBuilder::new(self.handle.clone())
    }
}
