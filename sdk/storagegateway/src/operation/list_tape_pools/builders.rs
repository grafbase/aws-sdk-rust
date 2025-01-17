// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tape_pools::_list_tape_pools_output::ListTapePoolsOutputBuilder;

pub use crate::operation::list_tape_pools::_list_tape_pools_input::ListTapePoolsInputBuilder;

impl ListTapePoolsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_tape_pools::ListTapePoolsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tape_pools::ListTapePoolsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_tape_pools();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTapePools`.
///
/// <p>Lists custom tape pools. You specify custom tape pools to list by specifying one or more custom tape pool Amazon Resource Names (ARNs). If you don't specify a custom tape pool ARN, the operation lists all custom tape pools.</p>
/// <p>This operation supports pagination. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tape pools in the response. If the number of tape pools returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tape pools.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTapePoolsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tape_pools::builders::ListTapePoolsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListTapePoolsFluentBuilder {
    /// Creates a new `ListTapePools`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTapePools as a reference.
    pub fn as_input(&self) -> &crate::operation::list_tape_pools::builders::ListTapePoolsInputBuilder {
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
        crate::operation::list_tape_pools::ListTapePoolsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tape_pools::ListTapePoolsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_tape_pools::ListTapePools::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_tape_pools::ListTapePools::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_tape_pools::ListTapePoolsOutput,
            crate::operation::list_tape_pools::ListTapePoolsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_tape_pools::ListTapePoolsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_tape_pools::paginator::ListTapePoolsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_tape_pools::paginator::ListTapePoolsPaginator {
        crate::operation::list_tape_pools::paginator::ListTapePoolsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `PoolARNs`.
    ///
    /// To override the contents of this collection use [`set_pool_ar_ns`](Self::set_pool_ar_ns).
    ///
    /// <p>The Amazon Resource Name (ARN) of each of the custom tape pools you want to list. If you don't specify a custom tape pool ARN, the response lists all custom tape pools. </p>
    pub fn pool_ar_ns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pool_ar_ns(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of each of the custom tape pools you want to list. If you don't specify a custom tape pool ARN, the response lists all custom tape pools. </p>
    pub fn set_pool_ar_ns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_pool_ar_ns(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of each of the custom tape pools you want to list. If you don't specify a custom tape pool ARN, the response lists all custom tape pools. </p>
    pub fn get_pool_ar_ns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_pool_ar_ns()
    }
    /// <p>A string that indicates the position at which to begin the returned list of tape pools.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>A string that indicates the position at which to begin the returned list of tape pools.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>A string that indicates the position at which to begin the returned list of tape pools.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>An optional number limit for the tape pools in the list returned by this call.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>An optional number limit for the tape pools in the list returned by this call.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>An optional number limit for the tape pools in the list returned by this call.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
}
