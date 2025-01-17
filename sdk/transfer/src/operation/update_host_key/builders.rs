// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_host_key::_update_host_key_output::UpdateHostKeyOutputBuilder;

pub use crate::operation::update_host_key::_update_host_key_input::UpdateHostKeyInputBuilder;

impl UpdateHostKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_host_key::UpdateHostKeyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_host_key::UpdateHostKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_host_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateHostKey`.
///
/// <p>Updates the description for the host key that's specified by the <code>ServerId</code> and <code>HostKeyId</code> parameters.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateHostKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_host_key::builders::UpdateHostKeyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateHostKeyFluentBuilder {
    /// Creates a new `UpdateHostKey`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateHostKey as a reference.
    pub fn as_input(&self) -> &crate::operation::update_host_key::builders::UpdateHostKeyInputBuilder {
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
        crate::operation::update_host_key::UpdateHostKeyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_host_key::UpdateHostKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_host_key::UpdateHostKey::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_host_key::UpdateHostKey::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_host_key::UpdateHostKeyOutput,
            crate::operation::update_host_key::UpdateHostKeyError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_host_key::UpdateHostKeyError>,
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
    /// <p>The identifier of the server that contains the host key that you are updating.</p>
    pub fn server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.server_id(input.into());
        self
    }
    /// <p>The identifier of the server that contains the host key that you are updating.</p>
    pub fn set_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_server_id(input);
        self
    }
    /// <p>The identifier of the server that contains the host key that you are updating.</p>
    pub fn get_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_server_id()
    }
    /// <p>The identifier of the host key that you are updating.</p>
    pub fn host_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.host_key_id(input.into());
        self
    }
    /// <p>The identifier of the host key that you are updating.</p>
    pub fn set_host_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_host_key_id(input);
        self
    }
    /// <p>The identifier of the host key that you are updating.</p>
    pub fn get_host_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_host_key_id()
    }
    /// <p>An updated description for the host key.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An updated description for the host key.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An updated description for the host key.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
}
