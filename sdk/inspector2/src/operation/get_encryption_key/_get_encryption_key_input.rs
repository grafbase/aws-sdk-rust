// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetEncryptionKeyInput {
    /// <p>The scan type the key encrypts.</p>
    pub scan_type: ::std::option::Option<crate::types::ScanType>,
    /// <p>The resource type the key encrypts.</p>
    pub resource_type: ::std::option::Option<crate::types::ResourceType>,
}
impl GetEncryptionKeyInput {
    /// <p>The scan type the key encrypts.</p>
    pub fn scan_type(&self) -> ::std::option::Option<&crate::types::ScanType> {
        self.scan_type.as_ref()
    }
    /// <p>The resource type the key encrypts.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&crate::types::ResourceType> {
        self.resource_type.as_ref()
    }
}
impl GetEncryptionKeyInput {
    /// Creates a new builder-style object to manufacture [`GetEncryptionKeyInput`](crate::operation::get_encryption_key::GetEncryptionKeyInput).
    pub fn builder() -> crate::operation::get_encryption_key::builders::GetEncryptionKeyInputBuilder {
        crate::operation::get_encryption_key::builders::GetEncryptionKeyInputBuilder::default()
    }
}

/// A builder for [`GetEncryptionKeyInput`](crate::operation::get_encryption_key::GetEncryptionKeyInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetEncryptionKeyInputBuilder {
    pub(crate) scan_type: ::std::option::Option<crate::types::ScanType>,
    pub(crate) resource_type: ::std::option::Option<crate::types::ResourceType>,
}
impl GetEncryptionKeyInputBuilder {
    /// <p>The scan type the key encrypts.</p>
    pub fn scan_type(mut self, input: crate::types::ScanType) -> Self {
        self.scan_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The scan type the key encrypts.</p>
    pub fn set_scan_type(mut self, input: ::std::option::Option<crate::types::ScanType>) -> Self {
        self.scan_type = input;
        self
    }
    /// <p>The scan type the key encrypts.</p>
    pub fn get_scan_type(&self) -> &::std::option::Option<crate::types::ScanType> {
        &self.scan_type
    }
    /// <p>The resource type the key encrypts.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The resource type the key encrypts.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::ResourceType>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The resource type the key encrypts.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::ResourceType> {
        &self.resource_type
    }
    /// Consumes the builder and constructs a [`GetEncryptionKeyInput`](crate::operation::get_encryption_key::GetEncryptionKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_encryption_key::GetEncryptionKeyInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_encryption_key::GetEncryptionKeyInput {
            scan_type: self.scan_type,
            resource_type: self.resource_type,
        })
    }
}
