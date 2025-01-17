// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Parameters to derive session key for Visa payment card for ARQC verification.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SessionKeyVisa {
    /// <p>The Primary Account Number (PAN) of the cardholder. A PAN is a unique identifier for a payment credit or debit card and associates the card to a specific account holder.</p>
    pub primary_account_number: ::std::option::Option<::std::string::String>,
    /// <p>A number that identifies and differentiates payment cards with the same Primary Account Number (PAN).</p>
    pub pan_sequence_number: ::std::option::Option<::std::string::String>,
}
impl SessionKeyVisa {
    /// <p>The Primary Account Number (PAN) of the cardholder. A PAN is a unique identifier for a payment credit or debit card and associates the card to a specific account holder.</p>
    pub fn primary_account_number(&self) -> ::std::option::Option<&str> {
        self.primary_account_number.as_deref()
    }
    /// <p>A number that identifies and differentiates payment cards with the same Primary Account Number (PAN).</p>
    pub fn pan_sequence_number(&self) -> ::std::option::Option<&str> {
        self.pan_sequence_number.as_deref()
    }
}
impl ::std::fmt::Debug for SessionKeyVisa {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SessionKeyVisa");
        formatter.field("primary_account_number", &"*** Sensitive Data Redacted ***");
        formatter.field("pan_sequence_number", &self.pan_sequence_number);
        formatter.finish()
    }
}
impl SessionKeyVisa {
    /// Creates a new builder-style object to manufacture [`SessionKeyVisa`](crate::types::SessionKeyVisa).
    pub fn builder() -> crate::types::builders::SessionKeyVisaBuilder {
        crate::types::builders::SessionKeyVisaBuilder::default()
    }
}

/// A builder for [`SessionKeyVisa`](crate::types::SessionKeyVisa).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct SessionKeyVisaBuilder {
    pub(crate) primary_account_number: ::std::option::Option<::std::string::String>,
    pub(crate) pan_sequence_number: ::std::option::Option<::std::string::String>,
}
impl SessionKeyVisaBuilder {
    /// <p>The Primary Account Number (PAN) of the cardholder. A PAN is a unique identifier for a payment credit or debit card and associates the card to a specific account holder.</p>
    pub fn primary_account_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.primary_account_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Primary Account Number (PAN) of the cardholder. A PAN is a unique identifier for a payment credit or debit card and associates the card to a specific account holder.</p>
    pub fn set_primary_account_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.primary_account_number = input;
        self
    }
    /// <p>The Primary Account Number (PAN) of the cardholder. A PAN is a unique identifier for a payment credit or debit card and associates the card to a specific account holder.</p>
    pub fn get_primary_account_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.primary_account_number
    }
    /// <p>A number that identifies and differentiates payment cards with the same Primary Account Number (PAN).</p>
    pub fn pan_sequence_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pan_sequence_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A number that identifies and differentiates payment cards with the same Primary Account Number (PAN).</p>
    pub fn set_pan_sequence_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pan_sequence_number = input;
        self
    }
    /// <p>A number that identifies and differentiates payment cards with the same Primary Account Number (PAN).</p>
    pub fn get_pan_sequence_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.pan_sequence_number
    }
    /// Consumes the builder and constructs a [`SessionKeyVisa`](crate::types::SessionKeyVisa).
    pub fn build(self) -> crate::types::SessionKeyVisa {
        crate::types::SessionKeyVisa {
            primary_account_number: self.primary_account_number,
            pan_sequence_number: self.pan_sequence_number,
        }
    }
}
impl ::std::fmt::Debug for SessionKeyVisaBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SessionKeyVisaBuilder");
        formatter.field("primary_account_number", &"*** Sensitive Data Redacted ***");
        formatter.field("pan_sequence_number", &self.pan_sequence_number);
        formatter.finish()
    }
}
