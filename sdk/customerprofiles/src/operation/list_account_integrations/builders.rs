// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_account_integrations::_list_account_integrations_output::ListAccountIntegrationsOutputBuilder;

pub use crate::operation::list_account_integrations::_list_account_integrations_input::ListAccountIntegrationsInputBuilder;

impl ListAccountIntegrationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_account_integrations::ListAccountIntegrationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_account_integrations::ListAccountIntegrationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_account_integrations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAccountIntegrations`.
///
/// <p>Lists all of the integrations associated to a specific URI in the AWS account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAccountIntegrationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_account_integrations::builders::ListAccountIntegrationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListAccountIntegrationsFluentBuilder {
    /// Creates a new `ListAccountIntegrations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAccountIntegrations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_account_integrations::builders::ListAccountIntegrationsInputBuilder {
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
        crate::operation::list_account_integrations::ListAccountIntegrationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_account_integrations::ListAccountIntegrationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_account_integrations::ListAccountIntegrations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_account_integrations::ListAccountIntegrations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_account_integrations::ListAccountIntegrationsOutput,
            crate::operation::list_account_integrations::ListAccountIntegrationsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_account_integrations::ListAccountIntegrationsError>,
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
    /// <p>The URI of the S3 bucket or any other type of data source.</p>
    pub fn uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.uri(input.into());
        self
    }
    /// <p>The URI of the S3 bucket or any other type of data source.</p>
    pub fn set_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_uri(input);
        self
    }
    /// <p>The URI of the S3 bucket or any other type of data source.</p>
    pub fn get_uri(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_uri()
    }
    /// <p>The pagination token from the previous ListAccountIntegrations API call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token from the previous ListAccountIntegrations API call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token from the previous ListAccountIntegrations API call.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of objects returned per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of objects returned per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of objects returned per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Boolean to indicate if hidden integration should be returned. Defaults to <code>False</code>.</p>
    pub fn include_hidden(mut self, input: bool) -> Self {
        self.inner = self.inner.include_hidden(input);
        self
    }
    /// <p>Boolean to indicate if hidden integration should be returned. Defaults to <code>False</code>.</p>
    pub fn set_include_hidden(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_hidden(input);
        self
    }
    /// <p>Boolean to indicate if hidden integration should be returned. Defaults to <code>False</code>.</p>
    pub fn get_include_hidden(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_hidden()
    }
}
