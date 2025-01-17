// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the utterances in the results of the test set execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UtteranceLevelTestResults {
    /// <p>Contains information about an utterance in the results of the test set execution.</p>
    pub items: ::std::option::Option<::std::vec::Vec<crate::types::UtteranceLevelTestResultItem>>,
}
impl UtteranceLevelTestResults {
    /// <p>Contains information about an utterance in the results of the test set execution.</p>
    pub fn items(&self) -> ::std::option::Option<&[crate::types::UtteranceLevelTestResultItem]> {
        self.items.as_deref()
    }
}
impl UtteranceLevelTestResults {
    /// Creates a new builder-style object to manufacture [`UtteranceLevelTestResults`](crate::types::UtteranceLevelTestResults).
    pub fn builder() -> crate::types::builders::UtteranceLevelTestResultsBuilder {
        crate::types::builders::UtteranceLevelTestResultsBuilder::default()
    }
}

/// A builder for [`UtteranceLevelTestResults`](crate::types::UtteranceLevelTestResults).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UtteranceLevelTestResultsBuilder {
    pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::types::UtteranceLevelTestResultItem>>,
}
impl UtteranceLevelTestResultsBuilder {
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>Contains information about an utterance in the results of the test set execution.</p>
    pub fn items(mut self, input: crate::types::UtteranceLevelTestResultItem) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains information about an utterance in the results of the test set execution.</p>
    pub fn set_items(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UtteranceLevelTestResultItem>>) -> Self {
        self.items = input;
        self
    }
    /// <p>Contains information about an utterance in the results of the test set execution.</p>
    pub fn get_items(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UtteranceLevelTestResultItem>> {
        &self.items
    }
    /// Consumes the builder and constructs a [`UtteranceLevelTestResults`](crate::types::UtteranceLevelTestResults).
    pub fn build(self) -> crate::types::UtteranceLevelTestResults {
        crate::types::UtteranceLevelTestResults { items: self.items }
    }
}
