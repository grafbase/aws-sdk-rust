// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_public_ipv4_pools::_describe_public_ipv4_pools_output::DescribePublicIpv4PoolsOutputBuilder;

pub use crate::operation::describe_public_ipv4_pools::_describe_public_ipv4_pools_input::DescribePublicIpv4PoolsInputBuilder;

impl DescribePublicIpv4PoolsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_public_ipv4_pools();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribePublicIpv4Pools`.
///
/// <p>Describes the specified IPv4 address pools.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribePublicIpv4PoolsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribePublicIpv4PoolsFluentBuilder {
    /// Creates a new `DescribePublicIpv4Pools`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribePublicIpv4Pools as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsInputBuilder {
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
        crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_public_ipv4_pools::DescribePublicIpv4Pools::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_public_ipv4_pools::DescribePublicIpv4Pools::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput,
            crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_public_ipv4_pools::paginator::DescribePublicIpv4PoolsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_public_ipv4_pools::paginator::DescribePublicIpv4PoolsPaginator {
        crate::operation::describe_public_ipv4_pools::paginator::DescribePublicIpv4PoolsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `PoolIds`.
    ///
    /// To override the contents of this collection use [`set_pool_ids`](Self::set_pool_ids).
    ///
    /// <p>The IDs of the address pools.</p>
    pub fn pool_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pool_ids(input.into());
        self
    }
    /// <p>The IDs of the address pools.</p>
    pub fn set_pool_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_pool_ids(input);
        self
    }
    /// <p>The IDs of the address pools.</p>
    pub fn get_pool_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_pool_ids()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
}
