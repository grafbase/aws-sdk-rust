// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMailboxPermissionsInput {
    /// <p>The identifier of the organization under which the member (user or group) exists.</p>
    pub organization_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the member (user or group) that owns the mailbox.</p>
    pub entity_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
    pub grantee_id: ::std::option::Option<::std::string::String>,
}
impl DeleteMailboxPermissionsInput {
    /// <p>The identifier of the organization under which the member (user or group) exists.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
    /// <p>The identifier of the member (user or group) that owns the mailbox.</p>
    pub fn entity_id(&self) -> ::std::option::Option<&str> {
        self.entity_id.as_deref()
    }
    /// <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
    pub fn grantee_id(&self) -> ::std::option::Option<&str> {
        self.grantee_id.as_deref()
    }
}
impl DeleteMailboxPermissionsInput {
    /// Creates a new builder-style object to manufacture [`DeleteMailboxPermissionsInput`](crate::operation::delete_mailbox_permissions::DeleteMailboxPermissionsInput).
    pub fn builder() -> crate::operation::delete_mailbox_permissions::builders::DeleteMailboxPermissionsInputBuilder {
        crate::operation::delete_mailbox_permissions::builders::DeleteMailboxPermissionsInputBuilder::default()
    }
}

/// A builder for [`DeleteMailboxPermissionsInput`](crate::operation::delete_mailbox_permissions::DeleteMailboxPermissionsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteMailboxPermissionsInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
    pub(crate) entity_id: ::std::option::Option<::std::string::String>,
    pub(crate) grantee_id: ::std::option::Option<::std::string::String>,
}
impl DeleteMailboxPermissionsInputBuilder {
    /// <p>The identifier of the organization under which the member (user or group) exists.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the organization under which the member (user or group) exists.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.organization_id = input;
        self
    }
    /// <p>The identifier of the organization under which the member (user or group) exists.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.organization_id
    }
    /// <p>The identifier of the member (user or group) that owns the mailbox.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the member (user or group) that owns the mailbox.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_id = input;
        self
    }
    /// <p>The identifier of the member (user or group) that owns the mailbox.</p>
    pub fn get_entity_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.entity_id
    }
    /// <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
    pub fn grantee_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.grantee_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
    pub fn set_grantee_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.grantee_id = input;
        self
    }
    /// <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
    pub fn get_grantee_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.grantee_id
    }
    /// Consumes the builder and constructs a [`DeleteMailboxPermissionsInput`](crate::operation::delete_mailbox_permissions::DeleteMailboxPermissionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_mailbox_permissions::DeleteMailboxPermissionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_mailbox_permissions::DeleteMailboxPermissionsInput {
            organization_id: self.organization_id,
            entity_id: self.entity_id,
            grantee_id: self.grantee_id,
        })
    }
}
