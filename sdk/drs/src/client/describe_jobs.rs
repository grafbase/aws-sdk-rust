// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeJobs`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(DescribeJobsRequestFilters)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::filters) / [`set_filters(Option<DescribeJobsRequestFilters>)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::set_filters): <p>A set of filters by which to return Jobs.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::set_max_results): <p>Maximum number of Jobs to retrieve.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::set_next_token): <p>The token of the next Job to retrieve.</p>
    /// - On success, responds with [`DescribeJobsOutput`](crate::operation::describe_jobs::DescribeJobsOutput) with field(s):
    ///   - [`items(Option<Vec<Job>>)`](crate::operation::describe_jobs::DescribeJobsOutput::items): <p>An array of Jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_jobs::DescribeJobsOutput::next_token): <p>The token of the next Job to retrieve.</p>
    /// - On failure, responds with [`SdkError<DescribeJobsError>`](crate::operation::describe_jobs::DescribeJobsError)
    pub fn describe_jobs(&self) -> crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder {
        crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::new(self.handle.clone())
    }
}
