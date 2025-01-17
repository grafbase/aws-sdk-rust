// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UnmonitorInstances`](crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_ids(Vec<String>)`](crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder::instance_ids) / [`set_instance_ids(Option<Vec<String>>)`](crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder::set_instance_ids): <p>The IDs of the instances.</p>
    ///   - [`dry_run(bool)`](crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`UnmonitorInstancesOutput`](crate::operation::unmonitor_instances::UnmonitorInstancesOutput) with field(s):
    ///   - [`instance_monitorings(Option<Vec<InstanceMonitoring>>)`](crate::operation::unmonitor_instances::UnmonitorInstancesOutput::instance_monitorings): <p>The monitoring information.</p>
    /// - On failure, responds with [`SdkError<UnmonitorInstancesError>`](crate::operation::unmonitor_instances::UnmonitorInstancesError)
    pub fn unmonitor_instances(&self) -> crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder {
        crate::operation::unmonitor_instances::builders::UnmonitorInstancesFluentBuilder::new(self.handle.clone())
    }
}
