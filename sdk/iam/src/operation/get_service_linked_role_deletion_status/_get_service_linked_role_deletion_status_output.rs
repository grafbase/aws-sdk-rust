// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetServiceLinkedRoleDeletionStatusOutput {
    /// <p>The status of the deletion.</p>
    pub status: ::std::option::Option<crate::types::DeletionTaskStatusType>,
    /// <p>An object that contains details about the reason the deletion failed.</p>
    pub reason: ::std::option::Option<crate::types::DeletionTaskFailureReasonType>,
    _request_id: Option<String>,
}
impl GetServiceLinkedRoleDeletionStatusOutput {
    /// <p>The status of the deletion.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DeletionTaskStatusType> {
        self.status.as_ref()
    }
    /// <p>An object that contains details about the reason the deletion failed.</p>
    pub fn reason(&self) -> ::std::option::Option<&crate::types::DeletionTaskFailureReasonType> {
        self.reason.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetServiceLinkedRoleDeletionStatusOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetServiceLinkedRoleDeletionStatusOutput {
    /// Creates a new builder-style object to manufacture [`GetServiceLinkedRoleDeletionStatusOutput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput).
    pub fn builder() -> crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusOutputBuilder {
        crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusOutputBuilder::default()
    }
}

/// A builder for [`GetServiceLinkedRoleDeletionStatusOutput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetServiceLinkedRoleDeletionStatusOutputBuilder {
    pub(crate) status: ::std::option::Option<crate::types::DeletionTaskStatusType>,
    pub(crate) reason: ::std::option::Option<crate::types::DeletionTaskFailureReasonType>,
    _request_id: Option<String>,
}
impl GetServiceLinkedRoleDeletionStatusOutputBuilder {
    /// <p>The status of the deletion.</p>
    pub fn status(mut self, input: crate::types::DeletionTaskStatusType) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the deletion.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::DeletionTaskStatusType>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the deletion.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::DeletionTaskStatusType> {
        &self.status
    }
    /// <p>An object that contains details about the reason the deletion failed.</p>
    pub fn reason(mut self, input: crate::types::DeletionTaskFailureReasonType) -> Self {
        self.reason = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object that contains details about the reason the deletion failed.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<crate::types::DeletionTaskFailureReasonType>) -> Self {
        self.reason = input;
        self
    }
    /// <p>An object that contains details about the reason the deletion failed.</p>
    pub fn get_reason(&self) -> &::std::option::Option<crate::types::DeletionTaskFailureReasonType> {
        &self.reason
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetServiceLinkedRoleDeletionStatusOutput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput).
    pub fn build(self) -> crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput {
        crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput {
            status: self.status,
            reason: self.reason,
            _request_id: self._request_id,
        }
    }
}
