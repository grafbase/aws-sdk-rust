// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_findings_feedback::_update_findings_feedback_output::UpdateFindingsFeedbackOutputBuilder;

pub use crate::operation::update_findings_feedback::_update_findings_feedback_input::UpdateFindingsFeedbackInputBuilder;

impl UpdateFindingsFeedbackInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_findings_feedback::UpdateFindingsFeedbackOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_findings_feedback::UpdateFindingsFeedbackError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_findings_feedback();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateFindingsFeedback`.
///
/// <p>Marks the specified GuardDuty findings as useful or not useful.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateFindingsFeedbackFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_findings_feedback::builders::UpdateFindingsFeedbackInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateFindingsFeedbackFluentBuilder {
    /// Creates a new `UpdateFindingsFeedback`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateFindingsFeedback as a reference.
    pub fn as_input(&self) -> &crate::operation::update_findings_feedback::builders::UpdateFindingsFeedbackInputBuilder {
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
        crate::operation::update_findings_feedback::UpdateFindingsFeedbackOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_findings_feedback::UpdateFindingsFeedbackError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_findings_feedback::UpdateFindingsFeedback::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_findings_feedback::UpdateFindingsFeedback::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_findings_feedback::UpdateFindingsFeedbackOutput,
            crate::operation::update_findings_feedback::UpdateFindingsFeedbackError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_findings_feedback::UpdateFindingsFeedbackError>,
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
    /// <p>The ID of the detector associated with the findings to update feedback for.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The ID of the detector associated with the findings to update feedback for.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The ID of the detector associated with the findings to update feedback for.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// Appends an item to `FindingIds`.
    ///
    /// To override the contents of this collection use [`set_finding_ids`](Self::set_finding_ids).
    ///
    /// <p>The IDs of the findings that you want to mark as useful or not useful.</p>
    pub fn finding_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.finding_ids(input.into());
        self
    }
    /// <p>The IDs of the findings that you want to mark as useful or not useful.</p>
    pub fn set_finding_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_finding_ids(input);
        self
    }
    /// <p>The IDs of the findings that you want to mark as useful or not useful.</p>
    pub fn get_finding_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_finding_ids()
    }
    /// <p>The feedback for the finding.</p>
    pub fn feedback(mut self, input: crate::types::Feedback) -> Self {
        self.inner = self.inner.feedback(input);
        self
    }
    /// <p>The feedback for the finding.</p>
    pub fn set_feedback(mut self, input: ::std::option::Option<crate::types::Feedback>) -> Self {
        self.inner = self.inner.set_feedback(input);
        self
    }
    /// <p>The feedback for the finding.</p>
    pub fn get_feedback(&self) -> &::std::option::Option<crate::types::Feedback> {
        self.inner.get_feedback()
    }
    /// <p>Additional feedback about the GuardDuty findings.</p>
    pub fn comments(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comments(input.into());
        self
    }
    /// <p>Additional feedback about the GuardDuty findings.</p>
    pub fn set_comments(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comments(input);
        self
    }
    /// <p>Additional feedback about the GuardDuty findings.</p>
    pub fn get_comments(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comments()
    }
}
