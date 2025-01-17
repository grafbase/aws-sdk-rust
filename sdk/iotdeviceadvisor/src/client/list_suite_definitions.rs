// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSuiteDefinitions`](crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder::set_max_results): <p>The maximum number of results to return at once.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder::set_next_token): <p>A token used to get the next set of results.</p>
    /// - On success, responds with [`ListSuiteDefinitionsOutput`](crate::operation::list_suite_definitions::ListSuiteDefinitionsOutput) with field(s):
    ///   - [`suite_definition_information_list(Option<Vec<SuiteDefinitionInformation>>)`](crate::operation::list_suite_definitions::ListSuiteDefinitionsOutput::suite_definition_information_list): <p>An array of objects that provide summaries of information about the suite definitions in the list.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_suite_definitions::ListSuiteDefinitionsOutput::next_token): <p>A token used to get the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListSuiteDefinitionsError>`](crate::operation::list_suite_definitions::ListSuiteDefinitionsError)
    pub fn list_suite_definitions(&self) -> crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder {
        crate::operation::list_suite_definitions::builders::ListSuiteDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
