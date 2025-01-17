// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDirectConnectGateway`](crate::operation::delete_direct_connect_gateway::builders::DeleteDirectConnectGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`direct_connect_gateway_id(impl ::std::convert::Into<String>)`](crate::operation::delete_direct_connect_gateway::builders::DeleteDirectConnectGatewayFluentBuilder::direct_connect_gateway_id) / [`set_direct_connect_gateway_id(Option<String>)`](crate::operation::delete_direct_connect_gateway::builders::DeleteDirectConnectGatewayFluentBuilder::set_direct_connect_gateway_id): <p>The ID of the Direct Connect gateway.</p>
    /// - On success, responds with [`DeleteDirectConnectGatewayOutput`](crate::operation::delete_direct_connect_gateway::DeleteDirectConnectGatewayOutput) with field(s):
    ///   - [`direct_connect_gateway(Option<DirectConnectGateway>)`](crate::operation::delete_direct_connect_gateway::DeleteDirectConnectGatewayOutput::direct_connect_gateway): <p>The Direct Connect gateway.</p>
    /// - On failure, responds with [`SdkError<DeleteDirectConnectGatewayError>`](crate::operation::delete_direct_connect_gateway::DeleteDirectConnectGatewayError)
    pub fn delete_direct_connect_gateway(
        &self,
    ) -> crate::operation::delete_direct_connect_gateway::builders::DeleteDirectConnectGatewayFluentBuilder {
        crate::operation::delete_direct_connect_gateway::builders::DeleteDirectConnectGatewayFluentBuilder::new(self.handle.clone())
    }
}
