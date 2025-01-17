// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartCodegenJob`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::set_app_id): <p>The unique ID for the Amplify app.</p>
    ///   - [`environment_name(impl ::std::convert::Into<String>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::environment_name) / [`set_environment_name(Option<String>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::set_environment_name): <p>The name of the backend environment that is a part of the Amplify app.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::set_client_token): <p>The idempotency token used to ensure that the code generation job request completes only once.</p>
    ///   - [`codegen_job_to_create(StartCodegenJobData)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::codegen_job_to_create) / [`set_codegen_job_to_create(Option<StartCodegenJobData>)`](crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::set_codegen_job_to_create): <p>The code generation job resource configuration.</p>
    /// - On success, responds with [`StartCodegenJobOutput`](crate::operation::start_codegen_job::StartCodegenJobOutput) with field(s):
    ///   - [`entity(Option<CodegenJob>)`](crate::operation::start_codegen_job::StartCodegenJobOutput::entity): <p>The code generation job for a UI component that is associated with an Amplify app.</p>
    /// - On failure, responds with [`SdkError<StartCodegenJobError>`](crate::operation::start_codegen_job::StartCodegenJobError)
    pub fn start_codegen_job(&self) -> crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder {
        crate::operation::start_codegen_job::builders::StartCodegenJobFluentBuilder::new(self.handle.clone())
    }
}
