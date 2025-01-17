// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddThingToBillingGroup`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`billing_group_name(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::billing_group_name) / [`set_billing_group_name(Option<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::set_billing_group_name): <p>The name of the billing group.</p> <note>   <p>This call is asynchronous. It might take several seconds for the detachment to propagate.</p>  </note>
    ///   - [`billing_group_arn(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::billing_group_arn) / [`set_billing_group_arn(Option<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::set_billing_group_arn): <p>The ARN of the billing group.</p>
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::set_thing_name): <p>The name of the thing to be added to the billing group.</p>
    ///   - [`thing_arn(impl ::std::convert::Into<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::thing_arn) / [`set_thing_arn(Option<String>)`](crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::set_thing_arn): <p>The ARN of the thing to be added to the billing group.</p>
    /// - On success, responds with [`AddThingToBillingGroupOutput`](crate::operation::add_thing_to_billing_group::AddThingToBillingGroupOutput)
    /// - On failure, responds with [`SdkError<AddThingToBillingGroupError>`](crate::operation::add_thing_to_billing_group::AddThingToBillingGroupError)
    pub fn add_thing_to_billing_group(&self) -> crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder {
        crate::operation::add_thing_to_billing_group::builders::AddThingToBillingGroupFluentBuilder::new(self.handle.clone())
    }
}
