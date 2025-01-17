// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SearchHoursOfOperations`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::set_max_results): <p>The maximum number of results to return per page.</p>
    ///   - [`search_filter(HoursOfOperationSearchFilter)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::search_filter) / [`set_search_filter(Option<HoursOfOperationSearchFilter>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::set_search_filter): <p>Filters to be applied to search results.</p>
    ///   - [`search_criteria(HoursOfOperationSearchCriteria)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::search_criteria) / [`set_search_criteria(Option<HoursOfOperationSearchCriteria>)`](crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::set_search_criteria): <p>The search criteria to be used to return hours of operations.</p>
    /// - On success, responds with [`SearchHoursOfOperationsOutput`](crate::operation::search_hours_of_operations::SearchHoursOfOperationsOutput) with field(s):
    ///   - [`hours_of_operations(Option<Vec<HoursOfOperation>>)`](crate::operation::search_hours_of_operations::SearchHoursOfOperationsOutput::hours_of_operations): <p>Information about the hours of operations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::search_hours_of_operations::SearchHoursOfOperationsOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    ///   - [`approximate_total_count(Option<i64>)`](crate::operation::search_hours_of_operations::SearchHoursOfOperationsOutput::approximate_total_count): <p>The total number of hours of operations which matched your search query.</p>
    /// - On failure, responds with [`SdkError<SearchHoursOfOperationsError>`](crate::operation::search_hours_of_operations::SearchHoursOfOperationsError)
    pub fn search_hours_of_operations(&self) -> crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder {
        crate::operation::search_hours_of_operations::builders::SearchHoursOfOperationsFluentBuilder::new(self.handle.clone())
    }
}
