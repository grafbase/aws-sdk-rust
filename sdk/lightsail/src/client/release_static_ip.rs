// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ReleaseStaticIp`](crate::operation::release_static_ip::builders::ReleaseStaticIpFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`static_ip_name(impl ::std::convert::Into<String>)`](crate::operation::release_static_ip::builders::ReleaseStaticIpFluentBuilder::static_ip_name) / [`set_static_ip_name(Option<String>)`](crate::operation::release_static_ip::builders::ReleaseStaticIpFluentBuilder::set_static_ip_name): <p>The name of the static IP to delete.</p>
    /// - On success, responds with [`ReleaseStaticIpOutput`](crate::operation::release_static_ip::ReleaseStaticIpOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::operation::release_static_ip::ReleaseStaticIpOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<ReleaseStaticIpError>`](crate::operation::release_static_ip::ReleaseStaticIpError)
    pub fn release_static_ip(&self) -> crate::operation::release_static_ip::builders::ReleaseStaticIpFluentBuilder {
        crate::operation::release_static_ip::builders::ReleaseStaticIpFluentBuilder::new(self.handle.clone())
    }
}
