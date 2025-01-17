// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_get_application_revisions::_batch_get_application_revisions_output::BatchGetApplicationRevisionsOutputBuilder;

pub use crate::operation::batch_get_application_revisions::_batch_get_application_revisions_input::BatchGetApplicationRevisionsInputBuilder;

impl BatchGetApplicationRevisionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_get_application_revisions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchGetApplicationRevisions`.
///
/// <p>Gets information about one or more application revisions. The maximum number of application revisions that can be returned is 25.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchGetApplicationRevisionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_get_application_revisions::builders::BatchGetApplicationRevisionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl BatchGetApplicationRevisionsFluentBuilder {
    /// Creates a new `BatchGetApplicationRevisions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchGetApplicationRevisions as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_get_application_revisions::builders::BatchGetApplicationRevisionsInputBuilder {
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
        crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_get_application_revisions::BatchGetApplicationRevisions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_get_application_revisions::BatchGetApplicationRevisions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsOutput,
            crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_get_application_revisions::BatchGetApplicationRevisionsError>,
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
    /// <p>The name of an CodeDeploy application about which to get revision information.</p>
    pub fn application_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of an CodeDeploy application about which to get revision information.</p>
    pub fn set_application_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The name of an CodeDeploy application about which to get revision information.</p>
    pub fn get_application_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_name()
    }
    /// Appends an item to `revisions`.
    ///
    /// To override the contents of this collection use [`set_revisions`](Self::set_revisions).
    ///
    /// <p>An array of <code>RevisionLocation</code> objects that specify information to get about the application revisions, including type and location. The maximum number of <code>RevisionLocation</code> objects you can specify is 25.</p>
    pub fn revisions(mut self, input: crate::types::RevisionLocation) -> Self {
        self.inner = self.inner.revisions(input);
        self
    }
    /// <p>An array of <code>RevisionLocation</code> objects that specify information to get about the application revisions, including type and location. The maximum number of <code>RevisionLocation</code> objects you can specify is 25.</p>
    pub fn set_revisions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RevisionLocation>>) -> Self {
        self.inner = self.inner.set_revisions(input);
        self
    }
    /// <p>An array of <code>RevisionLocation</code> objects that specify information to get about the application revisions, including type and location. The maximum number of <code>RevisionLocation</code> objects you can specify is 25.</p>
    pub fn get_revisions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RevisionLocation>> {
        self.inner.get_revisions()
    }
}
