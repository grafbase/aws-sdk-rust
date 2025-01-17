// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListChangeSets`](crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`stack_name(impl ::std::convert::Into<String>)`](crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder::stack_name) / [`set_stack_name(Option<String>)`](crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder::set_stack_name): <p>The name or the Amazon Resource Name (ARN) of the stack for which you want to list change sets.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder::set_next_token): <p>A string (provided by the <code>ListChangeSets</code> response output) that identifies the next page of change sets that you want to retrieve.</p>
    /// - On success, responds with [`ListChangeSetsOutput`](crate::operation::list_change_sets::ListChangeSetsOutput) with field(s):
    ///   - [`summaries(Option<Vec<ChangeSetSummary>>)`](crate::operation::list_change_sets::ListChangeSetsOutput::summaries): <p>A list of <code>ChangeSetSummary</code> structures that provides the ID and status of each change set for the specified stack.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_change_sets::ListChangeSetsOutput::next_token): <p>If the output exceeds 1 MB, a string that identifies the next page of change sets. If there is no additional page, this value is <code>null</code>.</p>
    /// - On failure, responds with [`SdkError<ListChangeSetsError>`](crate::operation::list_change_sets::ListChangeSetsError)
    pub fn list_change_sets(&self) -> crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder {
        crate::operation::list_change_sets::builders::ListChangeSetsFluentBuilder::new(self.handle.clone())
    }
}
