// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartPipeline`](crate::operation::start_pipeline::builders::StartPipelineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_name(impl ::std::convert::Into<String>)`](crate::operation::start_pipeline::builders::StartPipelineFluentBuilder::pipeline_name) / [`set_pipeline_name(Option<String>)`](crate::operation::start_pipeline::builders::StartPipelineFluentBuilder::set_pipeline_name): <p>The name of the pipeline to start.</p>
    /// - On success, responds with [`StartPipelineOutput`](crate::operation::start_pipeline::StartPipelineOutput) with field(s):
    ///   - [`pipeline(Option<Pipeline>)`](crate::operation::start_pipeline::StartPipelineOutput::pipeline): <p>Information about an existing OpenSearch Ingestion pipeline.</p>
    /// - On failure, responds with [`SdkError<StartPipelineError>`](crate::operation::start_pipeline::StartPipelineError)
    pub fn start_pipeline(&self) -> crate::operation::start_pipeline::builders::StartPipelineFluentBuilder {
        crate::operation::start_pipeline::builders::StartPipelineFluentBuilder::new(self.handle.clone())
    }
}
