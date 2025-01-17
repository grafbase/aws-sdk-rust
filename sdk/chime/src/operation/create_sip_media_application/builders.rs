// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_sip_media_application::_create_sip_media_application_output::CreateSipMediaApplicationOutputBuilder;

pub use crate::operation::create_sip_media_application::_create_sip_media_application_input::CreateSipMediaApplicationInputBuilder;

impl CreateSipMediaApplicationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_sip_media_application::CreateSipMediaApplicationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sip_media_application::CreateSipMediaApplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_sip_media_application();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSipMediaApplication`.
///
/// <p>Creates a SIP media application.</p> <important>
/// <p> <b>This API is is no longer supported and will not be updated.</b> We recommend using the latest version, <a href="https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_voice-chime_CreateSipMediaApplication.html">CreateSipMediaApplication</a>, in the Amazon Chime SDK.</p>
/// <p>Using the latest version requires migrating to a dedicated namespace. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/migrate-from-chm-namespace.html">Migrating from the Amazon Chime namespace</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// </important>
#[deprecated(note = "Replaced by CreateSipMediaApplication in the Amazon Chime SDK Voice Namespace")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSipMediaApplicationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateSipMediaApplicationFluentBuilder {
    /// Creates a new `CreateSipMediaApplication`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSipMediaApplication as a reference.
    pub fn as_input(&self) -> &crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationInputBuilder {
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
        crate::operation::create_sip_media_application::CreateSipMediaApplicationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sip_media_application::CreateSipMediaApplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_sip_media_application::CreateSipMediaApplication::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_sip_media_application::CreateSipMediaApplication::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_sip_media_application::CreateSipMediaApplicationOutput,
            crate::operation::create_sip_media_application::CreateSipMediaApplicationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_sip_media_application::CreateSipMediaApplicationError>,
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
    /// <p>The AWS Region assigned to the SIP media application.</p>
    pub fn aws_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_region(input.into());
        self
    }
    /// <p>The AWS Region assigned to the SIP media application.</p>
    pub fn set_aws_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_region(input);
        self
    }
    /// <p>The AWS Region assigned to the SIP media application.</p>
    pub fn get_aws_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_region()
    }
    /// <p>The SIP media application name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The SIP media application name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The SIP media application name.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// Appends an item to `Endpoints`.
    ///
    /// To override the contents of this collection use [`set_endpoints`](Self::set_endpoints).
    ///
    /// <p>List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported.</p>
    pub fn endpoints(mut self, input: crate::types::SipMediaApplicationEndpoint) -> Self {
        self.inner = self.inner.endpoints(input);
        self
    }
    /// <p>List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported.</p>
    pub fn set_endpoints(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SipMediaApplicationEndpoint>>) -> Self {
        self.inner = self.inner.set_endpoints(input);
        self
    }
    /// <p>List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported.</p>
    pub fn get_endpoints(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SipMediaApplicationEndpoint>> {
        self.inner.get_endpoints()
    }
}
