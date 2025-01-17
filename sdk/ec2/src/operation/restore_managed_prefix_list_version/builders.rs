// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_managed_prefix_list_version::_restore_managed_prefix_list_version_output::RestoreManagedPrefixListVersionOutputBuilder;

pub use crate::operation::restore_managed_prefix_list_version::_restore_managed_prefix_list_version_input::RestoreManagedPrefixListVersionInputBuilder;

impl RestoreManagedPrefixListVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.restore_managed_prefix_list_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestoreManagedPrefixListVersion`.
///
/// <p>Restores the entries from a previous version of a managed prefix list to a new version of the prefix list.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreManagedPrefixListVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::restore_managed_prefix_list_version::builders::RestoreManagedPrefixListVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl RestoreManagedPrefixListVersionFluentBuilder {
    /// Creates a new `RestoreManagedPrefixListVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RestoreManagedPrefixListVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::restore_managed_prefix_list_version::builders::RestoreManagedPrefixListVersionInputBuilder {
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
        crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionOutput,
            crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::restore_managed_prefix_list_version::RestoreManagedPrefixListVersionError>,
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
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.prefix_list_id(input.into());
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn set_prefix_list_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_prefix_list_id(input);
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn get_prefix_list_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_prefix_list_id()
    }
    /// <p>The version to restore.</p>
    pub fn previous_version(mut self, input: i64) -> Self {
        self.inner = self.inner.previous_version(input);
        self
    }
    /// <p>The version to restore.</p>
    pub fn set_previous_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_previous_version(input);
        self
    }
    /// <p>The version to restore.</p>
    pub fn get_previous_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_previous_version()
    }
    /// <p>The current version number for the prefix list.</p>
    pub fn current_version(mut self, input: i64) -> Self {
        self.inner = self.inner.current_version(input);
        self
    }
    /// <p>The current version number for the prefix list.</p>
    pub fn set_current_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_version(input);
        self
    }
    /// <p>The current version number for the prefix list.</p>
    pub fn get_current_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_current_version()
    }
}
