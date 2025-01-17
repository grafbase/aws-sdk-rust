// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteWebhook`](crate::operation::delete_webhook::builders::DeleteWebhookFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_webhook::builders::DeleteWebhookFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_webhook::builders::DeleteWebhookFluentBuilder::set_name): <p>The name of the webhook you want to delete.</p>
    /// - On success, responds with [`DeleteWebhookOutput`](crate::operation::delete_webhook::DeleteWebhookOutput)
    /// - On failure, responds with [`SdkError<DeleteWebhookError>`](crate::operation::delete_webhook::DeleteWebhookError)
    pub fn delete_webhook(&self) -> crate::operation::delete_webhook::builders::DeleteWebhookFluentBuilder {
        crate::operation::delete_webhook::builders::DeleteWebhookFluentBuilder::new(self.handle.clone())
    }
}
