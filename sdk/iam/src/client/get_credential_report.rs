// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCredentialReport`](crate::operation::get_credential_report::builders::GetCredentialReportFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_credential_report::builders::GetCredentialReportFluentBuilder::send) it.
    /// - On success, responds with [`GetCredentialReportOutput`](crate::operation::get_credential_report::GetCredentialReportOutput) with field(s):
    ///   - [`content(Option<Blob>)`](crate::operation::get_credential_report::GetCredentialReportOutput::content): <p>Contains the credential report. The report is Base64-encoded.</p>
    ///   - [`report_format(Option<ReportFormatType>)`](crate::operation::get_credential_report::GetCredentialReportOutput::report_format): <p>The format (MIME type) of the credential report.</p>
    ///   - [`generated_time(Option<DateTime>)`](crate::operation::get_credential_report::GetCredentialReportOutput::generated_time): <p> The date and time when the credential report was created, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>.</p>
    /// - On failure, responds with [`SdkError<GetCredentialReportError>`](crate::operation::get_credential_report::GetCredentialReportError)
    pub fn get_credential_report(&self) -> crate::operation::get_credential_report::builders::GetCredentialReportFluentBuilder {
        crate::operation::get_credential_report::builders::GetCredentialReportFluentBuilder::new(self.handle.clone())
    }
}
