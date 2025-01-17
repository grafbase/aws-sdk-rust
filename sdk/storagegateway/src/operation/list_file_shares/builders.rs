// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_file_shares::_list_file_shares_output::ListFileSharesOutputBuilder;

pub use crate::operation::list_file_shares::_list_file_shares_input::ListFileSharesInputBuilder;

impl ListFileSharesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_file_shares::ListFileSharesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_file_shares::ListFileSharesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_file_shares();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListFileShares`.
///
/// <p>Gets a list of the file shares for a specific S3 File Gateway, or the list of file shares that belong to the calling user account. This operation is only supported for S3 File Gateways.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListFileSharesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_file_shares::builders::ListFileSharesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListFileSharesFluentBuilder {
    /// Creates a new `ListFileShares`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListFileShares as a reference.
    pub fn as_input(&self) -> &crate::operation::list_file_shares::builders::ListFileSharesInputBuilder {
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
        crate::operation::list_file_shares::ListFileSharesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_file_shares::ListFileSharesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_file_shares::ListFileShares::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_file_shares::ListFileShares::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_file_shares::ListFileSharesOutput,
            crate::operation::list_file_shares::ListFileSharesError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_file_shares::ListFileSharesError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_file_shares::paginator::ListFileSharesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_file_shares::paginator::ListFileSharesPaginator {
        crate::operation::list_file_shares::paginator::ListFileSharesPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway whose file shares you want to list. If this field is not present, all file shares under your account are listed.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway whose file shares you want to list. If this field is not present, all file shares under your account are listed.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway whose file shares you want to list. If this field is not present, all file shares under your account are listed.</p>
    pub fn get_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_arn()
    }
    /// <p>The maximum number of file shares to return in the response. The value must be an integer with a value greater than zero. Optional.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of file shares to return in the response. The value must be an integer with a value greater than zero. Optional.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of file shares to return in the response. The value must be an integer with a value greater than zero. Optional.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>Opaque pagination token returned from a previous ListFileShares operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to ListFileShares. Optional.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Opaque pagination token returned from a previous ListFileShares operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to ListFileShares. Optional.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Opaque pagination token returned from a previous ListFileShares operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to ListFileShares. Optional.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
