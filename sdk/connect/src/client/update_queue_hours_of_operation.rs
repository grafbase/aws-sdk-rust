// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateQueueHoursOfOperation`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`queue_id(impl ::std::convert::Into<String>)`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::queue_id) / [`set_queue_id(Option<String>)`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::set_queue_id): <p>The identifier for the queue.</p>
    ///   - [`hours_of_operation_id(impl ::std::convert::Into<String>)`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::hours_of_operation_id) / [`set_hours_of_operation_id(Option<String>)`](crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::set_hours_of_operation_id): <p>The identifier for the hours of operation.</p>
    /// - On success, responds with [`UpdateQueueHoursOfOperationOutput`](crate::operation::update_queue_hours_of_operation::UpdateQueueHoursOfOperationOutput)
    /// - On failure, responds with [`SdkError<UpdateQueueHoursOfOperationError>`](crate::operation::update_queue_hours_of_operation::UpdateQueueHoursOfOperationError)
    pub fn update_queue_hours_of_operation(
        &self,
    ) -> crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder {
        crate::operation::update_queue_hours_of_operation::builders::UpdateQueueHoursOfOperationFluentBuilder::new(self.handle.clone())
    }
}
