// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Settings that allow management of telephony permissions for an Amazon Chime user, such as inbound and outbound calling and text messaging.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TelephonySettings {
    /// <p>Allows or denies inbound calling.</p>
    pub inbound_calling: ::std::option::Option<bool>,
    /// <p>Allows or denies outbound calling.</p>
    pub outbound_calling: ::std::option::Option<bool>,
    /// <p>Allows or denies SMS messaging.</p>
    pub sms: ::std::option::Option<bool>,
}
impl TelephonySettings {
    /// <p>Allows or denies inbound calling.</p>
    pub fn inbound_calling(&self) -> ::std::option::Option<bool> {
        self.inbound_calling
    }
    /// <p>Allows or denies outbound calling.</p>
    pub fn outbound_calling(&self) -> ::std::option::Option<bool> {
        self.outbound_calling
    }
    /// <p>Allows or denies SMS messaging.</p>
    pub fn sms(&self) -> ::std::option::Option<bool> {
        self.sms
    }
}
impl TelephonySettings {
    /// Creates a new builder-style object to manufacture [`TelephonySettings`](crate::types::TelephonySettings).
    pub fn builder() -> crate::types::builders::TelephonySettingsBuilder {
        crate::types::builders::TelephonySettingsBuilder::default()
    }
}

/// A builder for [`TelephonySettings`](crate::types::TelephonySettings).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TelephonySettingsBuilder {
    pub(crate) inbound_calling: ::std::option::Option<bool>,
    pub(crate) outbound_calling: ::std::option::Option<bool>,
    pub(crate) sms: ::std::option::Option<bool>,
}
impl TelephonySettingsBuilder {
    /// <p>Allows or denies inbound calling.</p>
    pub fn inbound_calling(mut self, input: bool) -> Self {
        self.inbound_calling = ::std::option::Option::Some(input);
        self
    }
    /// <p>Allows or denies inbound calling.</p>
    pub fn set_inbound_calling(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inbound_calling = input;
        self
    }
    /// <p>Allows or denies inbound calling.</p>
    pub fn get_inbound_calling(&self) -> &::std::option::Option<bool> {
        &self.inbound_calling
    }
    /// <p>Allows or denies outbound calling.</p>
    pub fn outbound_calling(mut self, input: bool) -> Self {
        self.outbound_calling = ::std::option::Option::Some(input);
        self
    }
    /// <p>Allows or denies outbound calling.</p>
    pub fn set_outbound_calling(mut self, input: ::std::option::Option<bool>) -> Self {
        self.outbound_calling = input;
        self
    }
    /// <p>Allows or denies outbound calling.</p>
    pub fn get_outbound_calling(&self) -> &::std::option::Option<bool> {
        &self.outbound_calling
    }
    /// <p>Allows or denies SMS messaging.</p>
    pub fn sms(mut self, input: bool) -> Self {
        self.sms = ::std::option::Option::Some(input);
        self
    }
    /// <p>Allows or denies SMS messaging.</p>
    pub fn set_sms(mut self, input: ::std::option::Option<bool>) -> Self {
        self.sms = input;
        self
    }
    /// <p>Allows or denies SMS messaging.</p>
    pub fn get_sms(&self) -> &::std::option::Option<bool> {
        &self.sms
    }
    /// Consumes the builder and constructs a [`TelephonySettings`](crate::types::TelephonySettings).
    pub fn build(self) -> crate::types::TelephonySettings {
        crate::types::TelephonySettings {
            inbound_calling: self.inbound_calling,
            outbound_calling: self.outbound_calling,
            sms: self.sms,
        }
    }
}
