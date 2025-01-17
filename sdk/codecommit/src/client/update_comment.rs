// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateComment`](crate::operation::update_comment::builders::UpdateCommentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`comment_id(impl ::std::convert::Into<String>)`](crate::operation::update_comment::builders::UpdateCommentFluentBuilder::comment_id) / [`set_comment_id(Option<String>)`](crate::operation::update_comment::builders::UpdateCommentFluentBuilder::set_comment_id): <p>The system-generated ID of the comment you want to update. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    ///   - [`content(impl ::std::convert::Into<String>)`](crate::operation::update_comment::builders::UpdateCommentFluentBuilder::content) / [`set_content(Option<String>)`](crate::operation::update_comment::builders::UpdateCommentFluentBuilder::set_content): <p>The updated content to replace the existing content of the comment.</p>
    /// - On success, responds with [`UpdateCommentOutput`](crate::operation::update_comment::UpdateCommentOutput) with field(s):
    ///   - [`comment(Option<Comment>)`](crate::operation::update_comment::UpdateCommentOutput::comment): <p>Information about the updated comment.</p>
    /// - On failure, responds with [`SdkError<UpdateCommentError>`](crate::operation::update_comment::UpdateCommentError)
    pub fn update_comment(&self) -> crate::operation::update_comment::builders::UpdateCommentFluentBuilder {
        crate::operation::update_comment::builders::UpdateCommentFluentBuilder::new(self.handle.clone())
    }
}
