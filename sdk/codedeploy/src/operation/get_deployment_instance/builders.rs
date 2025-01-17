// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_deployment_instance::_get_deployment_instance_output::GetDeploymentInstanceOutputBuilder;

pub use crate::operation::get_deployment_instance::_get_deployment_instance_input::GetDeploymentInstanceInputBuilder;

impl GetDeploymentInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_deployment_instance::GetDeploymentInstanceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_deployment_instance::GetDeploymentInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_deployment_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetDeploymentInstance`.
///
/// <p>Gets information about an instance as part of a deployment.</p>
#[deprecated(note = "This operation is deprecated, use GetDeploymentTarget instead.")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDeploymentInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_deployment_instance::builders::GetDeploymentInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetDeploymentInstanceFluentBuilder {
    /// Creates a new `GetDeploymentInstance`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetDeploymentInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::get_deployment_instance::builders::GetDeploymentInstanceInputBuilder {
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
        crate::operation::get_deployment_instance::GetDeploymentInstanceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_deployment_instance::GetDeploymentInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_deployment_instance::GetDeploymentInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_deployment_instance::GetDeploymentInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_deployment_instance::GetDeploymentInstanceOutput,
            crate::operation::get_deployment_instance::GetDeploymentInstanceError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_deployment_instance::GetDeploymentInstanceError>,
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
    /// <p> The unique ID of a deployment. </p>
    pub fn deployment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.deployment_id(input.into());
        self
    }
    /// <p> The unique ID of a deployment. </p>
    pub fn set_deployment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_deployment_id(input);
        self
    }
    /// <p> The unique ID of a deployment. </p>
    pub fn get_deployment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_deployment_id()
    }
    /// <p> The unique ID of an instance in the deployment group. </p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p> The unique ID of an instance in the deployment group. </p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p> The unique ID of an instance in the deployment group. </p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
}
