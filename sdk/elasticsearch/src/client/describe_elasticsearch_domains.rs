// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeElasticsearchDomains`](crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_names(Vec<String>)`](crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsFluentBuilder::domain_names) / [`set_domain_names(Option<Vec<String>>)`](crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsFluentBuilder::set_domain_names): <p>The Elasticsearch domains for which you want information.</p>
    /// - On success, responds with [`DescribeElasticsearchDomainsOutput`](crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput) with field(s):
    ///   - [`domain_status_list(Option<Vec<ElasticsearchDomainStatus>>)`](crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsOutput::domain_status_list): <p>The status of the domains requested in the <code>DescribeElasticsearchDomains</code> request.</p>
    /// - On failure, responds with [`SdkError<DescribeElasticsearchDomainsError>`](crate::operation::describe_elasticsearch_domains::DescribeElasticsearchDomainsError)
    pub fn describe_elasticsearch_domains(
        &self,
    ) -> crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsFluentBuilder {
        crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsFluentBuilder::new(self.handle.clone())
    }
}
