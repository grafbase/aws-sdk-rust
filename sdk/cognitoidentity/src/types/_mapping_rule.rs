// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A rule that maps a claim name, a claim value, and a match type to a role ARN.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MappingRule {
    /// <p>The claim name that must be present in the token, for example, "isAdmin" or "paid".</p>
    pub claim: ::std::option::Option<::std::string::String>,
    /// <p>The match condition that specifies how closely the claim value in the IdP token must match <code>Value</code>.</p>
    pub match_type: ::std::option::Option<crate::types::MappingRuleMatchType>,
    /// <p>A brief string that the claim must match, for example, "paid" or "yes".</p>
    pub value: ::std::option::Option<::std::string::String>,
    /// <p>The role ARN.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
}
impl MappingRule {
    /// <p>The claim name that must be present in the token, for example, "isAdmin" or "paid".</p>
    pub fn claim(&self) -> ::std::option::Option<&str> {
        self.claim.as_deref()
    }
    /// <p>The match condition that specifies how closely the claim value in the IdP token must match <code>Value</code>.</p>
    pub fn match_type(&self) -> ::std::option::Option<&crate::types::MappingRuleMatchType> {
        self.match_type.as_ref()
    }
    /// <p>A brief string that the claim must match, for example, "paid" or "yes".</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
    /// <p>The role ARN.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
}
impl MappingRule {
    /// Creates a new builder-style object to manufacture [`MappingRule`](crate::types::MappingRule).
    pub fn builder() -> crate::types::builders::MappingRuleBuilder {
        crate::types::builders::MappingRuleBuilder::default()
    }
}

/// A builder for [`MappingRule`](crate::types::MappingRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MappingRuleBuilder {
    pub(crate) claim: ::std::option::Option<::std::string::String>,
    pub(crate) match_type: ::std::option::Option<crate::types::MappingRuleMatchType>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
}
impl MappingRuleBuilder {
    /// <p>The claim name that must be present in the token, for example, "isAdmin" or "paid".</p>
    pub fn claim(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.claim = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The claim name that must be present in the token, for example, "isAdmin" or "paid".</p>
    pub fn set_claim(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.claim = input;
        self
    }
    /// <p>The claim name that must be present in the token, for example, "isAdmin" or "paid".</p>
    pub fn get_claim(&self) -> &::std::option::Option<::std::string::String> {
        &self.claim
    }
    /// <p>The match condition that specifies how closely the claim value in the IdP token must match <code>Value</code>.</p>
    pub fn match_type(mut self, input: crate::types::MappingRuleMatchType) -> Self {
        self.match_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The match condition that specifies how closely the claim value in the IdP token must match <code>Value</code>.</p>
    pub fn set_match_type(mut self, input: ::std::option::Option<crate::types::MappingRuleMatchType>) -> Self {
        self.match_type = input;
        self
    }
    /// <p>The match condition that specifies how closely the claim value in the IdP token must match <code>Value</code>.</p>
    pub fn get_match_type(&self) -> &::std::option::Option<crate::types::MappingRuleMatchType> {
        &self.match_type
    }
    /// <p>A brief string that the claim must match, for example, "paid" or "yes".</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A brief string that the claim must match, for example, "paid" or "yes".</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>A brief string that the claim must match, for example, "paid" or "yes".</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// <p>The role ARN.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The role ARN.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The role ARN.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// Consumes the builder and constructs a [`MappingRule`](crate::types::MappingRule).
    pub fn build(self) -> crate::types::MappingRule {
        crate::types::MappingRule {
            claim: self.claim,
            match_type: self.match_type,
            value: self.value,
            role_arn: self.role_arn,
        }
    }
}
