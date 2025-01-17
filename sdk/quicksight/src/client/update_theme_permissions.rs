// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateThemePermissions`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the theme.</p>
    ///   - [`theme_id(impl ::std::convert::Into<String>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::theme_id) / [`set_theme_id(Option<String>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::set_theme_id): <p>The ID for the theme.</p>
    ///   - [`grant_permissions(Vec<ResourcePermission>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::grant_permissions) / [`set_grant_permissions(Option<Vec<ResourcePermission>>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::set_grant_permissions): <p>A list of resource permissions to be granted for the theme.</p>
    ///   - [`revoke_permissions(Vec<ResourcePermission>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::revoke_permissions) / [`set_revoke_permissions(Option<Vec<ResourcePermission>>)`](crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::set_revoke_permissions): <p>A list of resource permissions to be revoked from the theme.</p>
    /// - On success, responds with [`UpdateThemePermissionsOutput`](crate::operation::update_theme_permissions::UpdateThemePermissionsOutput) with field(s):
    ///   - [`theme_id(Option<String>)`](crate::operation::update_theme_permissions::UpdateThemePermissionsOutput::theme_id): <p>The ID for the theme.</p>
    ///   - [`theme_arn(Option<String>)`](crate::operation::update_theme_permissions::UpdateThemePermissionsOutput::theme_arn): <p>The Amazon Resource Name (ARN) of the theme.</p>
    ///   - [`permissions(Option<Vec<ResourcePermission>>)`](crate::operation::update_theme_permissions::UpdateThemePermissionsOutput::permissions): <p>The resulting list of resource permissions for the theme.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::update_theme_permissions::UpdateThemePermissionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::update_theme_permissions::UpdateThemePermissionsOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<UpdateThemePermissionsError>`](crate::operation::update_theme_permissions::UpdateThemePermissionsError)
    pub fn update_theme_permissions(&self) -> crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder {
        crate::operation::update_theme_permissions::builders::UpdateThemePermissionsFluentBuilder::new(self.handle.clone())
    }
}
