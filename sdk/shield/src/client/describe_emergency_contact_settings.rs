// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEmergencyContactSettings`](crate::operation::describe_emergency_contact_settings::builders::DescribeEmergencyContactSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_emergency_contact_settings::builders::DescribeEmergencyContactSettingsFluentBuilder::send) it.
    /// - On success, responds with [`DescribeEmergencyContactSettingsOutput`](crate::operation::describe_emergency_contact_settings::DescribeEmergencyContactSettingsOutput) with field(s):
    ///   - [`emergency_contact_list(Option<Vec<EmergencyContact>>)`](crate::operation::describe_emergency_contact_settings::DescribeEmergencyContactSettingsOutput::emergency_contact_list): <p>A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p>
    /// - On failure, responds with [`SdkError<DescribeEmergencyContactSettingsError>`](crate::operation::describe_emergency_contact_settings::DescribeEmergencyContactSettingsError)
    pub fn describe_emergency_contact_settings(
        &self,
    ) -> crate::operation::describe_emergency_contact_settings::builders::DescribeEmergencyContactSettingsFluentBuilder {
        crate::operation::describe_emergency_contact_settings::builders::DescribeEmergencyContactSettingsFluentBuilder::new(self.handle.clone())
    }
}
