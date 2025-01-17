// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEvidenceFoldersByAssessment`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`assessment_id(impl ::std::convert::Into<String>)`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::assessment_id) / [`set_assessment_id(Option<String>)`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::set_assessment_id): <p> The unique identifier for the assessment. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::set_next_token): <p> The pagination token that's used to fetch the next set of results. </p>
    ///   - [`max_results(i32)`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::set_max_results): <p> Represents the maximum number of results on a page or for an API request call. </p>
    /// - On success, responds with [`GetEvidenceFoldersByAssessmentOutput`](crate::operation::get_evidence_folders_by_assessment::GetEvidenceFoldersByAssessmentOutput) with field(s):
    ///   - [`evidence_folders(Option<Vec<AssessmentEvidenceFolder>>)`](crate::operation::get_evidence_folders_by_assessment::GetEvidenceFoldersByAssessmentOutput::evidence_folders): <p> The list of evidence folders that the <code>GetEvidenceFoldersByAssessment</code> API returned. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_evidence_folders_by_assessment::GetEvidenceFoldersByAssessmentOutput::next_token): <p> The pagination token that's used to fetch the next set of results. </p>
    /// - On failure, responds with [`SdkError<GetEvidenceFoldersByAssessmentError>`](crate::operation::get_evidence_folders_by_assessment::GetEvidenceFoldersByAssessmentError)
    pub fn get_evidence_folders_by_assessment(
        &self,
    ) -> crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder {
        crate::operation::get_evidence_folders_by_assessment::builders::GetEvidenceFoldersByAssessmentFluentBuilder::new(self.handle.clone())
    }
}
