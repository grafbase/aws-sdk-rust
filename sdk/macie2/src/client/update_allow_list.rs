// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAllowList`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`criteria(AllowListCriteria)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::criteria) / [`set_criteria(Option<AllowListCriteria>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::set_criteria): <p>The criteria that specify the text or text pattern to ignore. The criteria can be the location and name of an S3 object that lists specific text to ignore (s3WordsList), or a regular expression that defines a text pattern to ignore (regex).</p>  <p>You can change a list's underlying criteria, such as the name of the S3 object or the regular expression to use. However, you can't change the type from s3WordsList to regex or the other way around.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::set_description): <p>A custom description of the allow list. The description can contain as many as 512 characters.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::set_id): <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::set_name): <p>A custom name for the allow list. The name can contain as many as 128 characters.</p>
    /// - On success, responds with [`UpdateAllowListOutput`](crate::operation::update_allow_list::UpdateAllowListOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_allow_list::UpdateAllowListOutput::arn): <p>The Amazon Resource Name (ARN) of the allow list.</p>
    ///   - [`id(Option<String>)`](crate::operation::update_allow_list::UpdateAllowListOutput::id): <p>The unique identifier for the allow list.</p>
    /// - On failure, responds with [`SdkError<UpdateAllowListError>`](crate::operation::update_allow_list::UpdateAllowListError)
    pub fn update_allow_list(&self) -> crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder {
        crate::operation::update_allow_list::builders::UpdateAllowListFluentBuilder::new(self.handle.clone())
    }
}
