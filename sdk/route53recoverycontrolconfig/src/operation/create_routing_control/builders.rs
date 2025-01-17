// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_routing_control::_create_routing_control_output::CreateRoutingControlOutputBuilder;

pub use crate::operation::create_routing_control::_create_routing_control_input::CreateRoutingControlInputBuilder;

impl CreateRoutingControlInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_routing_control::CreateRoutingControlOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_routing_control::CreateRoutingControlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_routing_control();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRoutingControl`.
///
/// <p>Creates a new routing control.</p>
/// <p>A routing control has one of two states: ON and OFF. You can map the routing control state to the state of an Amazon Route 53 health check, which can be used to control traffic routing.</p>
/// <p>To get or update the routing control state, see the Recovery Cluster (data plane) API actions for Amazon Route 53 Application Recovery Controller.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRoutingControlFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_routing_control::builders::CreateRoutingControlInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateRoutingControlFluentBuilder {
    /// Creates a new `CreateRoutingControl`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRoutingControl as a reference.
    pub fn as_input(&self) -> &crate::operation::create_routing_control::builders::CreateRoutingControlInputBuilder {
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
        crate::operation::create_routing_control::CreateRoutingControlOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_routing_control::CreateRoutingControlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_routing_control::CreateRoutingControl::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_routing_control::CreateRoutingControl::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_routing_control::CreateRoutingControlOutput,
            crate::operation::create_routing_control::CreateRoutingControlError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_routing_control::CreateRoutingControlError>,
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
    /// <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive string of up to 64 ASCII characters. To make an idempotent API request with an action, specify a client token in the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster that includes the routing control.</p>
    pub fn cluster_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster that includes the routing control.</p>
    pub fn set_cluster_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster that includes the routing control.</p>
    pub fn get_cluster_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of the control panel that includes the routing control.</p>
    pub fn control_panel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.control_panel_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the control panel that includes the routing control.</p>
    pub fn set_control_panel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_control_panel_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the control panel that includes the routing control.</p>
    pub fn get_control_panel_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_control_panel_arn()
    }
    /// <p>The name of the routing control.</p>
    pub fn routing_control_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.routing_control_name(input.into());
        self
    }
    /// <p>The name of the routing control.</p>
    pub fn set_routing_control_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_routing_control_name(input);
        self
    }
    /// <p>The name of the routing control.</p>
    pub fn get_routing_control_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_routing_control_name()
    }
}
