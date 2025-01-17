// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetWorkflowSteps`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::set_domain_name): <p>The unique name of the domain.</p>
    ///   - [`workflow_id(impl ::std::convert::Into<String>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::workflow_id) / [`set_workflow_id(Option<String>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::set_workflow_id): <p>Unique identifier for the workflow.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::set_max_results): <p>The maximum number of results to return per page.</p>
    /// - On success, responds with [`GetWorkflowStepsOutput`](crate::operation::get_workflow_steps::GetWorkflowStepsOutput) with field(s):
    ///   - [`workflow_id(Option<String>)`](crate::operation::get_workflow_steps::GetWorkflowStepsOutput::workflow_id): <p>Unique identifier for the workflow.</p>
    ///   - [`workflow_type(Option<WorkflowType>)`](crate::operation::get_workflow_steps::GetWorkflowStepsOutput::workflow_type): <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    ///   - [`items(Option<Vec<WorkflowStepItem>>)`](crate::operation::get_workflow_steps::GetWorkflowStepsOutput::items): <p>List containing workflow step details.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_workflow_steps::GetWorkflowStepsOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<GetWorkflowStepsError>`](crate::operation::get_workflow_steps::GetWorkflowStepsError)
    pub fn get_workflow_steps(&self) -> crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder {
        crate::operation::get_workflow_steps::builders::GetWorkflowStepsFluentBuilder::new(self.handle.clone())
    }
}
