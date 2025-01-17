// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePipelineDefinitionForExecution`](crate::operation::describe_pipeline_definition_for_execution::builders::DescribePipelineDefinitionForExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_execution_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_pipeline_definition_for_execution::builders::DescribePipelineDefinitionForExecutionFluentBuilder::pipeline_execution_arn) / [`set_pipeline_execution_arn(Option<String>)`](crate::operation::describe_pipeline_definition_for_execution::builders::DescribePipelineDefinitionForExecutionFluentBuilder::set_pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    /// - On success, responds with [`DescribePipelineDefinitionForExecutionOutput`](crate::operation::describe_pipeline_definition_for_execution::DescribePipelineDefinitionForExecutionOutput) with field(s):
    ///   - [`pipeline_definition(Option<String>)`](crate::operation::describe_pipeline_definition_for_execution::DescribePipelineDefinitionForExecutionOutput::pipeline_definition): <p>The JSON pipeline definition.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_pipeline_definition_for_execution::DescribePipelineDefinitionForExecutionOutput::creation_time): <p>The time when the pipeline was created.</p>
    /// - On failure, responds with [`SdkError<DescribePipelineDefinitionForExecutionError>`](crate::operation::describe_pipeline_definition_for_execution::DescribePipelineDefinitionForExecutionError)
    pub fn describe_pipeline_definition_for_execution(
        &self,
    ) -> crate::operation::describe_pipeline_definition_for_execution::builders::DescribePipelineDefinitionForExecutionFluentBuilder {
        crate::operation::describe_pipeline_definition_for_execution::builders::DescribePipelineDefinitionForExecutionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
