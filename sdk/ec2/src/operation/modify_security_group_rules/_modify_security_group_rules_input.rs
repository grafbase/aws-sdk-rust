// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifySecurityGroupRulesInput {
    /// <p>The ID of the security group.</p>
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>Information about the security group properties to update.</p>
    pub security_group_rules: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupRuleUpdate>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl ModifySecurityGroupRulesInput {
    /// <p>The ID of the security group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>Information about the security group properties to update.</p>
    pub fn security_group_rules(&self) -> ::std::option::Option<&[crate::types::SecurityGroupRuleUpdate]> {
        self.security_group_rules.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl ModifySecurityGroupRulesInput {
    /// Creates a new builder-style object to manufacture [`ModifySecurityGroupRulesInput`](crate::operation::modify_security_group_rules::ModifySecurityGroupRulesInput).
    pub fn builder() -> crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesInputBuilder {
        crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesInputBuilder::default()
    }
}

/// A builder for [`ModifySecurityGroupRulesInput`](crate::operation::modify_security_group_rules::ModifySecurityGroupRulesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifySecurityGroupRulesInputBuilder {
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) security_group_rules: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupRuleUpdate>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl ModifySecurityGroupRulesInputBuilder {
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_id
    }
    /// Appends an item to `security_group_rules`.
    ///
    /// To override the contents of this collection use [`set_security_group_rules`](Self::set_security_group_rules).
    ///
    /// <p>Information about the security group properties to update.</p>
    pub fn security_group_rules(mut self, input: crate::types::SecurityGroupRuleUpdate) -> Self {
        let mut v = self.security_group_rules.unwrap_or_default();
        v.push(input);
        self.security_group_rules = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the security group properties to update.</p>
    pub fn set_security_group_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupRuleUpdate>>) -> Self {
        self.security_group_rules = input;
        self
    }
    /// <p>Information about the security group properties to update.</p>
    pub fn get_security_group_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupRuleUpdate>> {
        &self.security_group_rules
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`ModifySecurityGroupRulesInput`](crate::operation::modify_security_group_rules::ModifySecurityGroupRulesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_security_group_rules::ModifySecurityGroupRulesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::modify_security_group_rules::ModifySecurityGroupRulesInput {
            group_id: self.group_id,
            security_group_rules: self.security_group_rules,
            dry_run: self.dry_run,
        })
    }
}
