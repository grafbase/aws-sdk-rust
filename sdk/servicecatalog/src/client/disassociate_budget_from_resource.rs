// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateBudgetFromResource`](crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`budget_name(impl ::std::convert::Into<String>)`](crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder::budget_name) / [`set_budget_name(Option<String>)`](crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder::set_budget_name): <p>The name of the budget you want to disassociate.</p>
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder::set_resource_id): <p>The resource identifier you want to disassociate from. Either a portfolio-id or a product-id.</p>
    /// - On success, responds with [`DisassociateBudgetFromResourceOutput`](crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceOutput)
    /// - On failure, responds with [`SdkError<DisassociateBudgetFromResourceError>`](crate::operation::disassociate_budget_from_resource::DisassociateBudgetFromResourceError)
    pub fn disassociate_budget_from_resource(
        &self,
    ) -> crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder {
        crate::operation::disassociate_budget_from_resource::builders::DisassociateBudgetFromResourceFluentBuilder::new(self.handle.clone())
    }
}
