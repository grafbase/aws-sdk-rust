// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartViewerSessionRevocation`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl ::std::convert::Into<String>)`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::set_channel_arn): <p>The ARN of the channel associated with the viewer session to revoke.</p>
    ///   - [`viewer_id(impl ::std::convert::Into<String>)`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::viewer_id) / [`set_viewer_id(Option<String>)`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::set_viewer_id): <p>The ID of the viewer associated with the viewer session to revoke. Do not use this field for personally identifying, confidential, or sensitive information.</p>
    ///   - [`viewer_session_versions_less_than_or_equal_to(i32)`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::viewer_session_versions_less_than_or_equal_to) / [`set_viewer_session_versions_less_than_or_equal_to(i32)`](crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::set_viewer_session_versions_less_than_or_equal_to): <p>An optional filter on which versions of the viewer session to revoke. All versions less than or equal to the specified version will be revoked. Default: 0.</p>
    /// - On success, responds with [`StartViewerSessionRevocationOutput`](crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationOutput)
    /// - On failure, responds with [`SdkError<StartViewerSessionRevocationError>`](crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError)
    pub fn start_viewer_session_revocation(
        &self,
    ) -> crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder {
        crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationFluentBuilder::new(self.handle.clone())
    }
}
