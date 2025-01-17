// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpc_endpoint_connection_notification::_create_vpc_endpoint_connection_notification_output::CreateVpcEndpointConnectionNotificationOutputBuilder;

pub use crate::operation::create_vpc_endpoint_connection_notification::_create_vpc_endpoint_connection_notification_input::CreateVpcEndpointConnectionNotificationInputBuilder;

impl CreateVpcEndpointConnectionNotificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_vpc_endpoint_connection_notification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateVpcEndpointConnectionNotification`.
///
/// <p>Creates a connection notification for a specified VPC endpoint or VPC endpoint service. A connection notification notifies you of specific endpoint events. You must create an SNS topic to receive notifications. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Create a Topic</a> in the <i>Amazon Simple Notification Service Developer Guide</i>.</p>
/// <p>You can create a connection notification for interface endpoints only.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateVpcEndpointConnectionNotificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateVpcEndpointConnectionNotificationFluentBuilder {
    /// Creates a new `CreateVpcEndpointConnectionNotification`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateVpcEndpointConnectionNotification as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder {
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
        crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotification::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotification::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationOutput,
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationError,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationError,
        >,
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
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn service_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_id(input.into());
        self
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn set_service_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_id(input);
        self
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn get_service_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_id()
    }
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn set_vpc_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_endpoint_id(input);
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn get_vpc_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_endpoint_id()
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn connection_notification_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_notification_arn(input.into());
        self
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn set_connection_notification_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_notification_arn(input);
        self
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn get_connection_notification_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_notification_arn()
    }
    /// Appends an item to `ConnectionEvents`.
    ///
    /// To override the contents of this collection use [`set_connection_events`](Self::set_connection_events).
    ///
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn connection_events(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_events(input.into());
        self
    }
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn set_connection_events(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_connection_events(input);
        self
    }
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn get_connection_events(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_connection_events()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
