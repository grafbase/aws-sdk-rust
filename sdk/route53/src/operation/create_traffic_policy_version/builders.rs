// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_traffic_policy_version::_create_traffic_policy_version_output::CreateTrafficPolicyVersionOutputBuilder;

pub use crate::operation::create_traffic_policy_version::_create_traffic_policy_version_input::CreateTrafficPolicyVersionInputBuilder;

impl CreateTrafficPolicyVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_traffic_policy_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateTrafficPolicyVersion`.
///
/// <p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTrafficPolicyVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateTrafficPolicyVersionFluentBuilder {
    /// Creates a new `CreateTrafficPolicyVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateTrafficPolicyVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionInputBuilder {
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
        crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionOutput,
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError>,
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
    /// <p>The ID of the traffic policy for which you want to create a new version.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the traffic policy for which you want to create a new version.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the traffic policy for which you want to create a new version.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
    pub fn document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.document(input.into());
        self
    }
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
    pub fn set_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_document(input);
        self
    }
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
    pub fn get_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_document()
    }
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comment()
    }
}
