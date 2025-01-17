// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteStudio`](crate::operation::delete_studio::builders::DeleteStudioFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::delete_studio::builders::DeleteStudioFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_studio::builders::DeleteStudioFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::delete_studio::builders::DeleteStudioFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::delete_studio::builders::DeleteStudioFluentBuilder::set_studio_id): <p>The studio ID. </p>
    /// - On success, responds with [`DeleteStudioOutput`](crate::operation::delete_studio::DeleteStudioOutput) with field(s):
    ///   - [`studio(Option<Studio>)`](crate::operation::delete_studio::DeleteStudioOutput::studio): <p>Information about a studio.</p>
    /// - On failure, responds with [`SdkError<DeleteStudioError>`](crate::operation::delete_studio::DeleteStudioError)
    pub fn delete_studio(&self) -> crate::operation::delete_studio::builders::DeleteStudioFluentBuilder {
        crate::operation::delete_studio::builders::DeleteStudioFluentBuilder::new(self.handle.clone())
    }
}
