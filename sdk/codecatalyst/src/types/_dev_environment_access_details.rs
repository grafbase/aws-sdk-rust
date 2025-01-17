// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about connection details for a Dev Environment.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DevEnvironmentAccessDetails {
    /// <p>The URL used to send commands to and from the Dev Environment.</p>
    pub stream_url: ::std::option::Option<::std::string::String>,
    /// <p>An encrypted token value that contains session and caller information used to authenticate the connection.</p>
    pub token_value: ::std::option::Option<::std::string::String>,
}
impl DevEnvironmentAccessDetails {
    /// <p>The URL used to send commands to and from the Dev Environment.</p>
    pub fn stream_url(&self) -> ::std::option::Option<&str> {
        self.stream_url.as_deref()
    }
    /// <p>An encrypted token value that contains session and caller information used to authenticate the connection.</p>
    pub fn token_value(&self) -> ::std::option::Option<&str> {
        self.token_value.as_deref()
    }
}
impl ::std::fmt::Debug for DevEnvironmentAccessDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DevEnvironmentAccessDetails");
        formatter.field("stream_url", &"*** Sensitive Data Redacted ***");
        formatter.field("token_value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl DevEnvironmentAccessDetails {
    /// Creates a new builder-style object to manufacture [`DevEnvironmentAccessDetails`](crate::types::DevEnvironmentAccessDetails).
    pub fn builder() -> crate::types::builders::DevEnvironmentAccessDetailsBuilder {
        crate::types::builders::DevEnvironmentAccessDetailsBuilder::default()
    }
}

/// A builder for [`DevEnvironmentAccessDetails`](crate::types::DevEnvironmentAccessDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DevEnvironmentAccessDetailsBuilder {
    pub(crate) stream_url: ::std::option::Option<::std::string::String>,
    pub(crate) token_value: ::std::option::Option<::std::string::String>,
}
impl DevEnvironmentAccessDetailsBuilder {
    /// <p>The URL used to send commands to and from the Dev Environment.</p>
    pub fn stream_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL used to send commands to and from the Dev Environment.</p>
    pub fn set_stream_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_url = input;
        self
    }
    /// <p>The URL used to send commands to and from the Dev Environment.</p>
    pub fn get_stream_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_url
    }
    /// <p>An encrypted token value that contains session and caller information used to authenticate the connection.</p>
    pub fn token_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.token_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An encrypted token value that contains session and caller information used to authenticate the connection.</p>
    pub fn set_token_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.token_value = input;
        self
    }
    /// <p>An encrypted token value that contains session and caller information used to authenticate the connection.</p>
    pub fn get_token_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.token_value
    }
    /// Consumes the builder and constructs a [`DevEnvironmentAccessDetails`](crate::types::DevEnvironmentAccessDetails).
    pub fn build(self) -> crate::types::DevEnvironmentAccessDetails {
        crate::types::DevEnvironmentAccessDetails {
            stream_url: self.stream_url,
            token_value: self.token_value,
        }
    }
}
impl ::std::fmt::Debug for DevEnvironmentAccessDetailsBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DevEnvironmentAccessDetailsBuilder");
        formatter.field("stream_url", &"*** Sensitive Data Redacted ***");
        formatter.field("token_value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
