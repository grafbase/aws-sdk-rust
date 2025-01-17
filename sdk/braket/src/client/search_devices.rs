// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SearchDevices`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::set_next_token): <p>A token used for pagination of results returned in the response. Use the token returned from the previous request continue results where the previous request ended.</p>
    ///   - [`max_results(i32)`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`filters(Vec<SearchDevicesFilter>)`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::filters) / [`set_filters(Option<Vec<SearchDevicesFilter>>)`](crate::operation::search_devices::builders::SearchDevicesFluentBuilder::set_filters): <p>The filter values to use to search for a device.</p>
    /// - On success, responds with [`SearchDevicesOutput`](crate::operation::search_devices::SearchDevicesOutput) with field(s):
    ///   - [`devices(Option<Vec<DeviceSummary>>)`](crate::operation::search_devices::SearchDevicesOutput::devices): <p>An array of <code>DeviceSummary</code> objects for devices that match the specified filter values.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::search_devices::SearchDevicesOutput::next_token): <p>A token used for pagination of results, or null if there are no additional results. Use the token value in a subsequent request to continue results where the previous request ended.</p>
    /// - On failure, responds with [`SdkError<SearchDevicesError>`](crate::operation::search_devices::SearchDevicesError)
    pub fn search_devices(&self) -> crate::operation::search_devices::builders::SearchDevicesFluentBuilder {
        crate::operation::search_devices::builders::SearchDevicesFluentBuilder::new(self.handle.clone())
    }
}
