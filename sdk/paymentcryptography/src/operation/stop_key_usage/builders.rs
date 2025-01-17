// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_key_usage::_stop_key_usage_output::StopKeyUsageOutputBuilder;

pub use crate::operation::stop_key_usage::_stop_key_usage_input::StopKeyUsageInputBuilder;

impl StopKeyUsageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_key_usage::StopKeyUsageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_key_usage::StopKeyUsageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_key_usage();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopKeyUsage`.
///
/// <p>Disables an Amazon Web Services Payment Cryptography key, which makes it inactive within Amazon Web Services Payment Cryptography.</p>
/// <p>You can use this operation instead of <code>DeleteKey</code> to deactivate a key. You can enable the key in the future by calling <code>StartKeyUsage</code>.</p>
/// <p> <b>Cross-account use:</b> This operation can't be used across different Amazon Web Services accounts.</p>
/// <p> <b>Related operations:</b> </p>
/// <ul>
/// <li> <p> <code>DeleteKey</code> </p> </li>
/// <li> <p> <code>StartKeyUsage</code> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopKeyUsageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_key_usage::builders::StopKeyUsageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl StopKeyUsageFluentBuilder {
    /// Creates a new `StopKeyUsage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopKeyUsage as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_key_usage::builders::StopKeyUsageInputBuilder {
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
        crate::operation::stop_key_usage::StopKeyUsageOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_key_usage::StopKeyUsageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_key_usage::StopKeyUsage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_key_usage::StopKeyUsage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::stop_key_usage::StopKeyUsageOutput,
            crate::operation::stop_key_usage::StopKeyUsageError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::stop_key_usage::StopKeyUsageError>,
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
    /// <p>The <code>KeyArn</code> of the key.</p>
    pub fn key_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_identifier(input.into());
        self
    }
    /// <p>The <code>KeyArn</code> of the key.</p>
    pub fn set_key_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_identifier(input);
        self
    }
    /// <p>The <code>KeyArn</code> of the key.</p>
    pub fn get_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_identifier()
    }
}
