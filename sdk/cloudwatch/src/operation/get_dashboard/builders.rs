// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_dashboard::_get_dashboard_output::GetDashboardOutputBuilder;

pub use crate::operation::get_dashboard::_get_dashboard_input::GetDashboardInputBuilder;

impl GetDashboardInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_dashboard::GetDashboardOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_dashboard::GetDashboardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_dashboard();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetDashboard`.
///
/// <p>Displays the details of the dashboard that you specify.</p>
/// <p>To copy an existing dashboard, use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code> to create the copy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDashboardFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_dashboard::builders::GetDashboardInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetDashboardFluentBuilder {
    /// Creates a new `GetDashboard`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetDashboard as a reference.
    pub fn as_input(&self) -> &crate::operation::get_dashboard::builders::GetDashboardInputBuilder {
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
        crate::operation::get_dashboard::GetDashboardOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_dashboard::GetDashboardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_dashboard::GetDashboard::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_dashboard::GetDashboard::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_dashboard::GetDashboardOutput,
            crate::operation::get_dashboard::GetDashboardError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_dashboard::GetDashboardError>,
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
    /// <p>The name of the dashboard to be described.</p>
    pub fn dashboard_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dashboard_name(input.into());
        self
    }
    /// <p>The name of the dashboard to be described.</p>
    pub fn set_dashboard_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dashboard_name(input);
        self
    }
    /// <p>The name of the dashboard to be described.</p>
    pub fn get_dashboard_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dashboard_name()
    }
}
