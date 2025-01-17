// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAssessmentTarget`](crate::operation::delete_assessment_target::builders::DeleteAssessmentTargetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`assessment_target_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_assessment_target::builders::DeleteAssessmentTargetFluentBuilder::assessment_target_arn) / [`set_assessment_target_arn(Option<String>)`](crate::operation::delete_assessment_target::builders::DeleteAssessmentTargetFluentBuilder::set_assessment_target_arn): <p>The ARN that specifies the assessment target that you want to delete.</p>
    /// - On success, responds with [`DeleteAssessmentTargetOutput`](crate::operation::delete_assessment_target::DeleteAssessmentTargetOutput)
    /// - On failure, responds with [`SdkError<DeleteAssessmentTargetError>`](crate::operation::delete_assessment_target::DeleteAssessmentTargetError)
    pub fn delete_assessment_target(&self) -> crate::operation::delete_assessment_target::builders::DeleteAssessmentTargetFluentBuilder {
        crate::operation::delete_assessment_target::builders::DeleteAssessmentTargetFluentBuilder::new(self.handle.clone())
    }
}
