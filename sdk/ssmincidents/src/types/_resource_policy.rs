// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The resource policy that allows Incident Manager to perform actions on resources on your behalf.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourcePolicy {
    /// <p>The JSON blob that describes the policy.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the resource policy.</p>
    pub policy_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region that policy allows resources to be used in.</p>
    pub ram_resource_share_region: ::std::option::Option<::std::string::String>,
}
impl ResourcePolicy {
    /// <p>The JSON blob that describes the policy.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>The ID of the resource policy.</p>
    pub fn policy_id(&self) -> ::std::option::Option<&str> {
        self.policy_id.as_deref()
    }
    /// <p>The Amazon Web Services Region that policy allows resources to be used in.</p>
    pub fn ram_resource_share_region(&self) -> ::std::option::Option<&str> {
        self.ram_resource_share_region.as_deref()
    }
}
impl ResourcePolicy {
    /// Creates a new builder-style object to manufacture [`ResourcePolicy`](crate::types::ResourcePolicy).
    pub fn builder() -> crate::types::builders::ResourcePolicyBuilder {
        crate::types::builders::ResourcePolicyBuilder::default()
    }
}

/// A builder for [`ResourcePolicy`](crate::types::ResourcePolicy).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ResourcePolicyBuilder {
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) policy_id: ::std::option::Option<::std::string::String>,
    pub(crate) ram_resource_share_region: ::std::option::Option<::std::string::String>,
}
impl ResourcePolicyBuilder {
    /// <p>The JSON blob that describes the policy.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The JSON blob that describes the policy.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>The JSON blob that describes the policy.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// <p>The ID of the resource policy.</p>
    pub fn policy_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource policy.</p>
    pub fn set_policy_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_id = input;
        self
    }
    /// <p>The ID of the resource policy.</p>
    pub fn get_policy_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_id
    }
    /// <p>The Amazon Web Services Region that policy allows resources to be used in.</p>
    pub fn ram_resource_share_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ram_resource_share_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region that policy allows resources to be used in.</p>
    pub fn set_ram_resource_share_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ram_resource_share_region = input;
        self
    }
    /// <p>The Amazon Web Services Region that policy allows resources to be used in.</p>
    pub fn get_ram_resource_share_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.ram_resource_share_region
    }
    /// Consumes the builder and constructs a [`ResourcePolicy`](crate::types::ResourcePolicy).
    pub fn build(self) -> crate::types::ResourcePolicy {
        crate::types::ResourcePolicy {
            policy_document: self.policy_document,
            policy_id: self.policy_id,
            ram_resource_share_region: self.ram_resource_share_region,
        }
    }
}
