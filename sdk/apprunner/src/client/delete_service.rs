// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteService`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::service_arn) / [`set_service_arn(Option<String>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::set_service_arn): <p>The Amazon Resource Name (ARN) of the App Runner service that you want to delete.</p>
    /// - On success, responds with [`DeleteServiceOutput`](crate::operation::delete_service::DeleteServiceOutput) with field(s):
    ///   - [`service(Option<Service>)`](crate::operation::delete_service::DeleteServiceOutput::service): <p>A description of the App Runner service that this request just deleted.</p>
    ///   - [`operation_id(Option<String>)`](crate::operation::delete_service::DeleteServiceOutput::operation_id): <p>The unique ID of the asynchronous operation that this request started. You can use it combined with the <code>ListOperations</code> call to track the operation's progress.</p>
    /// - On failure, responds with [`SdkError<DeleteServiceError>`](crate::operation::delete_service::DeleteServiceError)
    pub fn delete_service(&self) -> crate::operation::delete_service::builders::DeleteServiceFluentBuilder {
        crate::operation::delete_service::builders::DeleteServiceFluentBuilder::new(self.handle.clone())
    }
}
