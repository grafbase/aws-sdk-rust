// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateUserDefinedFunction`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl ::std::convert::Into<String>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::set_catalog_id): <p>The ID of the Data Catalog where the function to be updated is located. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl ::std::convert::Into<String>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::set_database_name): <p>The name of the catalog database where the function to be updated is located.</p>
    ///   - [`function_name(impl ::std::convert::Into<String>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::set_function_name): <p>The name of the function.</p>
    ///   - [`function_input(UserDefinedFunctionInput)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::function_input) / [`set_function_input(Option<UserDefinedFunctionInput>)`](crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::set_function_input): <p>A <code>FunctionInput</code> object that redefines the function in the Data Catalog.</p>
    /// - On success, responds with [`UpdateUserDefinedFunctionOutput`](crate::operation::update_user_defined_function::UpdateUserDefinedFunctionOutput)
    /// - On failure, responds with [`SdkError<UpdateUserDefinedFunctionError>`](crate::operation::update_user_defined_function::UpdateUserDefinedFunctionError)
    pub fn update_user_defined_function(&self) -> crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder {
        crate::operation::update_user_defined_function::builders::UpdateUserDefinedFunctionFluentBuilder::new(self.handle.clone())
    }
}
