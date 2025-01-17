// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateContactWithAddressBook`](crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contact_arn(impl ::std::convert::Into<String>)`](crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder::contact_arn) / [`set_contact_arn(Option<String>)`](crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder::set_contact_arn): <p>The ARN of the contact to associate with an address book.</p>
    ///   - [`address_book_arn(impl ::std::convert::Into<String>)`](crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder::address_book_arn) / [`set_address_book_arn(Option<String>)`](crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder::set_address_book_arn): <p>The ARN of the address book with which to associate the contact.</p>
    /// - On success, responds with [`AssociateContactWithAddressBookOutput`](crate::operation::associate_contact_with_address_book::AssociateContactWithAddressBookOutput)
    /// - On failure, responds with [`SdkError<AssociateContactWithAddressBookError>`](crate::operation::associate_contact_with_address_book::AssociateContactWithAddressBookError)
    #[deprecated(note = "Alexa For Business is no longer supported")]
    pub fn associate_contact_with_address_book(
        &self,
    ) -> crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder {
        crate::operation::associate_contact_with_address_book::builders::AssociateContactWithAddressBookFluentBuilder::new(self.handle.clone())
    }
}
