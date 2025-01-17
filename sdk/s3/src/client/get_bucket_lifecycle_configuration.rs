// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketLifecycleConfiguration`](crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder::set_bucket): <p>The name of the bucket for which to get the lifecycle information.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketLifecycleConfigurationOutput`](crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationOutput) with field(s):
    ///   - [`rules(Option<Vec<LifecycleRule>>)`](crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationOutput::rules): <p>Container for a lifecycle rule.</p>
    /// - On failure, responds with [`SdkError<GetBucketLifecycleConfigurationError>`](crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationError)
    pub fn get_bucket_lifecycle_configuration(
        &self,
    ) -> crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder {
        crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationFluentBuilder::new(self.handle.clone())
    }
}
