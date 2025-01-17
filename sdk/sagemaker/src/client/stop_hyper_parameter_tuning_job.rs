// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopHyperParameterTuningJob`](crate::operation::stop_hyper_parameter_tuning_job::builders::StopHyperParameterTuningJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hyper_parameter_tuning_job_name(impl ::std::convert::Into<String>)`](crate::operation::stop_hyper_parameter_tuning_job::builders::StopHyperParameterTuningJobFluentBuilder::hyper_parameter_tuning_job_name) / [`set_hyper_parameter_tuning_job_name(Option<String>)`](crate::operation::stop_hyper_parameter_tuning_job::builders::StopHyperParameterTuningJobFluentBuilder::set_hyper_parameter_tuning_job_name): <p>The name of the tuning job to stop.</p>
    /// - On success, responds with [`StopHyperParameterTuningJobOutput`](crate::operation::stop_hyper_parameter_tuning_job::StopHyperParameterTuningJobOutput)
    /// - On failure, responds with [`SdkError<StopHyperParameterTuningJobError>`](crate::operation::stop_hyper_parameter_tuning_job::StopHyperParameterTuningJobError)
    pub fn stop_hyper_parameter_tuning_job(
        &self,
    ) -> crate::operation::stop_hyper_parameter_tuning_job::builders::StopHyperParameterTuningJobFluentBuilder {
        crate::operation::stop_hyper_parameter_tuning_job::builders::StopHyperParameterTuningJobFluentBuilder::new(self.handle.clone())
    }
}
