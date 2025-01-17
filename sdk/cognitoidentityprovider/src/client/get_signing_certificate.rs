// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSigningCertificate`](crate::operation::get_signing_certificate::builders::GetSigningCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl ::std::convert::Into<String>)`](crate::operation::get_signing_certificate::builders::GetSigningCertificateFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::get_signing_certificate::builders::GetSigningCertificateFluentBuilder::set_user_pool_id): <p>The user pool ID.</p>
    /// - On success, responds with [`GetSigningCertificateOutput`](crate::operation::get_signing_certificate::GetSigningCertificateOutput) with field(s):
    ///   - [`certificate(Option<String>)`](crate::operation::get_signing_certificate::GetSigningCertificateOutput::certificate): <p>The signing certificate.</p>
    /// - On failure, responds with [`SdkError<GetSigningCertificateError>`](crate::operation::get_signing_certificate::GetSigningCertificateError)
    pub fn get_signing_certificate(&self) -> crate::operation::get_signing_certificate::builders::GetSigningCertificateFluentBuilder {
        crate::operation::get_signing_certificate::builders::GetSigningCertificateFluentBuilder::new(self.handle.clone())
    }
}
