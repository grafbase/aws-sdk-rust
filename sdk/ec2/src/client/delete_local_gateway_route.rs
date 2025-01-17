// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLocalGatewayRoute`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`destination_cidr_block(impl ::std::convert::Into<String>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::destination_cidr_block) / [`set_destination_cidr_block(Option<String>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::set_destination_cidr_block): <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    ///   - [`local_gateway_route_table_id(impl ::std::convert::Into<String>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::local_gateway_route_table_id) / [`set_local_gateway_route_table_id(Option<String>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::set_local_gateway_route_table_id): <p>The ID of the local gateway route table.</p>
    ///   - [`dry_run(bool)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`destination_prefix_list_id(impl ::std::convert::Into<String>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::destination_prefix_list_id) / [`set_destination_prefix_list_id(Option<String>)`](crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::set_destination_prefix_list_id): <p> Use a prefix list in place of <code>DestinationCidrBlock</code>. You cannot use <code>DestinationPrefixListId</code> and <code>DestinationCidrBlock</code> in the same request. </p>
    /// - On success, responds with [`DeleteLocalGatewayRouteOutput`](crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteOutput) with field(s):
    ///   - [`route(Option<LocalGatewayRoute>)`](crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteOutput::route): <p>Information about the route.</p>
    /// - On failure, responds with [`SdkError<DeleteLocalGatewayRouteError>`](crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteError)
    pub fn delete_local_gateway_route(&self) -> crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder {
        crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteFluentBuilder::new(self.handle.clone())
    }
}
