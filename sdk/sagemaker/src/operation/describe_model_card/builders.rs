// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_model_card::_describe_model_card_output::DescribeModelCardOutputBuilder;

pub use crate::operation::describe_model_card::_describe_model_card_input::DescribeModelCardInputBuilder;

impl DescribeModelCardInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_model_card::DescribeModelCardOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_model_card::DescribeModelCardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_model_card();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeModelCard`.
///
/// <p>Describes the content, creation time, and security configuration of an Amazon SageMaker Model Card.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeModelCardFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_model_card::builders::DescribeModelCardInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeModelCardFluentBuilder {
    /// Creates a new `DescribeModelCard`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeModelCard as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_model_card::builders::DescribeModelCardInputBuilder {
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
        crate::operation::describe_model_card::DescribeModelCardOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_model_card::DescribeModelCardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_model_card::DescribeModelCard::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_model_card::DescribeModelCard::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_model_card::DescribeModelCardOutput,
            crate::operation::describe_model_card::DescribeModelCardError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_model_card::DescribeModelCardError>,
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
    /// <p>The name of the model card to describe.</p>
    pub fn model_card_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_card_name(input.into());
        self
    }
    /// <p>The name of the model card to describe.</p>
    pub fn set_model_card_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_card_name(input);
        self
    }
    /// <p>The name of the model card to describe.</p>
    pub fn get_model_card_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_card_name()
    }
    /// <p>The version of the model card to describe. If a version is not provided, then the latest version of the model card is described.</p>
    pub fn model_card_version(mut self, input: i32) -> Self {
        self.inner = self.inner.model_card_version(input);
        self
    }
    /// <p>The version of the model card to describe. If a version is not provided, then the latest version of the model card is described.</p>
    pub fn set_model_card_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_model_card_version(input);
        self
    }
    /// <p>The version of the model card to describe. If a version is not provided, then the latest version of the model card is described.</p>
    pub fn get_model_card_version(&self) -> &::std::option::Option<i32> {
        self.inner.get_model_card_version()
    }
}
