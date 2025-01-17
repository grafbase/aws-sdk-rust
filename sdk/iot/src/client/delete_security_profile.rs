// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSecurityProfile`](crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`security_profile_name(impl ::std::convert::Into<String>)`](crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder::security_profile_name) / [`set_security_profile_name(Option<String>)`](crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder::set_security_profile_name): <p>The name of the security profile to be deleted.</p>
    ///   - [`expected_version(i64)`](crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder::expected_version) / [`set_expected_version(Option<i64>)`](crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder::set_expected_version): <p>The expected version of the security profile. A new version is generated whenever the security profile is updated. If you specify a value that is different from the actual version, a <code>VersionConflictException</code> is thrown.</p>
    /// - On success, responds with [`DeleteSecurityProfileOutput`](crate::operation::delete_security_profile::DeleteSecurityProfileOutput)
    /// - On failure, responds with [`SdkError<DeleteSecurityProfileError>`](crate::operation::delete_security_profile::DeleteSecurityProfileError)
    pub fn delete_security_profile(&self) -> crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder {
        crate::operation::delete_security_profile::builders::DeleteSecurityProfileFluentBuilder::new(self.handle.clone())
    }
}
