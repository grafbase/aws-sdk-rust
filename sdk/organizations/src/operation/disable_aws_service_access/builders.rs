// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_aws_service_access::_disable_aws_service_access_output::DisableAwsServiceAccessOutputBuilder;

pub use crate::operation::disable_aws_service_access::_disable_aws_service_access_input::DisableAwsServiceAccessInputBuilder;

impl DisableAwsServiceAccessInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_aws_service_access::DisableAwsServiceAccessOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_aws_service_access::DisableAWSServiceAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_aws_service_access();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableAWSServiceAccess`.
///
/// <p>Disables the integration of an Amazon Web Services service (the service that is specified by <code>ServicePrincipal</code>) with Organizations. When you disable integration, the specified service no longer can create a <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from Organizations.</p> <important>
/// <p>We <b> <i>strongly recommend</i> </b> that you don't use this command to disable integration between Organizations and the specified Amazon Web Services service. Instead, use the console or commands that are provided by the specified service. This lets the trusted service perform any required initialization when enabling trusted access, such as creating any required resources and any required clean up of resources when disabling trusted access. </p>
/// <p>For information about how to disable trusted service access to your organization using the trusted service, see the <b>Learn more</b> link under the <b>Supports Trusted Access</b> column at <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services_list.html">Amazon Web Services services that you can use with Organizations</a>. on this page.</p>
/// <p>If you disable access by using this command, it causes the following actions to occur:</p>
/// <ul>
/// <li> <p>The service can no longer create a service-linked role in the accounts in your organization. This means that the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from Organizations. </p> </li>
/// <li> <p>The service can no longer perform tasks in the member accounts in the organization, unless those operations are explicitly permitted by the IAM policies that are attached to your roles. This includes any data aggregation from the member accounts to the management account, or to a delegated administrator account, where relevant.</p> </li>
/// <li> <p>Some services detect this and clean up any remaining data or resources related to the integration, while other services stop accessing the organization but leave any historical data and configuration in place to support a possible re-enabling of the integration.</p> </li>
/// </ul>
/// <p>Using the other service's console or commands to disable the integration ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other Amazon Web Services service. </p>
/// </important>
/// <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts </p>
/// <p>For more information about integrating other services with Organizations, including the list of services that work with Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating Organizations with Other Amazon Web Services Services</a> in the <i>Organizations User Guide.</i> </p>
/// <p>This operation can be called only from the organization's management account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableAWSServiceAccessFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_aws_service_access::builders::DisableAwsServiceAccessInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DisableAWSServiceAccessFluentBuilder {
    /// Creates a new `DisableAWSServiceAccess`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableAWSServiceAccess as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_aws_service_access::builders::DisableAwsServiceAccessInputBuilder {
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
        crate::operation::disable_aws_service_access::DisableAwsServiceAccessOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_aws_service_access::DisableAWSServiceAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_aws_service_access::DisableAWSServiceAccess::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_aws_service_access::DisableAWSServiceAccess::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::disable_aws_service_access::DisableAwsServiceAccessOutput,
            crate::operation::disable_aws_service_access::DisableAWSServiceAccessError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::disable_aws_service_access::DisableAWSServiceAccessError>,
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
    /// <p>The service principal name of the Amazon Web Services service for which you want to disable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    pub fn service_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_principal(input.into());
        self
    }
    /// <p>The service principal name of the Amazon Web Services service for which you want to disable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    pub fn set_service_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_principal(input);
        self
    }
    /// <p>The service principal name of the Amazon Web Services service for which you want to disable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    pub fn get_service_principal(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_principal()
    }
}
