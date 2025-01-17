// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_network_acl_entry::_delete_network_acl_entry_output::DeleteNetworkAclEntryOutputBuilder;

pub use crate::operation::delete_network_acl_entry::_delete_network_acl_entry_input::DeleteNetworkAclEntryInputBuilder;

impl DeleteNetworkAclEntryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_network_acl_entry();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteNetworkAclEntry`.
///
/// <p>Deletes the specified ingress or egress entry (rule) from the specified network ACL.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteNetworkAclEntryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_network_acl_entry::builders::DeleteNetworkAclEntryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteNetworkAclEntryFluentBuilder {
    /// Creates a new `DeleteNetworkAclEntry`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteNetworkAclEntry as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_network_acl_entry::builders::DeleteNetworkAclEntryInputBuilder {
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
        crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_network_acl_entry::DeleteNetworkAclEntry::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_network_acl_entry::DeleteNetworkAclEntry::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryOutput,
            crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryError>,
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
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>Indicates whether the rule is an egress rule.</p>
    pub fn egress(mut self, input: bool) -> Self {
        self.inner = self.inner.egress(input);
        self
    }
    /// <p>Indicates whether the rule is an egress rule.</p>
    pub fn set_egress(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_egress(input);
        self
    }
    /// <p>Indicates whether the rule is an egress rule.</p>
    pub fn get_egress(&self) -> &::std::option::Option<bool> {
        self.inner.get_egress()
    }
    /// <p>The ID of the network ACL.</p>
    pub fn network_acl_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_acl_id(input.into());
        self
    }
    /// <p>The ID of the network ACL.</p>
    pub fn set_network_acl_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_acl_id(input);
        self
    }
    /// <p>The ID of the network ACL.</p>
    pub fn get_network_acl_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_acl_id()
    }
    /// <p>The rule number of the entry to delete.</p>
    pub fn rule_number(mut self, input: i32) -> Self {
        self.inner = self.inner.rule_number(input);
        self
    }
    /// <p>The rule number of the entry to delete.</p>
    pub fn set_rule_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_rule_number(input);
        self
    }
    /// <p>The rule number of the entry to delete.</p>
    pub fn get_rule_number(&self) -> &::std::option::Option<i32> {
        self.inner.get_rule_number()
    }
}
