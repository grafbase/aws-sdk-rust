// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLexicons`](crate::operation::list_lexicons::builders::ListLexiconsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_lexicons::builders::ListLexiconsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_lexicons::builders::ListLexiconsFluentBuilder::set_next_token): <p>An opaque pagination token returned from previous <code>ListLexicons</code> operation. If present, indicates where to continue the list of lexicons.</p>
    /// - On success, responds with [`ListLexiconsOutput`](crate::operation::list_lexicons::ListLexiconsOutput) with field(s):
    ///   - [`lexicons(Option<Vec<LexiconDescription>>)`](crate::operation::list_lexicons::ListLexiconsOutput::lexicons): <p>A list of lexicon names and attributes.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_lexicons::ListLexiconsOutput::next_token): <p>The pagination token to use in the next request to continue the listing of lexicons. <code>NextToken</code> is returned only if the response is truncated.</p>
    /// - On failure, responds with [`SdkError<ListLexiconsError>`](crate::operation::list_lexicons::ListLexiconsError)
    pub fn list_lexicons(&self) -> crate::operation::list_lexicons::builders::ListLexiconsFluentBuilder {
        crate::operation::list_lexicons::builders::ListLexiconsFluentBuilder::new(self.handle.clone())
    }
}
