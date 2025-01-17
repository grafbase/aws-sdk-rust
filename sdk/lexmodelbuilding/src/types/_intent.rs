// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Identifies the specific version of an intent.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Intent {
    /// <p>The name of the intent.</p>
    pub intent_name: ::std::option::Option<::std::string::String>,
    /// <p>The version of the intent.</p>
    pub intent_version: ::std::option::Option<::std::string::String>,
}
impl Intent {
    /// <p>The name of the intent.</p>
    pub fn intent_name(&self) -> ::std::option::Option<&str> {
        self.intent_name.as_deref()
    }
    /// <p>The version of the intent.</p>
    pub fn intent_version(&self) -> ::std::option::Option<&str> {
        self.intent_version.as_deref()
    }
}
impl Intent {
    /// Creates a new builder-style object to manufacture [`Intent`](crate::types::Intent).
    pub fn builder() -> crate::types::builders::IntentBuilder {
        crate::types::builders::IntentBuilder::default()
    }
}

/// A builder for [`Intent`](crate::types::Intent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IntentBuilder {
    pub(crate) intent_name: ::std::option::Option<::std::string::String>,
    pub(crate) intent_version: ::std::option::Option<::std::string::String>,
}
impl IntentBuilder {
    /// <p>The name of the intent.</p>
    pub fn intent_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.intent_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the intent.</p>
    pub fn set_intent_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.intent_name = input;
        self
    }
    /// <p>The name of the intent.</p>
    pub fn get_intent_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.intent_name
    }
    /// <p>The version of the intent.</p>
    pub fn intent_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.intent_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the intent.</p>
    pub fn set_intent_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.intent_version = input;
        self
    }
    /// <p>The version of the intent.</p>
    pub fn get_intent_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.intent_version
    }
    /// Consumes the builder and constructs a [`Intent`](crate::types::Intent).
    pub fn build(self) -> crate::types::Intent {
        crate::types::Intent {
            intent_name: self.intent_name,
            intent_version: self.intent_version,
        }
    }
}
