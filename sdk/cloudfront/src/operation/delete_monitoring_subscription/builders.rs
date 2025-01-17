// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_monitoring_subscription::_delete_monitoring_subscription_output::DeleteMonitoringSubscriptionOutputBuilder;

pub use crate::operation::delete_monitoring_subscription::_delete_monitoring_subscription_input::DeleteMonitoringSubscriptionInputBuilder;

impl DeleteMonitoringSubscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_monitoring_subscription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteMonitoringSubscription`.
///
/// <p>Disables additional CloudWatch metrics for the specified CloudFront distribution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteMonitoringSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_monitoring_subscription::builders::DeleteMonitoringSubscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteMonitoringSubscriptionFluentBuilder {
    /// Creates a new `DeleteMonitoringSubscription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteMonitoringSubscription as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_monitoring_subscription::builders::DeleteMonitoringSubscriptionInputBuilder {
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
        crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscription::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionOutput,
            crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_monitoring_subscription::DeleteMonitoringSubscriptionError>,
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
    /// <p>The ID of the distribution that you are disabling metrics for.</p>
    pub fn distribution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.distribution_id(input.into());
        self
    }
    /// <p>The ID of the distribution that you are disabling metrics for.</p>
    pub fn set_distribution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_distribution_id(input);
        self
    }
    /// <p>The ID of the distribution that you are disabling metrics for.</p>
    pub fn get_distribution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_distribution_id()
    }
}
