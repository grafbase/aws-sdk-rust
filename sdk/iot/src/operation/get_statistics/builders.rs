// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_statistics::_get_statistics_output::GetStatisticsOutputBuilder;

pub use crate::operation::get_statistics::_get_statistics_input::GetStatisticsInputBuilder;

impl GetStatisticsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_statistics::GetStatisticsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_statistics::GetStatisticsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_statistics();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetStatistics`.
///
/// <p>Returns the count, average, sum, minimum, maximum, sum of squares, variance, and standard deviation for the specified aggregated field. If the aggregation field is of type <code>String</code>, only the count statistic is returned.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">GetStatistics</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetStatisticsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_statistics::builders::GetStatisticsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetStatisticsFluentBuilder {
    /// Creates a new `GetStatistics`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetStatistics as a reference.
    pub fn as_input(&self) -> &crate::operation::get_statistics::builders::GetStatisticsInputBuilder {
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
        crate::operation::get_statistics::GetStatisticsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_statistics::GetStatisticsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_statistics::GetStatistics::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_statistics::GetStatistics::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_statistics::GetStatisticsOutput,
            crate::operation::get_statistics::GetStatisticsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_statistics::GetStatisticsError>,
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
    /// <p>The name of the index to search. The default value is <code>AWS_Things</code>.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_name(input.into());
        self
    }
    /// <p>The name of the index to search. The default value is <code>AWS_Things</code>.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_name(input);
        self
    }
    /// <p>The name of the index to search. The default value is <code>AWS_Things</code>.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_index_name()
    }
    /// <p>The query used to search. You can specify "*" for the query string to get the count of all indexed things in your Amazon Web Services account.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_string(input.into());
        self
    }
    /// <p>The query used to search. You can specify "*" for the query string to get the count of all indexed things in your Amazon Web Services account.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_string(input);
        self
    }
    /// <p>The query used to search. You can specify "*" for the query string to get the count of all indexed things in your Amazon Web Services account.</p>
    pub fn get_query_string(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_string()
    }
    /// <p>The aggregation field name.</p>
    pub fn aggregation_field(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aggregation_field(input.into());
        self
    }
    /// <p>The aggregation field name.</p>
    pub fn set_aggregation_field(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aggregation_field(input);
        self
    }
    /// <p>The aggregation field name.</p>
    pub fn get_aggregation_field(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aggregation_field()
    }
    /// <p>The version of the query used to search.</p>
    pub fn query_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_version(input.into());
        self
    }
    /// <p>The version of the query used to search.</p>
    pub fn set_query_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_version(input);
        self
    }
    /// <p>The version of the query used to search.</p>
    pub fn get_query_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_version()
    }
}
