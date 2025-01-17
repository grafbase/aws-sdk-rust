// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemoveThingFromBillingGroup`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`billing_group_name(impl ::std::convert::Into<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::billing_group_name) / [`set_billing_group_name(Option<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::set_billing_group_name): <p>The name of the billing group.</p>
    ///   - [`billing_group_arn(impl ::std::convert::Into<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::billing_group_arn) / [`set_billing_group_arn(Option<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::set_billing_group_arn): <p>The ARN of the billing group.</p>
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::set_thing_name): <p>The name of the thing to be removed from the billing group.</p>
    ///   - [`thing_arn(impl ::std::convert::Into<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::thing_arn) / [`set_thing_arn(Option<String>)`](crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::set_thing_arn): <p>The ARN of the thing to be removed from the billing group.</p>
    /// - On success, responds with [`RemoveThingFromBillingGroupOutput`](crate::operation::remove_thing_from_billing_group::RemoveThingFromBillingGroupOutput)
    /// - On failure, responds with [`SdkError<RemoveThingFromBillingGroupError>`](crate::operation::remove_thing_from_billing_group::RemoveThingFromBillingGroupError)
    pub fn remove_thing_from_billing_group(
        &self,
    ) -> crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder {
        crate::operation::remove_thing_from_billing_group::builders::RemoveThingFromBillingGroupFluentBuilder::new(self.handle.clone())
    }
}
