// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_service_settings::_update_service_settings_output::UpdateServiceSettingsOutputBuilder;

pub use crate::operation::update_service_settings::_update_service_settings_input::UpdateServiceSettingsInputBuilder;

impl UpdateServiceSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_service_settings::UpdateServiceSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_settings::UpdateServiceSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_service_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateServiceSettings`.
///
/// <p>Updates License Manager settings for the current Region.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateServiceSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_service_settings::builders::UpdateServiceSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateServiceSettingsFluentBuilder {
    /// Creates a new `UpdateServiceSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateServiceSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_service_settings::builders::UpdateServiceSettingsInputBuilder {
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
        crate::operation::update_service_settings::UpdateServiceSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_settings::UpdateServiceSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_service_settings::UpdateServiceSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_service_settings::UpdateServiceSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_service_settings::UpdateServiceSettingsOutput,
            crate::operation::update_service_settings::UpdateServiceSettingsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_service_settings::UpdateServiceSettingsError>,
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
    /// <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p>
    pub fn s3_bucket_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.s3_bucket_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p>
    pub fn set_s3_bucket_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_s3_bucket_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p>
    pub fn get_s3_bucket_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_s3_bucket_arn()
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p>
    pub fn sns_topic_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sns_topic_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p>
    pub fn set_sns_topic_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sns_topic_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p>
    pub fn get_sns_topic_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sns_topic_arn()
    }
    /// <p>Enables integration with Organizations for cross-account discovery.</p>
    pub fn organization_configuration(mut self, input: crate::types::OrganizationConfiguration) -> Self {
        self.inner = self.inner.organization_configuration(input);
        self
    }
    /// <p>Enables integration with Organizations for cross-account discovery.</p>
    pub fn set_organization_configuration(mut self, input: ::std::option::Option<crate::types::OrganizationConfiguration>) -> Self {
        self.inner = self.inner.set_organization_configuration(input);
        self
    }
    /// <p>Enables integration with Organizations for cross-account discovery.</p>
    pub fn get_organization_configuration(&self) -> &::std::option::Option<crate::types::OrganizationConfiguration> {
        self.inner.get_organization_configuration()
    }
    /// <p>Activates cross-account discovery.</p>
    pub fn enable_cross_accounts_discovery(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_cross_accounts_discovery(input);
        self
    }
    /// <p>Activates cross-account discovery.</p>
    pub fn set_enable_cross_accounts_discovery(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_cross_accounts_discovery(input);
        self
    }
    /// <p>Activates cross-account discovery.</p>
    pub fn get_enable_cross_accounts_discovery(&self) -> &::std::option::Option<bool> {
        self.inner.get_enable_cross_accounts_discovery()
    }
}
