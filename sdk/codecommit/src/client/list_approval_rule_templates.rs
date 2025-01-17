// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListApprovalRuleTemplates`](crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder::set_next_token): <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder::set_max_results): <p>A non-zero, non-negative integer used to limit the number of returned results.</p>
    /// - On success, responds with [`ListApprovalRuleTemplatesOutput`](crate::operation::list_approval_rule_templates::ListApprovalRuleTemplatesOutput) with field(s):
    ///   - [`approval_rule_template_names(Option<Vec<String>>)`](crate::operation::list_approval_rule_templates::ListApprovalRuleTemplatesOutput::approval_rule_template_names): <p>The names of all the approval rule templates found in the AWS Region for your AWS account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_approval_rule_templates::ListApprovalRuleTemplatesOutput::next_token): <p>An enumeration token that allows the operation to batch the next results of the operation.</p>
    /// - On failure, responds with [`SdkError<ListApprovalRuleTemplatesError>`](crate::operation::list_approval_rule_templates::ListApprovalRuleTemplatesError)
    pub fn list_approval_rule_templates(&self) -> crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder {
        crate::operation::list_approval_rule_templates::builders::ListApprovalRuleTemplatesFluentBuilder::new(self.handle.clone())
    }
}
