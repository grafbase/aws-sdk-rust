// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_monitoring_members::_stop_monitoring_members_output::StopMonitoringMembersOutputBuilder;

pub use crate::operation::stop_monitoring_members::_stop_monitoring_members_input::StopMonitoringMembersInputBuilder;

impl StopMonitoringMembersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_monitoring_members::StopMonitoringMembersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_monitoring_members::StopMonitoringMembersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_monitoring_members();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopMonitoringMembers`.
///
/// <p>Stops GuardDuty monitoring for the specified member accounts. Use the <code>StartMonitoringMembers</code> operation to restart monitoring for those accounts.</p>
/// <p>With <code>autoEnableOrganizationMembers</code> configuration for your organization set to <code>ALL</code>, you'll receive an error if you attempt to stop monitoring the member accounts in your organization.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopMonitoringMembersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_monitoring_members::builders::StopMonitoringMembersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl StopMonitoringMembersFluentBuilder {
    /// Creates a new `StopMonitoringMembers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopMonitoringMembers as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_monitoring_members::builders::StopMonitoringMembersInputBuilder {
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
        crate::operation::stop_monitoring_members::StopMonitoringMembersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_monitoring_members::StopMonitoringMembersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_monitoring_members::StopMonitoringMembers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_monitoring_members::StopMonitoringMembers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::stop_monitoring_members::StopMonitoringMembersOutput,
            crate::operation::stop_monitoring_members::StopMonitoringMembersError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::stop_monitoring_members::StopMonitoringMembersError>,
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
    /// <p>The unique ID of the detector associated with the GuardDuty administrator account that is monitoring member accounts.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector associated with the GuardDuty administrator account that is monitoring member accounts.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The unique ID of the detector associated with the GuardDuty administrator account that is monitoring member accounts.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// Appends an item to `AccountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>A list of account IDs for the member accounts to stop monitoring.</p>
    pub fn account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_ids(input.into());
        self
    }
    /// <p>A list of account IDs for the member accounts to stop monitoring.</p>
    pub fn set_account_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
    /// <p>A list of account IDs for the member accounts to stop monitoring.</p>
    pub fn get_account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_account_ids()
    }
}
