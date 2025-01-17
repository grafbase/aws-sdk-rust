// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetJobDocument`](crate::operation::get_job_document::builders::GetJobDocumentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl ::std::convert::Into<String>)`](crate::operation::get_job_document::builders::GetJobDocumentFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::get_job_document::builders::GetJobDocumentFluentBuilder::set_job_id): <p>The unique identifier you assigned to this job when it was created.</p>
    /// - On success, responds with [`GetJobDocumentOutput`](crate::operation::get_job_document::GetJobDocumentOutput) with field(s):
    ///   - [`document(Option<String>)`](crate::operation::get_job_document::GetJobDocumentOutput::document): <p>The job document content.</p>
    /// - On failure, responds with [`SdkError<GetJobDocumentError>`](crate::operation::get_job_document::GetJobDocumentError)
    pub fn get_job_document(&self) -> crate::operation::get_job_document::builders::GetJobDocumentFluentBuilder {
        crate::operation::get_job_document::builders::GetJobDocumentFluentBuilder::new(self.handle.clone())
    }
}
