// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartFailbackLaunch`](crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`recovery_instance_i_ds(Vec<String>)`](crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder::recovery_instance_i_ds) / [`set_recovery_instance_i_ds(Option<Vec<String>>)`](crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder::set_recovery_instance_i_ds): <p>The IDs of the Recovery Instance whose failback launch we want to request.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder::set_tags): <p>The tags to be associated with the failback launch Job.</p>
    /// - On success, responds with [`StartFailbackLaunchOutput`](crate::operation::start_failback_launch::StartFailbackLaunchOutput) with field(s):
    ///   - [`job(Option<Job>)`](crate::operation::start_failback_launch::StartFailbackLaunchOutput::job): <p>The failback launch Job.</p>
    /// - On failure, responds with [`SdkError<StartFailbackLaunchError>`](crate::operation::start_failback_launch::StartFailbackLaunchError)
    pub fn start_failback_launch(&self) -> crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder {
        crate::operation::start_failback_launch::builders::StartFailbackLaunchFluentBuilder::new(self.handle.clone())
    }
}
