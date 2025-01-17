// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSourceApiAssociation`](crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`merged_api_identifier(impl ::std::convert::Into<String>)`](crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder::merged_api_identifier) / [`set_merged_api_identifier(Option<String>)`](crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder::set_merged_api_identifier): <p>The identifier of the AppSync Merged API. This is generated by the AppSync service. In most cases, Merged APIs (especially in your account) only require the API ID value or ARN of the merged API. However, Merged APIs in other accounts (cross-account use cases) strictly require the full resource ARN of the merged API.</p>
    ///   - [`association_id(impl ::std::convert::Into<String>)`](crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder::association_id) / [`set_association_id(Option<String>)`](crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder::set_association_id): <p>The ID generated by the AppSync service for the source API association.</p>
    /// - On success, responds with [`GetSourceApiAssociationOutput`](crate::operation::get_source_api_association::GetSourceApiAssociationOutput) with field(s):
    ///   - [`source_api_association(Option<SourceApiAssociation>)`](crate::operation::get_source_api_association::GetSourceApiAssociationOutput::source_api_association): <p>The <code>SourceApiAssociation</code> object data.</p>
    /// - On failure, responds with [`SdkError<GetSourceApiAssociationError>`](crate::operation::get_source_api_association::GetSourceApiAssociationError)
    pub fn get_source_api_association(&self) -> crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder {
        crate::operation::get_source_api_association::builders::GetSourceApiAssociationFluentBuilder::new(self.handle.clone())
    }
}
