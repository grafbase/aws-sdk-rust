// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_asset_model_properties::_list_asset_model_properties_output::ListAssetModelPropertiesOutputBuilder;

pub use crate::operation::list_asset_model_properties::_list_asset_model_properties_input::ListAssetModelPropertiesInputBuilder;

impl ListAssetModelPropertiesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_asset_model_properties::ListAssetModelPropertiesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_asset_model_properties::ListAssetModelPropertiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_asset_model_properties();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAssetModelProperties`.
///
/// <p>Retrieves a paginated list of properties associated with an asset model. If you update properties associated with the model before you finish listing all the properties, you need to start all over again.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAssetModelPropertiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_asset_model_properties::builders::ListAssetModelPropertiesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListAssetModelPropertiesFluentBuilder {
    /// Creates a new `ListAssetModelProperties`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAssetModelProperties as a reference.
    pub fn as_input(&self) -> &crate::operation::list_asset_model_properties::builders::ListAssetModelPropertiesInputBuilder {
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
        crate::operation::list_asset_model_properties::ListAssetModelPropertiesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_asset_model_properties::ListAssetModelPropertiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_asset_model_properties::ListAssetModelProperties::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_asset_model_properties::ListAssetModelProperties::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_asset_model_properties::ListAssetModelPropertiesOutput,
            crate::operation::list_asset_model_properties::ListAssetModelPropertiesError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_asset_model_properties::ListAssetModelPropertiesError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_asset_model_properties::paginator::ListAssetModelPropertiesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_asset_model_properties::paginator::ListAssetModelPropertiesPaginator {
        crate::operation::list_asset_model_properties::paginator::ListAssetModelPropertiesPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the asset model.</p>
    pub fn asset_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset_model_id(input.into());
        self
    }
    /// <p>The ID of the asset model.</p>
    pub fn set_asset_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asset_model_id(input);
        self
    }
    /// <p>The ID of the asset model.</p>
    pub fn get_asset_model_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asset_model_id()
    }
    /// <p>The token to be used for the next set of paginated results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to be used for the next set of paginated results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return for each paginated request. If not specified, the default value is 50.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return for each paginated request. If not specified, the default value is 50.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return for each paginated request. If not specified, the default value is 50.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p> Filters the requested list of asset model properties. You can choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ALL</code> – The list includes all asset model properties for a given asset model ID. </p> </li>
    /// <li> <p> <code>BASE</code> – The list includes only base asset model properties for a given asset model ID. </p> </li>
    /// </ul>
    /// <p>Default: <code>BASE</code> </p>
    pub fn filter(mut self, input: crate::types::ListAssetModelPropertiesFilter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p> Filters the requested list of asset model properties. You can choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ALL</code> – The list includes all asset model properties for a given asset model ID. </p> </li>
    /// <li> <p> <code>BASE</code> – The list includes only base asset model properties for a given asset model ID. </p> </li>
    /// </ul>
    /// <p>Default: <code>BASE</code> </p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::ListAssetModelPropertiesFilter>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p> Filters the requested list of asset model properties. You can choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ALL</code> – The list includes all asset model properties for a given asset model ID. </p> </li>
    /// <li> <p> <code>BASE</code> – The list includes only base asset model properties for a given asset model ID. </p> </li>
    /// </ul>
    /// <p>Default: <code>BASE</code> </p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::ListAssetModelPropertiesFilter> {
        self.inner.get_filter()
    }
}
