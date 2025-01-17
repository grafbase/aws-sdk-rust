// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListInstances`](crate::operation::list_instances::builders::ListInstancesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::set_max_results): <p>Maximum number of results to return in a single call.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::set_next_token): <p>Token for the next set of results.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::set_filters): <p>An array of structures that you can use to filter the results to those that match one or more sets of key-value pairs that you specify.</p>
    /// - On success, responds with [`ListInstancesOutput`](crate::operation::list_instances::ListInstancesOutput) with field(s):
    ///   - [`instance_summaries(Option<Vec<InstanceSummary>>)`](crate::operation::list_instances::ListInstancesOutput::instance_summaries): <p>Metadata that describes the list instances operation.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_instances::ListInstancesOutput::next_token): <p>Token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListInstancesError>`](crate::operation::list_instances::ListInstancesError)
    pub fn list_instances(&self) -> crate::operation::list_instances::builders::ListInstancesFluentBuilder {
        crate::operation::list_instances::builders::ListInstancesFluentBuilder::new(self.handle.clone())
    }
}
