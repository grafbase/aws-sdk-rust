// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the cross-origin resource sharing (CORS) configuration for the API. CORS is only supported for HTTP APIs.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsCorsConfiguration {
    /// <p>The allowed origins for CORS requests.</p>
    pub allow_origins: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether the CORS request includes credentials.</p>
    pub allow_credentials: bool,
    /// <p>The exposed headers for CORS requests.</p>
    pub expose_headers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The number of seconds for which the browser caches preflight request results.</p>
    pub max_age: i32,
    /// <p>The allowed methods for CORS requests.</p>
    pub allow_methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The allowed headers for CORS requests.</p>
    pub allow_headers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AwsCorsConfiguration {
    /// <p>The allowed origins for CORS requests.</p>
    pub fn allow_origins(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.allow_origins.as_deref()
    }
    /// <p>Indicates whether the CORS request includes credentials.</p>
    pub fn allow_credentials(&self) -> bool {
        self.allow_credentials
    }
    /// <p>The exposed headers for CORS requests.</p>
    pub fn expose_headers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.expose_headers.as_deref()
    }
    /// <p>The number of seconds for which the browser caches preflight request results.</p>
    pub fn max_age(&self) -> i32 {
        self.max_age
    }
    /// <p>The allowed methods for CORS requests.</p>
    pub fn allow_methods(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.allow_methods.as_deref()
    }
    /// <p>The allowed headers for CORS requests.</p>
    pub fn allow_headers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.allow_headers.as_deref()
    }
}
impl AwsCorsConfiguration {
    /// Creates a new builder-style object to manufacture [`AwsCorsConfiguration`](crate::types::AwsCorsConfiguration).
    pub fn builder() -> crate::types::builders::AwsCorsConfigurationBuilder {
        crate::types::builders::AwsCorsConfigurationBuilder::default()
    }
}

/// A builder for [`AwsCorsConfiguration`](crate::types::AwsCorsConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AwsCorsConfigurationBuilder {
    pub(crate) allow_origins: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) allow_credentials: ::std::option::Option<bool>,
    pub(crate) expose_headers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) max_age: ::std::option::Option<i32>,
    pub(crate) allow_methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) allow_headers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AwsCorsConfigurationBuilder {
    /// Appends an item to `allow_origins`.
    ///
    /// To override the contents of this collection use [`set_allow_origins`](Self::set_allow_origins).
    ///
    /// <p>The allowed origins for CORS requests.</p>
    pub fn allow_origins(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.allow_origins.unwrap_or_default();
        v.push(input.into());
        self.allow_origins = ::std::option::Option::Some(v);
        self
    }
    /// <p>The allowed origins for CORS requests.</p>
    pub fn set_allow_origins(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.allow_origins = input;
        self
    }
    /// <p>The allowed origins for CORS requests.</p>
    pub fn get_allow_origins(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.allow_origins
    }
    /// <p>Indicates whether the CORS request includes credentials.</p>
    pub fn allow_credentials(mut self, input: bool) -> Self {
        self.allow_credentials = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the CORS request includes credentials.</p>
    pub fn set_allow_credentials(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_credentials = input;
        self
    }
    /// <p>Indicates whether the CORS request includes credentials.</p>
    pub fn get_allow_credentials(&self) -> &::std::option::Option<bool> {
        &self.allow_credentials
    }
    /// Appends an item to `expose_headers`.
    ///
    /// To override the contents of this collection use [`set_expose_headers`](Self::set_expose_headers).
    ///
    /// <p>The exposed headers for CORS requests.</p>
    pub fn expose_headers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.expose_headers.unwrap_or_default();
        v.push(input.into());
        self.expose_headers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The exposed headers for CORS requests.</p>
    pub fn set_expose_headers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.expose_headers = input;
        self
    }
    /// <p>The exposed headers for CORS requests.</p>
    pub fn get_expose_headers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.expose_headers
    }
    /// <p>The number of seconds for which the browser caches preflight request results.</p>
    pub fn max_age(mut self, input: i32) -> Self {
        self.max_age = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of seconds for which the browser caches preflight request results.</p>
    pub fn set_max_age(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_age = input;
        self
    }
    /// <p>The number of seconds for which the browser caches preflight request results.</p>
    pub fn get_max_age(&self) -> &::std::option::Option<i32> {
        &self.max_age
    }
    /// Appends an item to `allow_methods`.
    ///
    /// To override the contents of this collection use [`set_allow_methods`](Self::set_allow_methods).
    ///
    /// <p>The allowed methods for CORS requests.</p>
    pub fn allow_methods(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.allow_methods.unwrap_or_default();
        v.push(input.into());
        self.allow_methods = ::std::option::Option::Some(v);
        self
    }
    /// <p>The allowed methods for CORS requests.</p>
    pub fn set_allow_methods(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.allow_methods = input;
        self
    }
    /// <p>The allowed methods for CORS requests.</p>
    pub fn get_allow_methods(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.allow_methods
    }
    /// Appends an item to `allow_headers`.
    ///
    /// To override the contents of this collection use [`set_allow_headers`](Self::set_allow_headers).
    ///
    /// <p>The allowed headers for CORS requests.</p>
    pub fn allow_headers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.allow_headers.unwrap_or_default();
        v.push(input.into());
        self.allow_headers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The allowed headers for CORS requests.</p>
    pub fn set_allow_headers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.allow_headers = input;
        self
    }
    /// <p>The allowed headers for CORS requests.</p>
    pub fn get_allow_headers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.allow_headers
    }
    /// Consumes the builder and constructs a [`AwsCorsConfiguration`](crate::types::AwsCorsConfiguration).
    pub fn build(self) -> crate::types::AwsCorsConfiguration {
        crate::types::AwsCorsConfiguration {
            allow_origins: self.allow_origins,
            allow_credentials: self.allow_credentials.unwrap_or_default(),
            expose_headers: self.expose_headers,
            max_age: self.max_age.unwrap_or_default(),
            allow_methods: self.allow_methods,
            allow_headers: self.allow_headers,
        }
    }
}
