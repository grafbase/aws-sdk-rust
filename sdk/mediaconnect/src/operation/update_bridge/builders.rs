// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bridge::_update_bridge_output::UpdateBridgeOutputBuilder;

pub use crate::operation::update_bridge::_update_bridge_input::UpdateBridgeInputBuilder;

impl UpdateBridgeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_bridge::UpdateBridgeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bridge::UpdateBridgeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_bridge();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBridge`.
///
/// Updates the bridge
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBridgeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bridge::builders::UpdateBridgeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateBridgeFluentBuilder {
    /// Creates a new `UpdateBridge`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBridge as a reference.
    pub fn as_input(&self) -> &crate::operation::update_bridge::builders::UpdateBridgeInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_bridge::UpdateBridgeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_bridge::UpdateBridgeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_bridge::UpdateBridge::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_bridge::UpdateBridge::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_bridge::UpdateBridgeOutput,
            crate::operation::update_bridge::UpdateBridgeError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_bridge::UpdateBridgeError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// The Amazon Resource Number (ARN) of the bridge that you want to update.
    pub fn bridge_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bridge_arn(input.into());
        self
    }
    /// The Amazon Resource Number (ARN) of the bridge that you want to update.
    pub fn set_bridge_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bridge_arn(input);
        self
    }
    /// The Amazon Resource Number (ARN) of the bridge that you want to update.
    pub fn get_bridge_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bridge_arn()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn egress_gateway_bridge(mut self, input: crate::types::UpdateEgressGatewayBridgeRequest) -> Self {
        self.inner = self.inner.egress_gateway_bridge(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_egress_gateway_bridge(mut self, input: ::std::option::Option<crate::types::UpdateEgressGatewayBridgeRequest>) -> Self {
        self.inner = self.inner.set_egress_gateway_bridge(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_egress_gateway_bridge(&self) -> &::std::option::Option<crate::types::UpdateEgressGatewayBridgeRequest> {
        self.inner.get_egress_gateway_bridge()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn ingress_gateway_bridge(mut self, input: crate::types::UpdateIngressGatewayBridgeRequest) -> Self {
        self.inner = self.inner.ingress_gateway_bridge(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_ingress_gateway_bridge(mut self, input: ::std::option::Option<crate::types::UpdateIngressGatewayBridgeRequest>) -> Self {
        self.inner = self.inner.set_ingress_gateway_bridge(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_ingress_gateway_bridge(&self) -> &::std::option::Option<crate::types::UpdateIngressGatewayBridgeRequest> {
        self.inner.get_ingress_gateway_bridge()
    }
    /// The settings for source failover.
    pub fn source_failover_config(mut self, input: crate::types::UpdateFailoverConfig) -> Self {
        self.inner = self.inner.source_failover_config(input);
        self
    }
    /// The settings for source failover.
    pub fn set_source_failover_config(mut self, input: ::std::option::Option<crate::types::UpdateFailoverConfig>) -> Self {
        self.inner = self.inner.set_source_failover_config(input);
        self
    }
    /// The settings for source failover.
    pub fn get_source_failover_config(&self) -> &::std::option::Option<crate::types::UpdateFailoverConfig> {
        self.inner.get_source_failover_config()
    }
}
