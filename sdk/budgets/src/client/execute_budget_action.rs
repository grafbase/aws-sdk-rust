// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExecuteBudgetAction`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::set_account_id): <p>The account ID of the user. It's a 12-digit number.</p>
    ///   - [`budget_name(impl ::std::convert::Into<String>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::budget_name) / [`set_budget_name(Option<String>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::set_budget_name): <p> A string that represents the budget name. The ":" and "\" characters aren't allowed.</p>
    ///   - [`action_id(impl ::std::convert::Into<String>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::action_id) / [`set_action_id(Option<String>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::set_action_id): <p> A system-generated universally unique identifier (UUID) for the action. </p>
    ///   - [`execution_type(ExecutionType)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::execution_type) / [`set_execution_type(Option<ExecutionType>)`](crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::set_execution_type): <p> The type of execution. </p>
    /// - On success, responds with [`ExecuteBudgetActionOutput`](crate::operation::execute_budget_action::ExecuteBudgetActionOutput) with field(s):
    ///   - [`account_id(Option<String>)`](crate::operation::execute_budget_action::ExecuteBudgetActionOutput::account_id): <p>The account ID of the user. It's a 12-digit number.</p>
    ///   - [`budget_name(Option<String>)`](crate::operation::execute_budget_action::ExecuteBudgetActionOutput::budget_name): <p> A string that represents the budget name. The ":" and "\" characters aren't allowed.</p>
    ///   - [`action_id(Option<String>)`](crate::operation::execute_budget_action::ExecuteBudgetActionOutput::action_id): <p> A system-generated universally unique identifier (UUID) for the action. </p>
    ///   - [`execution_type(Option<ExecutionType>)`](crate::operation::execute_budget_action::ExecuteBudgetActionOutput::execution_type): <p> The type of execution. </p>
    /// - On failure, responds with [`SdkError<ExecuteBudgetActionError>`](crate::operation::execute_budget_action::ExecuteBudgetActionError)
    pub fn execute_budget_action(&self) -> crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder {
        crate::operation::execute_budget_action::builders::ExecuteBudgetActionFluentBuilder::new(self.handle.clone())
    }
}
