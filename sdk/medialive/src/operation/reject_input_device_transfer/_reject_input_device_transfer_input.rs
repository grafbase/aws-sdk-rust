// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Placeholder documentation for RejectInputDeviceTransferRequest
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RejectInputDeviceTransferInput {
    /// The unique ID of the input device to reject. For example, hd-123456789abcdef.
    pub input_device_id: ::std::option::Option<::std::string::String>,
}
impl RejectInputDeviceTransferInput {
    /// The unique ID of the input device to reject. For example, hd-123456789abcdef.
    pub fn input_device_id(&self) -> ::std::option::Option<&str> {
        self.input_device_id.as_deref()
    }
}
impl RejectInputDeviceTransferInput {
    /// Creates a new builder-style object to manufacture [`RejectInputDeviceTransferInput`](crate::operation::reject_input_device_transfer::RejectInputDeviceTransferInput).
    pub fn builder() -> crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferInputBuilder {
        crate::operation::reject_input_device_transfer::builders::RejectInputDeviceTransferInputBuilder::default()
    }
}

/// A builder for [`RejectInputDeviceTransferInput`](crate::operation::reject_input_device_transfer::RejectInputDeviceTransferInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RejectInputDeviceTransferInputBuilder {
    pub(crate) input_device_id: ::std::option::Option<::std::string::String>,
}
impl RejectInputDeviceTransferInputBuilder {
    /// The unique ID of the input device to reject. For example, hd-123456789abcdef.
    pub fn input_device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.input_device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The unique ID of the input device to reject. For example, hd-123456789abcdef.
    pub fn set_input_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.input_device_id = input;
        self
    }
    /// The unique ID of the input device to reject. For example, hd-123456789abcdef.
    pub fn get_input_device_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.input_device_id
    }
    /// Consumes the builder and constructs a [`RejectInputDeviceTransferInput`](crate::operation::reject_input_device_transfer::RejectInputDeviceTransferInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reject_input_device_transfer::RejectInputDeviceTransferInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::reject_input_device_transfer::RejectInputDeviceTransferInput {
            input_device_id: self.input_device_id,
        })
    }
}
