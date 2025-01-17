// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration for read only disk cache associated with a cluster.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KxCacheStorageConfiguration {
    /// <p>The type of cache storage . The valid values are: </p>
    /// <ul>
    /// <li> <p>CACHE_1000 – This type provides at least 1000 MB/s disk access throughput. </p> </li>
    /// </ul>
    pub r#type: ::std::option::Option<::std::string::String>,
    /// <p>The size of cache in Gigabytes.</p>
    pub size: ::std::option::Option<i32>,
}
impl KxCacheStorageConfiguration {
    /// <p>The type of cache storage . The valid values are: </p>
    /// <ul>
    /// <li> <p>CACHE_1000 – This type provides at least 1000 MB/s disk access throughput. </p> </li>
    /// </ul>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>The size of cache in Gigabytes.</p>
    pub fn size(&self) -> ::std::option::Option<i32> {
        self.size
    }
}
impl KxCacheStorageConfiguration {
    /// Creates a new builder-style object to manufacture [`KxCacheStorageConfiguration`](crate::types::KxCacheStorageConfiguration).
    pub fn builder() -> crate::types::builders::KxCacheStorageConfigurationBuilder {
        crate::types::builders::KxCacheStorageConfigurationBuilder::default()
    }
}

/// A builder for [`KxCacheStorageConfiguration`](crate::types::KxCacheStorageConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct KxCacheStorageConfigurationBuilder {
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
    pub(crate) size: ::std::option::Option<i32>,
}
impl KxCacheStorageConfigurationBuilder {
    /// <p>The type of cache storage . The valid values are: </p>
    /// <ul>
    /// <li> <p>CACHE_1000 – This type provides at least 1000 MB/s disk access throughput. </p> </li>
    /// </ul>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of cache storage . The valid values are: </p>
    /// <ul>
    /// <li> <p>CACHE_1000 – This type provides at least 1000 MB/s disk access throughput. </p> </li>
    /// </ul>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of cache storage . The valid values are: </p>
    /// <ul>
    /// <li> <p>CACHE_1000 – This type provides at least 1000 MB/s disk access throughput. </p> </li>
    /// </ul>
    pub fn get_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.r#type
    }
    /// <p>The size of cache in Gigabytes.</p>
    pub fn size(mut self, input: i32) -> Self {
        self.size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The size of cache in Gigabytes.</p>
    pub fn set_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.size = input;
        self
    }
    /// <p>The size of cache in Gigabytes.</p>
    pub fn get_size(&self) -> &::std::option::Option<i32> {
        &self.size
    }
    /// Consumes the builder and constructs a [`KxCacheStorageConfiguration`](crate::types::KxCacheStorageConfiguration).
    pub fn build(self) -> crate::types::KxCacheStorageConfiguration {
        crate::types::KxCacheStorageConfiguration {
            r#type: self.r#type,
            size: self.size,
        }
    }
}
