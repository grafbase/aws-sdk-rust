// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateMembers`](crate::operation::create_members::builders::CreateMembersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_details(Vec<AccountDetails>)`](crate::operation::create_members::builders::CreateMembersFluentBuilder::account_details) / [`set_account_details(Option<Vec<AccountDetails>>)`](crate::operation::create_members::builders::CreateMembersFluentBuilder::set_account_details): <p>The list of accounts to associate with the Security Hub administrator account. For each account, the list includes the account ID and optionally the email address.</p>
    /// - On success, responds with [`CreateMembersOutput`](crate::operation::create_members::CreateMembersOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec<Result>>)`](crate::operation::create_members::CreateMembersOutput::unprocessed_accounts): <p>The list of Amazon Web Services accounts that were not processed. For each account, the list includes the account ID and the email address.</p>
    /// - On failure, responds with [`SdkError<CreateMembersError>`](crate::operation::create_members::CreateMembersError)
    pub fn create_members(&self) -> crate::operation::create_members::builders::CreateMembersFluentBuilder {
        crate::operation::create_members::builders::CreateMembersFluentBuilder::new(self.handle.clone())
    }
}
