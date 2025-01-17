// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_template_step::_get_template_step_output::GetTemplateStepOutputBuilder;

pub use crate::operation::get_template_step::_get_template_step_input::GetTemplateStepInputBuilder;

impl GetTemplateStepInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_template_step::GetTemplateStepOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_template_step::GetTemplateStepError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_template_step();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTemplateStep`.
///
/// <p>Get a specific step in a template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTemplateStepFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_template_step::builders::GetTemplateStepInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetTemplateStepFluentBuilder {
    /// Creates a new `GetTemplateStep`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetTemplateStep as a reference.
    pub fn as_input(&self) -> &crate::operation::get_template_step::builders::GetTemplateStepInputBuilder {
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
        crate::operation::get_template_step::GetTemplateStepOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_template_step::GetTemplateStepError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_template_step::GetTemplateStep::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_template_step::GetTemplateStep::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_template_step::GetTemplateStepOutput,
            crate::operation::get_template_step::GetTemplateStepError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_template_step::GetTemplateStepError>,
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
    /// <p>The ID of the step.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the step.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the step.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The ID of the template.</p>
    pub fn template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_id(input.into());
        self
    }
    /// <p>The ID of the template.</p>
    pub fn set_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_id(input);
        self
    }
    /// <p>The ID of the template.</p>
    pub fn get_template_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_id()
    }
    /// <p>The ID of the step group.</p>
    pub fn step_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.step_group_id(input.into());
        self
    }
    /// <p>The ID of the step group.</p>
    pub fn set_step_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_step_group_id(input);
        self
    }
    /// <p>The ID of the step group.</p>
    pub fn get_step_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_step_group_id()
    }
}
