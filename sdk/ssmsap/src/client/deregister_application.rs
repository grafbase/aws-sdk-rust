// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterApplication`](crate::operation::deregister_application::builders::DeregisterApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::deregister_application::builders::DeregisterApplicationFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::deregister_application::builders::DeregisterApplicationFluentBuilder::set_application_id): <p>The ID of the application.</p>
    /// - On success, responds with [`DeregisterApplicationOutput`](crate::operation::deregister_application::DeregisterApplicationOutput)
    /// - On failure, responds with [`SdkError<DeregisterApplicationError>`](crate::operation::deregister_application::DeregisterApplicationError)
    pub fn deregister_application(&self) -> crate::operation::deregister_application::builders::DeregisterApplicationFluentBuilder {
        crate::operation::deregister_application::builders::DeregisterApplicationFluentBuilder::new(self.handle.clone())
    }
}
