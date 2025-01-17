// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSampleFindings`](crate::operation::create_sample_findings::builders::CreateSampleFindingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`finding_types(Vec<FindingType>)`](crate::operation::create_sample_findings::builders::CreateSampleFindingsFluentBuilder::finding_types) / [`set_finding_types(Option<Vec<FindingType>>)`](crate::operation::create_sample_findings::builders::CreateSampleFindingsFluentBuilder::set_finding_types): <p>An array of finding types, one for each type of sample finding to create. To create a sample of every type of finding that Amazon Macie supports, don't include this array in your request.</p>
    /// - On success, responds with [`CreateSampleFindingsOutput`](crate::operation::create_sample_findings::CreateSampleFindingsOutput)
    /// - On failure, responds with [`SdkError<CreateSampleFindingsError>`](crate::operation::create_sample_findings::CreateSampleFindingsError)
    pub fn create_sample_findings(&self) -> crate::operation::create_sample_findings::builders::CreateSampleFindingsFluentBuilder {
        crate::operation::create_sample_findings::builders::CreateSampleFindingsFluentBuilder::new(self.handle.clone())
    }
}
