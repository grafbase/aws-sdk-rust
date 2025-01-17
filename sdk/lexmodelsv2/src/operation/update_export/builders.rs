// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_export::_update_export_output::UpdateExportOutputBuilder;

pub use crate::operation::update_export::_update_export_input::UpdateExportInputBuilder;

impl UpdateExportInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_export::UpdateExportOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_export::UpdateExportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_export();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateExport`.
///
/// <p>Updates the password used to protect an export zip archive.</p>
/// <p>The password is not required. If you don't supply a password, Amazon Lex generates a zip file that is not protected by a password. This is the archive that is available at the pre-signed S3 URL provided by the <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_DescribeExport.html">DescribeExport</a> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateExportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_export::builders::UpdateExportInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateExportFluentBuilder {
    /// Creates a new `UpdateExport`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateExport as a reference.
    pub fn as_input(&self) -> &crate::operation::update_export::builders::UpdateExportInputBuilder {
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
        crate::operation::update_export::UpdateExportOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_export::UpdateExportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_export::UpdateExport::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_export::UpdateExport::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_export::UpdateExportOutput,
            crate::operation::update_export::UpdateExportError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_export::UpdateExportError>,
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
    /// <p>The unique identifier Amazon Lex assigned to the export.</p>
    pub fn export_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.export_id(input.into());
        self
    }
    /// <p>The unique identifier Amazon Lex assigned to the export.</p>
    pub fn set_export_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_export_id(input);
        self
    }
    /// <p>The unique identifier Amazon Lex assigned to the export.</p>
    pub fn get_export_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_export_id()
    }
    /// <p>The new password to use to encrypt the export zip archive.</p>
    pub fn file_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.file_password(input.into());
        self
    }
    /// <p>The new password to use to encrypt the export zip archive.</p>
    pub fn set_file_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_file_password(input);
        self
    }
    /// <p>The new password to use to encrypt the export zip archive.</p>
    pub fn get_file_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_file_password()
    }
}
