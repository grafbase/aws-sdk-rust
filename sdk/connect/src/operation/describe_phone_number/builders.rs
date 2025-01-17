// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_phone_number::_describe_phone_number_output::DescribePhoneNumberOutputBuilder;

pub use crate::operation::describe_phone_number::_describe_phone_number_input::DescribePhoneNumberInputBuilder;

impl DescribePhoneNumberInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_phone_number::DescribePhoneNumberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_phone_number::DescribePhoneNumberError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_phone_number();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribePhoneNumber`.
///
/// <p>Gets details and status of a phone number that’s claimed to your Amazon Connect instance or traffic distribution group.</p> <important>
/// <p>If the number is claimed to a traffic distribution group, and you are calling in the Amazon Web Services Region where the traffic distribution group was created, you can use either a phone number ARN or UUID value for the <code>PhoneNumberId</code> URI request parameter. However, if the number is claimed to a traffic distribution group and you are calling this API in the alternate Amazon Web Services Region associated with the traffic distribution group, you must provide a full phone number ARN. If a UUID is provided in this scenario, you will receive a <code>ResourceNotFoundException</code>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribePhoneNumberFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_phone_number::builders::DescribePhoneNumberInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribePhoneNumberFluentBuilder {
    /// Creates a new `DescribePhoneNumber`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribePhoneNumber as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_phone_number::builders::DescribePhoneNumberInputBuilder {
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
        crate::operation::describe_phone_number::DescribePhoneNumberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_phone_number::DescribePhoneNumberError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_phone_number::DescribePhoneNumber::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_phone_number::DescribePhoneNumber::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_phone_number::DescribePhoneNumberOutput,
            crate::operation::describe_phone_number::DescribePhoneNumberError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_phone_number::DescribePhoneNumberError>,
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
    /// <p>A unique identifier for the phone number.</p>
    pub fn phone_number_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.phone_number_id(input.into());
        self
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn set_phone_number_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_phone_number_id(input);
        self
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn get_phone_number_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_phone_number_id()
    }
}
