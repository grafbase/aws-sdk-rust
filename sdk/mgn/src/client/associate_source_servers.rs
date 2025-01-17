// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateSourceServers`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::set_application_id): <p>Application ID.</p>
    ///   - [`source_server_i_ds(Vec<String>)`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::source_server_i_ds) / [`set_source_server_i_ds(Option<Vec<String>>)`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::set_source_server_i_ds): <p>Source server IDs list.</p>
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::set_account_id): <p>Account ID.</p>
    /// - On success, responds with [`AssociateSourceServersOutput`](crate::operation::associate_source_servers::AssociateSourceServersOutput)
    /// - On failure, responds with [`SdkError<AssociateSourceServersError>`](crate::operation::associate_source_servers::AssociateSourceServersError)
    pub fn associate_source_servers(&self) -> crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder {
        crate::operation::associate_source_servers::builders::AssociateSourceServersFluentBuilder::new(self.handle.clone())
    }
}
