// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDefaultRetentionPolicyInput {
    /// <p>The organization ID.</p>
    pub organization_id: ::std::option::Option<::std::string::String>,
}
impl GetDefaultRetentionPolicyInput {
    /// <p>The organization ID.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
}
impl GetDefaultRetentionPolicyInput {
    /// Creates a new builder-style object to manufacture [`GetDefaultRetentionPolicyInput`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyInput).
    pub fn builder() -> crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyInputBuilder {
        crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyInputBuilder::default()
    }
}

/// A builder for [`GetDefaultRetentionPolicyInput`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetDefaultRetentionPolicyInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
}
impl GetDefaultRetentionPolicyInputBuilder {
    /// <p>The organization ID.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The organization ID.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.organization_id = input;
        self
    }
    /// <p>The organization ID.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.organization_id
    }
    /// Consumes the builder and constructs a [`GetDefaultRetentionPolicyInput`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyInput {
            organization_id: self.organization_id,
        })
    }
}
