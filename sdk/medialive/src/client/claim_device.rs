// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ClaimDevice`](crate::operation::claim_device::builders::ClaimDeviceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::claim_device::builders::ClaimDeviceFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::claim_device::builders::ClaimDeviceFluentBuilder::set_id): The id of the device you want to claim.
    /// - On success, responds with [`ClaimDeviceOutput`](crate::operation::claim_device::ClaimDeviceOutput)
    /// - On failure, responds with [`SdkError<ClaimDeviceError>`](crate::operation::claim_device::ClaimDeviceError)
    pub fn claim_device(&self) -> crate::operation::claim_device::builders::ClaimDeviceFluentBuilder {
        crate::operation::claim_device::builders::ClaimDeviceFluentBuilder::new(self.handle.clone())
    }
}
