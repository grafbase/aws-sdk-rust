// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_smb_local_groups::_update_smb_local_groups_output::UpdateSmbLocalGroupsOutputBuilder;

pub use crate::operation::update_smb_local_groups::_update_smb_local_groups_input::UpdateSmbLocalGroupsInputBuilder;

impl UpdateSmbLocalGroupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_smb_local_groups::UpdateSmbLocalGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_smb_local_groups::UpdateSMBLocalGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_smb_local_groups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSMBLocalGroups`.
///
/// <p>Updates the list of Active Directory users and groups that have special permissions for SMB file shares on the gateway.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSMBLocalGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_smb_local_groups::builders::UpdateSmbLocalGroupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateSMBLocalGroupsFluentBuilder {
    /// Creates a new `UpdateSMBLocalGroups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSMBLocalGroups as a reference.
    pub fn as_input(&self) -> &crate::operation::update_smb_local_groups::builders::UpdateSmbLocalGroupsInputBuilder {
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
        crate::operation::update_smb_local_groups::UpdateSmbLocalGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_smb_local_groups::UpdateSMBLocalGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_smb_local_groups::UpdateSMBLocalGroups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_smb_local_groups::UpdateSMBLocalGroups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_smb_local_groups::UpdateSmbLocalGroupsOutput,
            crate::operation::update_smb_local_groups::UpdateSMBLocalGroupsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_smb_local_groups::UpdateSMBLocalGroupsError>,
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
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn get_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_arn()
    }
    /// <p>A list of Active Directory users and groups that you want to grant special permissions for SMB file shares on the gateway.</p>
    pub fn smb_local_groups(mut self, input: crate::types::SmbLocalGroups) -> Self {
        self.inner = self.inner.smb_local_groups(input);
        self
    }
    /// <p>A list of Active Directory users and groups that you want to grant special permissions for SMB file shares on the gateway.</p>
    pub fn set_smb_local_groups(mut self, input: ::std::option::Option<crate::types::SmbLocalGroups>) -> Self {
        self.inner = self.inner.set_smb_local_groups(input);
        self
    }
    /// <p>A list of Active Directory users and groups that you want to grant special permissions for SMB file shares on the gateway.</p>
    pub fn get_smb_local_groups(&self) -> &::std::option::Option<crate::types::SmbLocalGroups> {
        self.inner.get_smb_local_groups()
    }
}
