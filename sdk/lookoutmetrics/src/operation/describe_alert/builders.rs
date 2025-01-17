// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_alert::_describe_alert_output::DescribeAlertOutputBuilder;

pub use crate::operation::describe_alert::_describe_alert_input::DescribeAlertInputBuilder;

impl DescribeAlertInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_alert::DescribeAlertOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_alert::DescribeAlertError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_alert();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAlert`.
///
/// <p>Describes an alert.</p>
/// <p>Amazon Lookout for Metrics API actions are eventually consistent. If you do a read operation on a resource immediately after creating or modifying it, use retries to allow time for the write operation to complete.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAlertFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_alert::builders::DescribeAlertInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeAlertFluentBuilder {
    /// Creates a new `DescribeAlert`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAlert as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_alert::builders::DescribeAlertInputBuilder {
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
        crate::operation::describe_alert::DescribeAlertOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_alert::DescribeAlertError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_alert::DescribeAlert::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_alert::DescribeAlert::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_alert::DescribeAlertOutput,
            crate::operation::describe_alert::DescribeAlertError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_alert::DescribeAlertError>,
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
    /// <p>The ARN of the alert to describe.</p>
    pub fn alert_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alert_arn(input.into());
        self
    }
    /// <p>The ARN of the alert to describe.</p>
    pub fn set_alert_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alert_arn(input);
        self
    }
    /// <p>The ARN of the alert to describe.</p>
    pub fn get_alert_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alert_arn()
    }
}
