// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_aliases::_list_aliases_output::ListAliasesOutputBuilder;

pub use crate::operation::list_aliases::_list_aliases_input::ListAliasesInputBuilder;

impl ListAliasesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_aliases::ListAliasesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_aliases::ListAliasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_aliases();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAliases`.
///
/// <p>Creates a paginated call to list the aliases associated with a given entity.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAliasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_aliases::builders::ListAliasesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListAliasesFluentBuilder {
    /// Creates a new `ListAliases`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAliases as a reference.
    pub fn as_input(&self) -> &crate::operation::list_aliases::builders::ListAliasesInputBuilder {
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
        crate::operation::list_aliases::ListAliasesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_aliases::ListAliasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_aliases::ListAliases::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_aliases::ListAliases::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_aliases::ListAliasesOutput,
            crate::operation::list_aliases::ListAliasesError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_aliases::ListAliasesError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_aliases::paginator::ListAliasesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_aliases::paginator::ListAliasesPaginator {
        crate::operation::list_aliases::paginator::ListAliasesPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier for the organization under which the entity exists.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The identifier for the organization under which the entity exists.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The identifier for the organization under which the entity exists.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_id()
    }
    /// <p>The identifier for the entity for which to list the aliases.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_id(input.into());
        self
    }
    /// <p>The identifier for the entity for which to list the aliases.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_id(input);
        self
    }
    /// <p>The identifier for the entity for which to list the aliases.</p>
    pub fn get_entity_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_entity_id()
    }
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
