// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_member::_create_member_output::CreateMemberOutputBuilder;

pub use crate::operation::create_member::_create_member_input::CreateMemberInputBuilder;

impl CreateMemberInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_member::CreateMemberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_member::CreateMemberError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_member();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateMember`.
///
/// <p>Associates an account with an Amazon Macie administrator account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMemberFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_member::builders::CreateMemberInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateMemberFluentBuilder {
    /// Creates a new `CreateMember`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateMember as a reference.
    pub fn as_input(&self) -> &crate::operation::create_member::builders::CreateMemberInputBuilder {
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
        crate::operation::create_member::CreateMemberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_member::CreateMemberError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_member::CreateMember::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_member::CreateMember::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_member::CreateMemberOutput,
            crate::operation::create_member::CreateMemberError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_member::CreateMemberError>,
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
    /// <p>The details of the account to associate with the administrator account.</p>
    pub fn account(mut self, input: crate::types::AccountDetail) -> Self {
        self.inner = self.inner.account(input);
        self
    }
    /// <p>The details of the account to associate with the administrator account.</p>
    pub fn set_account(mut self, input: ::std::option::Option<crate::types::AccountDetail>) -> Self {
        self.inner = self.inner.set_account(input);
        self
    }
    /// <p>The details of the account to associate with the administrator account.</p>
    pub fn get_account(&self) -> &::std::option::Option<crate::types::AccountDetail> {
        self.inner.get_account()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.</p>
    /// <p>An account can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.</p>
    /// <p>An account can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.</p>
    /// <p>An account can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
