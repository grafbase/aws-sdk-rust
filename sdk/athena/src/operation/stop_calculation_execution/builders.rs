// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_calculation_execution::_stop_calculation_execution_output::StopCalculationExecutionOutputBuilder;

pub use crate::operation::stop_calculation_execution::_stop_calculation_execution_input::StopCalculationExecutionInputBuilder;

impl StopCalculationExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_calculation_execution::StopCalculationExecutionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_calculation_execution::StopCalculationExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_calculation_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopCalculationExecution`.
///
/// <p>Requests the cancellation of a calculation. A <code>StopCalculationExecution</code> call on a calculation that is already in a terminal state (for example, <code>STOPPED</code>, <code>FAILED</code>, or <code>COMPLETED</code>) succeeds but has no effect.</p> <note>
/// <p>Cancelling a calculation is done on a best effort basis. If a calculation cannot be cancelled, you can be charged for its completion. If you are concerned about being charged for a calculation that cannot be cancelled, consider terminating the session in which the calculation is running.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopCalculationExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_calculation_execution::builders::StopCalculationExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl StopCalculationExecutionFluentBuilder {
    /// Creates a new `StopCalculationExecution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopCalculationExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_calculation_execution::builders::StopCalculationExecutionInputBuilder {
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
        crate::operation::stop_calculation_execution::StopCalculationExecutionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_calculation_execution::StopCalculationExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_calculation_execution::StopCalculationExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_calculation_execution::StopCalculationExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::stop_calculation_execution::StopCalculationExecutionOutput,
            crate::operation::stop_calculation_execution::StopCalculationExecutionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::stop_calculation_execution::StopCalculationExecutionError>,
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
    /// <p>The calculation execution UUID.</p>
    pub fn calculation_execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.calculation_execution_id(input.into());
        self
    }
    /// <p>The calculation execution UUID.</p>
    pub fn set_calculation_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_calculation_execution_id(input);
        self
    }
    /// <p>The calculation execution UUID.</p>
    pub fn get_calculation_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_calculation_execution_id()
    }
}
