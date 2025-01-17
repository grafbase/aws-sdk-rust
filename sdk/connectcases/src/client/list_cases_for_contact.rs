// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCasesForContact`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl ::std::convert::Into<String>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::set_domain_id): <p>The unique identifier of the Cases domain. </p>
    ///   - [`contact_arn(impl ::std::convert::Into<String>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::contact_arn) / [`set_contact_arn(Option<String>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::set_contact_arn): <p>A unique identifier of a contact in Amazon Connect.</p>
    ///   - [`max_results(i32)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::set_max_results): <p>The maximum number of results to return per page.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    /// - On success, responds with [`ListCasesForContactOutput`](crate::operation::list_cases_for_contact::ListCasesForContactOutput) with field(s):
    ///   - [`cases(Option<Vec<CaseSummary>>)`](crate::operation::list_cases_for_contact::ListCasesForContactOutput::cases): <p>A list of Case summary information.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_cases_for_contact::ListCasesForContactOutput::next_token): <p>The token for the next set of results. This is null if there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListCasesForContactError>`](crate::operation::list_cases_for_contact::ListCasesForContactError)
    pub fn list_cases_for_contact(&self) -> crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder {
        crate::operation::list_cases_for_contact::builders::ListCasesForContactFluentBuilder::new(self.handle.clone())
    }
}
