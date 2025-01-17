// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelArchival`](crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl ::std::convert::Into<String>)`](crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`tape_arn(impl ::std::convert::Into<String>)`](crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder::tape_arn) / [`set_tape_arn(Option<String>)`](crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder::set_tape_arn): <p>The Amazon Resource Name (ARN) of the virtual tape you want to cancel archiving for.</p>
    /// - On success, responds with [`CancelArchivalOutput`](crate::operation::cancel_archival::CancelArchivalOutput) with field(s):
    ///   - [`tape_arn(Option<String>)`](crate::operation::cancel_archival::CancelArchivalOutput::tape_arn): <p>The Amazon Resource Name (ARN) of the virtual tape for which archiving was canceled.</p>
    /// - On failure, responds with [`SdkError<CancelArchivalError>`](crate::operation::cancel_archival::CancelArchivalError)
    pub fn cancel_archival(&self) -> crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder {
        crate::operation::cancel_archival::builders::CancelArchivalFluentBuilder::new(self.handle.clone())
    }
}
