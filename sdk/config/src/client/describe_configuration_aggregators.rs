// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeConfigurationAggregators`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_aggregator_names(Vec<String>)`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::configuration_aggregator_names) / [`set_configuration_aggregator_names(Option<Vec<String>>)`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::set_configuration_aggregator_names): <p>The name of the configuration aggregators.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    ///   - [`limit(i32)`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::set_limit): <p>The maximum number of configuration aggregators returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    /// - On success, responds with [`DescribeConfigurationAggregatorsOutput`](crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsOutput) with field(s):
    ///   - [`configuration_aggregators(Option<Vec<ConfigurationAggregator>>)`](crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsOutput::configuration_aggregators): <p>Returns a ConfigurationAggregators object.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsOutput::next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<DescribeConfigurationAggregatorsError>`](crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsError)
    pub fn describe_configuration_aggregators(
        &self,
    ) -> crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder {
        crate::operation::describe_configuration_aggregators::builders::DescribeConfigurationAggregatorsFluentBuilder::new(self.handle.clone())
    }
}
