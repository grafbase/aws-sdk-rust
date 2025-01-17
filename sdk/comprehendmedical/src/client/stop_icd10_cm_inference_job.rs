// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopICD10CMInferenceJob`](crate::operation::stop_icd10_cm_inference_job::builders::StopICD10CMInferenceJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl ::std::convert::Into<String>)`](crate::operation::stop_icd10_cm_inference_job::builders::StopICD10CMInferenceJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::stop_icd10_cm_inference_job::builders::StopICD10CMInferenceJobFluentBuilder::set_job_id): <p>The identifier of the job.</p>
    /// - On success, responds with [`StopIcd10CmInferenceJobOutput`](crate::operation::stop_icd10_cm_inference_job::StopIcd10CmInferenceJobOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::operation::stop_icd10_cm_inference_job::StopIcd10CmInferenceJobOutput::job_id): <p>The identifier generated for the job. To get the status of job, use this identifier with the <code>DescribeICD10CMInferenceJob</code> operation.</p>
    /// - On failure, responds with [`SdkError<StopICD10CMInferenceJobError>`](crate::operation::stop_icd10_cm_inference_job::StopICD10CMInferenceJobError)
    pub fn stop_icd10_cm_inference_job(&self) -> crate::operation::stop_icd10_cm_inference_job::builders::StopICD10CMInferenceJobFluentBuilder {
        crate::operation::stop_icd10_cm_inference_job::builders::StopICD10CMInferenceJobFluentBuilder::new(self.handle.clone())
    }
}
