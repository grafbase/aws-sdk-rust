// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSuiteDefinition`](crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`suite_definition_configuration(SuiteDefinitionConfiguration)`](crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder::suite_definition_configuration) / [`set_suite_definition_configuration(Option<SuiteDefinitionConfiguration>)`](crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder::set_suite_definition_configuration): <p>Creates a Device Advisor test suite with suite definition configuration.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder::set_tags): <p>The tags to be attached to the suite definition.</p>
    /// - On success, responds with [`CreateSuiteDefinitionOutput`](crate::operation::create_suite_definition::CreateSuiteDefinitionOutput) with field(s):
    ///   - [`suite_definition_id(Option<String>)`](crate::operation::create_suite_definition::CreateSuiteDefinitionOutput::suite_definition_id): <p>The UUID of the test suite created.</p>
    ///   - [`suite_definition_arn(Option<String>)`](crate::operation::create_suite_definition::CreateSuiteDefinitionOutput::suite_definition_arn): <p>The Amazon Resource Name (ARN) of the test suite.</p>
    ///   - [`suite_definition_name(Option<String>)`](crate::operation::create_suite_definition::CreateSuiteDefinitionOutput::suite_definition_name): <p>The suite definition name of the test suite. This is a required parameter.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::create_suite_definition::CreateSuiteDefinitionOutput::created_at): <p>The timestamp of when the test suite was created.</p>
    /// - On failure, responds with [`SdkError<CreateSuiteDefinitionError>`](crate::operation::create_suite_definition::CreateSuiteDefinitionError)
    pub fn create_suite_definition(&self) -> crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder {
        crate::operation::create_suite_definition::builders::CreateSuiteDefinitionFluentBuilder::new(self.handle.clone())
    }
}
