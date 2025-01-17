// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateTagOptionFromResource`](crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder::set_resource_id): <p>The resource identifier.</p>
    ///   - [`tag_option_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder::tag_option_id) / [`set_tag_option_id(Option<String>)`](crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder::set_tag_option_id): <p>The TagOption identifier.</p>
    /// - On success, responds with [`DisassociateTagOptionFromResourceOutput`](crate::operation::disassociate_tag_option_from_resource::DisassociateTagOptionFromResourceOutput)
    /// - On failure, responds with [`SdkError<DisassociateTagOptionFromResourceError>`](crate::operation::disassociate_tag_option_from_resource::DisassociateTagOptionFromResourceError)
    pub fn disassociate_tag_option_from_resource(
        &self,
    ) -> crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder {
        crate::operation::disassociate_tag_option_from_resource::builders::DisassociateTagOptionFromResourceFluentBuilder::new(self.handle.clone())
    }
}
