// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTrafficMirrorFilters`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`traffic_mirror_filter_ids(Vec<String>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::traffic_mirror_filter_ids) / [`set_traffic_mirror_filter_ids(Option<Vec<String>>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::set_traffic_mirror_filter_ids): <p>The ID of the Traffic Mirror filter.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::set_filters): <p>One or more filters. The possible values are:</p>  <ul>   <li> <p> <code>description</code>: The Traffic Mirror filter description.</p> </li>   <li> <p> <code>traffic-mirror-filter-id</code>: The ID of the Traffic Mirror filter.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    /// - On success, responds with [`DescribeTrafficMirrorFiltersOutput`](crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersOutput) with field(s):
    ///   - [`traffic_mirror_filters(Option<Vec<TrafficMirrorFilter>>)`](crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersOutput::traffic_mirror_filters): <p>Information about one or more Traffic Mirror filters.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersOutput::next_token): <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeTrafficMirrorFiltersError>`](crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersError)
    pub fn describe_traffic_mirror_filters(
        &self,
    ) -> crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder {
        crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersFluentBuilder::new(self.handle.clone())
    }
}
