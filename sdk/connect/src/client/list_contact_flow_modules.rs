// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListContactFlowModules`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::set_max_results): <p>The maximum number of results to return per page.</p>
    ///   - [`contact_flow_module_state(ContactFlowModuleState)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::contact_flow_module_state) / [`set_contact_flow_module_state(Option<ContactFlowModuleState>)`](crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::set_contact_flow_module_state): <p>The state of the flow module.</p>
    /// - On success, responds with [`ListContactFlowModulesOutput`](crate::operation::list_contact_flow_modules::ListContactFlowModulesOutput) with field(s):
    ///   - [`contact_flow_modules_summary_list(Option<Vec<ContactFlowModuleSummary>>)`](crate::operation::list_contact_flow_modules::ListContactFlowModulesOutput::contact_flow_modules_summary_list): <p>Information about the flow module.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_contact_flow_modules::ListContactFlowModulesOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListContactFlowModulesError>`](crate::operation::list_contact_flow_modules::ListContactFlowModulesError)
    pub fn list_contact_flow_modules(&self) -> crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder {
        crate::operation::list_contact_flow_modules::builders::ListContactFlowModulesFluentBuilder::new(self.handle.clone())
    }
}
