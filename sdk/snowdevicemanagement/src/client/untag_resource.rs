// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UntagResource`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the device or task.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_tag_keys): <p>Optional metadata that you assign to a resource. You can use tags to categorize a resource in different ways, such as by purpose, owner, or environment.</p>
    /// - On success, responds with [`UntagResourceOutput`](crate::operation::untag_resource::UntagResourceOutput)
    /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::operation::untag_resource::UntagResourceError)
    pub fn untag_resource(&self) -> crate::operation::untag_resource::builders::UntagResourceFluentBuilder {
        crate::operation::untag_resource::builders::UntagResourceFluentBuilder::new(self.handle.clone())
    }
}
