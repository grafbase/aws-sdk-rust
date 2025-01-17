// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMessageMoveTasks`](crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_arn(impl ::std::convert::Into<String>)`](crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder::source_arn) / [`set_source_arn(Option<String>)`](crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder::set_source_arn): <p>The ARN of the queue whose message movement tasks are to be listed.</p>
    ///   - [`max_results(i32)`](crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder::set_max_results): <p>The maximum number of results to include in the response. The default is 1, which provides the most recent message movement task. The upper limit is 10.</p>
    /// - On success, responds with [`ListMessageMoveTasksOutput`](crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput) with field(s):
    ///   - [`results(Option<Vec<ListMessageMoveTasksResultEntry>>)`](crate::operation::list_message_move_tasks::ListMessageMoveTasksOutput::results): <p>A list of message movement tasks and their attributes.</p>
    /// - On failure, responds with [`SdkError<ListMessageMoveTasksError>`](crate::operation::list_message_move_tasks::ListMessageMoveTasksError)
    pub fn list_message_move_tasks(&self) -> crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder {
        crate::operation::list_message_move_tasks::builders::ListMessageMoveTasksFluentBuilder::new(self.handle.clone())
    }
}
