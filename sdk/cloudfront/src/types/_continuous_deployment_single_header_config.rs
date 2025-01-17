// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This configuration determines which HTTP requests are sent to the staging distribution. If the HTTP request contains a header and value that matches what you specify here, the request is sent to the staging distribution. Otherwise the request is sent to the primary distribution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContinuousDeploymentSingleHeaderConfig {
    /// <p>The request header name that you want CloudFront to send to your staging distribution. The header must contain the prefix <code>aws-cf-cd-</code>.</p>
    pub header: ::std::option::Option<::std::string::String>,
    /// <p>The request header value.</p>
    pub value: ::std::option::Option<::std::string::String>,
}
impl ContinuousDeploymentSingleHeaderConfig {
    /// <p>The request header name that you want CloudFront to send to your staging distribution. The header must contain the prefix <code>aws-cf-cd-</code>.</p>
    pub fn header(&self) -> ::std::option::Option<&str> {
        self.header.as_deref()
    }
    /// <p>The request header value.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl ContinuousDeploymentSingleHeaderConfig {
    /// Creates a new builder-style object to manufacture [`ContinuousDeploymentSingleHeaderConfig`](crate::types::ContinuousDeploymentSingleHeaderConfig).
    pub fn builder() -> crate::types::builders::ContinuousDeploymentSingleHeaderConfigBuilder {
        crate::types::builders::ContinuousDeploymentSingleHeaderConfigBuilder::default()
    }
}

/// A builder for [`ContinuousDeploymentSingleHeaderConfig`](crate::types::ContinuousDeploymentSingleHeaderConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ContinuousDeploymentSingleHeaderConfigBuilder {
    pub(crate) header: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl ContinuousDeploymentSingleHeaderConfigBuilder {
    /// <p>The request header name that you want CloudFront to send to your staging distribution. The header must contain the prefix <code>aws-cf-cd-</code>.</p>
    pub fn header(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.header = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The request header name that you want CloudFront to send to your staging distribution. The header must contain the prefix <code>aws-cf-cd-</code>.</p>
    pub fn set_header(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.header = input;
        self
    }
    /// <p>The request header name that you want CloudFront to send to your staging distribution. The header must contain the prefix <code>aws-cf-cd-</code>.</p>
    pub fn get_header(&self) -> &::std::option::Option<::std::string::String> {
        &self.header
    }
    /// <p>The request header value.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The request header value.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The request header value.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// Consumes the builder and constructs a [`ContinuousDeploymentSingleHeaderConfig`](crate::types::ContinuousDeploymentSingleHeaderConfig).
    pub fn build(self) -> crate::types::ContinuousDeploymentSingleHeaderConfig {
        crate::types::ContinuousDeploymentSingleHeaderConfig {
            header: self.header,
            value: self.value,
        }
    }
}
