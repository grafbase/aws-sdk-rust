// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateApprovalRuleTemplateNameInput {
    /// <p>The current name of the approval rule template.</p>
    pub old_approval_rule_template_name: ::std::option::Option<::std::string::String>,
    /// <p>The new name you want to apply to the approval rule template.</p>
    pub new_approval_rule_template_name: ::std::option::Option<::std::string::String>,
}
impl UpdateApprovalRuleTemplateNameInput {
    /// <p>The current name of the approval rule template.</p>
    pub fn old_approval_rule_template_name(&self) -> ::std::option::Option<&str> {
        self.old_approval_rule_template_name.as_deref()
    }
    /// <p>The new name you want to apply to the approval rule template.</p>
    pub fn new_approval_rule_template_name(&self) -> ::std::option::Option<&str> {
        self.new_approval_rule_template_name.as_deref()
    }
}
impl UpdateApprovalRuleTemplateNameInput {
    /// Creates a new builder-style object to manufacture [`UpdateApprovalRuleTemplateNameInput`](crate::operation::update_approval_rule_template_name::UpdateApprovalRuleTemplateNameInput).
    pub fn builder() -> crate::operation::update_approval_rule_template_name::builders::UpdateApprovalRuleTemplateNameInputBuilder {
        crate::operation::update_approval_rule_template_name::builders::UpdateApprovalRuleTemplateNameInputBuilder::default()
    }
}

/// A builder for [`UpdateApprovalRuleTemplateNameInput`](crate::operation::update_approval_rule_template_name::UpdateApprovalRuleTemplateNameInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateApprovalRuleTemplateNameInputBuilder {
    pub(crate) old_approval_rule_template_name: ::std::option::Option<::std::string::String>,
    pub(crate) new_approval_rule_template_name: ::std::option::Option<::std::string::String>,
}
impl UpdateApprovalRuleTemplateNameInputBuilder {
    /// <p>The current name of the approval rule template.</p>
    pub fn old_approval_rule_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.old_approval_rule_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The current name of the approval rule template.</p>
    pub fn set_old_approval_rule_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.old_approval_rule_template_name = input;
        self
    }
    /// <p>The current name of the approval rule template.</p>
    pub fn get_old_approval_rule_template_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.old_approval_rule_template_name
    }
    /// <p>The new name you want to apply to the approval rule template.</p>
    pub fn new_approval_rule_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.new_approval_rule_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new name you want to apply to the approval rule template.</p>
    pub fn set_new_approval_rule_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.new_approval_rule_template_name = input;
        self
    }
    /// <p>The new name you want to apply to the approval rule template.</p>
    pub fn get_new_approval_rule_template_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.new_approval_rule_template_name
    }
    /// Consumes the builder and constructs a [`UpdateApprovalRuleTemplateNameInput`](crate::operation::update_approval_rule_template_name::UpdateApprovalRuleTemplateNameInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_approval_rule_template_name::UpdateApprovalRuleTemplateNameInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_approval_rule_template_name::UpdateApprovalRuleTemplateNameInput {
                old_approval_rule_template_name: self.old_approval_rule_template_name,
                new_approval_rule_template_name: self.new_approval_rule_template_name,
            },
        )
    }
}
