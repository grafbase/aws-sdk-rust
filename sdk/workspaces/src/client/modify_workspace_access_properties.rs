// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyWorkspaceAccessProperties`](crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder::set_resource_id): <p>The identifier of the directory.</p>
    ///   - [`workspace_access_properties(WorkspaceAccessProperties)`](crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder::workspace_access_properties) / [`set_workspace_access_properties(Option<WorkspaceAccessProperties>)`](crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder::set_workspace_access_properties): <p>The device types and operating systems to enable or disable for access.</p>
    /// - On success, responds with [`ModifyWorkspaceAccessPropertiesOutput`](crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesOutput)
    /// - On failure, responds with [`SdkError<ModifyWorkspaceAccessPropertiesError>`](crate::operation::modify_workspace_access_properties::ModifyWorkspaceAccessPropertiesError)
    pub fn modify_workspace_access_properties(
        &self,
    ) -> crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder {
        crate::operation::modify_workspace_access_properties::builders::ModifyWorkspaceAccessPropertiesFluentBuilder::new(self.handle.clone())
    }
}
