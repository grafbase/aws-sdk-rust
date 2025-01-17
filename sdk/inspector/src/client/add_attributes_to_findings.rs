// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddAttributesToFindings`](crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`finding_arns(Vec<String>)`](crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder::finding_arns) / [`set_finding_arns(Option<Vec<String>>)`](crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder::set_finding_arns): <p>The ARNs that specify the findings that you want to assign attributes to.</p>
    ///   - [`attributes(Vec<Attribute>)`](crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder::attributes) / [`set_attributes(Option<Vec<Attribute>>)`](crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder::set_attributes): <p>The array of attributes that you want to assign to specified findings.</p>
    /// - On success, responds with [`AddAttributesToFindingsOutput`](crate::operation::add_attributes_to_findings::AddAttributesToFindingsOutput) with field(s):
    ///   - [`failed_items(Option<HashMap<String, FailedItemDetails>>)`](crate::operation::add_attributes_to_findings::AddAttributesToFindingsOutput::failed_items): <p>Attribute details that cannot be described. An error code is provided for each failed item.</p>
    /// - On failure, responds with [`SdkError<AddAttributesToFindingsError>`](crate::operation::add_attributes_to_findings::AddAttributesToFindingsError)
    pub fn add_attributes_to_findings(&self) -> crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder {
        crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsFluentBuilder::new(self.handle.clone())
    }
}
