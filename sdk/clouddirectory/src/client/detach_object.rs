// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DetachObject`](crate::operation::detach_object::builders::DetachObjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl ::std::convert::Into<String>)`](crate::operation::detach_object::builders::DetachObjectFluentBuilder::directory_arn) / [`set_directory_arn(Option<String>)`](crate::operation::detach_object::builders::DetachObjectFluentBuilder::set_directory_arn): <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where objects reside. For more information, see <code>arns</code>.</p>
    ///   - [`parent_reference(ObjectReference)`](crate::operation::detach_object::builders::DetachObjectFluentBuilder::parent_reference) / [`set_parent_reference(Option<ObjectReference>)`](crate::operation::detach_object::builders::DetachObjectFluentBuilder::set_parent_reference): <p>The parent reference from which the object with the specified link name is detached.</p>
    ///   - [`link_name(impl ::std::convert::Into<String>)`](crate::operation::detach_object::builders::DetachObjectFluentBuilder::link_name) / [`set_link_name(Option<String>)`](crate::operation::detach_object::builders::DetachObjectFluentBuilder::set_link_name): <p>The link name associated with the object that needs to be detached.</p>
    /// - On success, responds with [`DetachObjectOutput`](crate::operation::detach_object::DetachObjectOutput) with field(s):
    ///   - [`detached_object_identifier(Option<String>)`](crate::operation::detach_object::DetachObjectOutput::detached_object_identifier): <p>The <code>ObjectIdentifier</code> that was detached from the object.</p>
    /// - On failure, responds with [`SdkError<DetachObjectError>`](crate::operation::detach_object::DetachObjectError)
    pub fn detach_object(&self) -> crate::operation::detach_object::builders::DetachObjectFluentBuilder {
        crate::operation::detach_object::builders::DetachObjectFluentBuilder::new(self.handle.clone())
    }
}
