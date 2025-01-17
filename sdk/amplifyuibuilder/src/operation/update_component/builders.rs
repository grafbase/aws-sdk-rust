// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_component::_update_component_output::UpdateComponentOutputBuilder;

pub use crate::operation::update_component::_update_component_input::UpdateComponentInputBuilder;

impl UpdateComponentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_component::UpdateComponentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_component::UpdateComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_component();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateComponent`.
///
/// <p>Updates an existing component.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateComponentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_component::builders::UpdateComponentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateComponentFluentBuilder {
    /// Creates a new `UpdateComponent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateComponent as a reference.
    pub fn as_input(&self) -> &crate::operation::update_component::builders::UpdateComponentInputBuilder {
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
        crate::operation::update_component::UpdateComponentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_component::UpdateComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_component::UpdateComponent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_component::UpdateComponent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_component::UpdateComponentOutput,
            crate::operation::update_component::UpdateComponentError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_component::UpdateComponentError>,
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
    /// <p>The unique ID for the Amplify app.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The unique ID for the Amplify app.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The unique ID for the Amplify app.</p>
    pub fn get_app_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_id()
    }
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    pub fn environment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_name(input.into());
        self
    }
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    pub fn set_environment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_name(input);
        self
    }
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    pub fn get_environment_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_name()
    }
    /// <p>The unique ID for the component.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique ID for the component.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The unique ID for the component.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The unique client token.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The unique client token.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The unique client token.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The configuration of the updated component.</p>
    pub fn updated_component(mut self, input: crate::types::UpdateComponentData) -> Self {
        self.inner = self.inner.updated_component(input);
        self
    }
    /// <p>The configuration of the updated component.</p>
    pub fn set_updated_component(mut self, input: ::std::option::Option<crate::types::UpdateComponentData>) -> Self {
        self.inner = self.inner.set_updated_component(input);
        self
    }
    /// <p>The configuration of the updated component.</p>
    pub fn get_updated_component(&self) -> &::std::option::Option<crate::types::UpdateComponentData> {
        self.inner.get_updated_component()
    }
}
