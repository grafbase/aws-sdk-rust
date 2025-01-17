// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRegexPatternSet`](crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`regex_pattern_set_id(impl ::std::convert::Into<String>)`](crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder::regex_pattern_set_id) / [`set_regex_pattern_set_id(Option<String>)`](crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder::set_regex_pattern_set_id): <p>The <code>RegexPatternSetId</code> of the <code>RegexPatternSet</code> that you want to delete. <code>RegexPatternSetId</code> is returned by <code>CreateRegexPatternSet</code> and by <code>ListRegexPatternSets</code>.</p>
    ///   - [`change_token(impl ::std::convert::Into<String>)`](crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder::change_token) / [`set_change_token(Option<String>)`](crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder::set_change_token): <p>The value returned by the most recent call to <code>GetChangeToken</code>.</p>
    /// - On success, responds with [`DeleteRegexPatternSetOutput`](crate::operation::delete_regex_pattern_set::DeleteRegexPatternSetOutput) with field(s):
    ///   - [`change_token(Option<String>)`](crate::operation::delete_regex_pattern_set::DeleteRegexPatternSetOutput::change_token): <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <code>GetChangeTokenStatus</code>.</p>
    /// - On failure, responds with [`SdkError<DeleteRegexPatternSetError>`](crate::operation::delete_regex_pattern_set::DeleteRegexPatternSetError)
    pub fn delete_regex_pattern_set(&self) -> crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder {
        crate::operation::delete_regex_pattern_set::builders::DeleteRegexPatternSetFluentBuilder::new(self.handle.clone())
    }
}
