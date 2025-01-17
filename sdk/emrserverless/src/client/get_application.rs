// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetApplication`](crate::operation::get_application::builders::GetApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::get_application::builders::GetApplicationFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::get_application::builders::GetApplicationFluentBuilder::set_application_id): <p>The ID of the application that will be described.</p>
    /// - On success, responds with [`GetApplicationOutput`](crate::operation::get_application::GetApplicationOutput) with field(s):
    ///   - [`application(Option<Application>)`](crate::operation::get_application::GetApplicationOutput::application): <p>The output displays information about the specified application.</p>
    /// - On failure, responds with [`SdkError<GetApplicationError>`](crate::operation::get_application::GetApplicationError)
    pub fn get_application(&self) -> crate::operation::get_application::builders::GetApplicationFluentBuilder {
        crate::operation::get_application::builders::GetApplicationFluentBuilder::new(self.handle.clone())
    }
}
