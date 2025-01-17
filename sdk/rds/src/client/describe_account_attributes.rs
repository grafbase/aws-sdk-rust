// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAccountAttributes`](crate::operation::describe_account_attributes::builders::DescribeAccountAttributesFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_account_attributes::builders::DescribeAccountAttributesFluentBuilder::send) it.
    /// - On success, responds with [`DescribeAccountAttributesOutput`](crate::operation::describe_account_attributes::DescribeAccountAttributesOutput) with field(s):
    ///   - [`account_quotas(Option<Vec<AccountQuota>>)`](crate::operation::describe_account_attributes::DescribeAccountAttributesOutput::account_quotas): <p>A list of <code>AccountQuota</code> objects. Within this list, each quota has a name, a count of usage toward the quota maximum, and a maximum value for the quota.</p>
    /// - On failure, responds with [`SdkError<DescribeAccountAttributesError>`](crate::operation::describe_account_attributes::DescribeAccountAttributesError)
    pub fn describe_account_attributes(&self) -> crate::operation::describe_account_attributes::builders::DescribeAccountAttributesFluentBuilder {
        crate::operation::describe_account_attributes::builders::DescribeAccountAttributesFluentBuilder::new(self.handle.clone())
    }
}
