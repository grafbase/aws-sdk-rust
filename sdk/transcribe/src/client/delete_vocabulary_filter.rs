// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVocabularyFilter`](crate::operation::delete_vocabulary_filter::builders::DeleteVocabularyFilterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vocabulary_filter_name(impl ::std::convert::Into<String>)`](crate::operation::delete_vocabulary_filter::builders::DeleteVocabularyFilterFluentBuilder::vocabulary_filter_name) / [`set_vocabulary_filter_name(Option<String>)`](crate::operation::delete_vocabulary_filter::builders::DeleteVocabularyFilterFluentBuilder::set_vocabulary_filter_name): <p>The name of the custom vocabulary filter you want to delete. Custom vocabulary filter names are case sensitive.</p>
    /// - On success, responds with [`DeleteVocabularyFilterOutput`](crate::operation::delete_vocabulary_filter::DeleteVocabularyFilterOutput)
    /// - On failure, responds with [`SdkError<DeleteVocabularyFilterError>`](crate::operation::delete_vocabulary_filter::DeleteVocabularyFilterError)
    pub fn delete_vocabulary_filter(&self) -> crate::operation::delete_vocabulary_filter::builders::DeleteVocabularyFilterFluentBuilder {
        crate::operation::delete_vocabulary_filter::builders::DeleteVocabularyFilterFluentBuilder::new(self.handle.clone())
    }
}
