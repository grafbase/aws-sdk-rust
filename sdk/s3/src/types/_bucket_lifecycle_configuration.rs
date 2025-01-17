// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object Lifecycle Management</a> in the <i>Amazon S3 User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BucketLifecycleConfiguration {
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub rules: ::std::option::Option<::std::vec::Vec<crate::types::LifecycleRule>>,
}
impl BucketLifecycleConfiguration {
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub fn rules(&self) -> ::std::option::Option<&[crate::types::LifecycleRule]> {
        self.rules.as_deref()
    }
}
impl BucketLifecycleConfiguration {
    /// Creates a new builder-style object to manufacture [`BucketLifecycleConfiguration`](crate::types::BucketLifecycleConfiguration).
    pub fn builder() -> crate::types::builders::BucketLifecycleConfigurationBuilder {
        crate::types::builders::BucketLifecycleConfigurationBuilder::default()
    }
}

/// A builder for [`BucketLifecycleConfiguration`](crate::types::BucketLifecycleConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BucketLifecycleConfigurationBuilder {
    pub(crate) rules: ::std::option::Option<::std::vec::Vec<crate::types::LifecycleRule>>,
}
impl BucketLifecycleConfigurationBuilder {
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub fn rules(mut self, input: crate::types::LifecycleRule) -> Self {
        let mut v = self.rules.unwrap_or_default();
        v.push(input);
        self.rules = ::std::option::Option::Some(v);
        self
    }
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub fn set_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LifecycleRule>>) -> Self {
        self.rules = input;
        self
    }
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
    pub fn get_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LifecycleRule>> {
        &self.rules
    }
    /// Consumes the builder and constructs a [`BucketLifecycleConfiguration`](crate::types::BucketLifecycleConfiguration).
    pub fn build(self) -> crate::types::BucketLifecycleConfiguration {
        crate::types::BucketLifecycleConfiguration { rules: self.rules }
    }
}
