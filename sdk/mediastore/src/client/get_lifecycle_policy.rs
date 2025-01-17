// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLifecyclePolicy`](crate::operation::get_lifecycle_policy::builders::GetLifecyclePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`container_name(impl ::std::convert::Into<String>)`](crate::operation::get_lifecycle_policy::builders::GetLifecyclePolicyFluentBuilder::container_name) / [`set_container_name(Option<String>)`](crate::operation::get_lifecycle_policy::builders::GetLifecyclePolicyFluentBuilder::set_container_name): <p>The name of the container that the object lifecycle policy is assigned to.</p>
    /// - On success, responds with [`GetLifecyclePolicyOutput`](crate::operation::get_lifecycle_policy::GetLifecyclePolicyOutput) with field(s):
    ///   - [`lifecycle_policy(Option<String>)`](crate::operation::get_lifecycle_policy::GetLifecyclePolicyOutput::lifecycle_policy): <p>The object lifecycle policy that is assigned to the container.</p>
    /// - On failure, responds with [`SdkError<GetLifecyclePolicyError>`](crate::operation::get_lifecycle_policy::GetLifecyclePolicyError)
    pub fn get_lifecycle_policy(&self) -> crate::operation::get_lifecycle_policy::builders::GetLifecyclePolicyFluentBuilder {
        crate::operation::get_lifecycle_policy::builders::GetLifecyclePolicyFluentBuilder::new(self.handle.clone())
    }
}
