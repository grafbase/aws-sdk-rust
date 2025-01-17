// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_configuration_recorder::_start_configuration_recorder_output::StartConfigurationRecorderOutputBuilder;

pub use crate::operation::start_configuration_recorder::_start_configuration_recorder_input::StartConfigurationRecorderInputBuilder;

impl StartConfigurationRecorderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_configuration_recorder::StartConfigurationRecorderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_configuration_recorder::StartConfigurationRecorderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_configuration_recorder();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartConfigurationRecorder`.
///
/// <p>Starts recording configurations of the Amazon Web Services resources you have selected to record in your Amazon Web Services account.</p>
/// <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartConfigurationRecorderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl StartConfigurationRecorderFluentBuilder {
    /// Creates a new `StartConfigurationRecorder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartConfigurationRecorder as a reference.
    pub fn as_input(&self) -> &crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderInputBuilder {
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
        crate::operation::start_configuration_recorder::StartConfigurationRecorderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_configuration_recorder::StartConfigurationRecorderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_configuration_recorder::StartConfigurationRecorder::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_configuration_recorder::StartConfigurationRecorder::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::start_configuration_recorder::StartConfigurationRecorderOutput,
            crate::operation::start_configuration_recorder::StartConfigurationRecorderError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_configuration_recorder::StartConfigurationRecorderError>,
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
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    pub fn configuration_recorder_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configuration_recorder_name(input.into());
        self
    }
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    pub fn set_configuration_recorder_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configuration_recorder_name(input);
        self
    }
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    pub fn get_configuration_recorder_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configuration_recorder_name()
    }
}
