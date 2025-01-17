// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateMembers`](crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder::set_detector_id): <p>The unique ID of the detector of the GuardDuty account whose members you want to disassociate from the administrator account.</p>
    ///   - [`account_ids(Vec<String>)`](crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder::set_account_ids): <p>A list of account IDs of the GuardDuty member accounts that you want to disassociate from the administrator account.</p>
    /// - On success, responds with [`DisassociateMembersOutput`](crate::operation::disassociate_members::DisassociateMembersOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec<UnprocessedAccount>>)`](crate::operation::disassociate_members::DisassociateMembersOutput::unprocessed_accounts): <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    /// - On failure, responds with [`SdkError<DisassociateMembersError>`](crate::operation::disassociate_members::DisassociateMembersError)
    pub fn disassociate_members(&self) -> crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder {
        crate::operation::disassociate_members::builders::DisassociateMembersFluentBuilder::new(self.handle.clone())
    }
}
