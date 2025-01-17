// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateFilter`](crate::operation::create_filter::builders::CreateFilterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`action(FilterAction)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::action) / [`set_action(Option<FilterAction>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::set_action): <p>Defines the action that is to be applied to the findings that match the filter.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::set_description): <p>A description of the filter.</p>
    ///   - [`filter_criteria(FilterCriteria)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::filter_criteria) / [`set_filter_criteria(Option<FilterCriteria>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::set_filter_criteria): <p>Defines the criteria to be used in the filter for querying findings.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::set_name): <p>The name of the filter. Minimum length of 3. Maximum length of 64. Valid characters include alphanumeric characters, dot (.), underscore (_), and dash (-). Spaces are not allowed.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::set_tags): <p>A list of tags for the filter.</p>
    ///   - [`reason(impl ::std::convert::Into<String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::reason) / [`set_reason(Option<String>)`](crate::operation::create_filter::builders::CreateFilterFluentBuilder::set_reason): <p>The reason for creating the filter.</p>
    /// - On success, responds with [`CreateFilterOutput`](crate::operation::create_filter::CreateFilterOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_filter::CreateFilterOutput::arn): <p>The Amazon Resource Number (ARN) of the successfully created filter.</p>
    /// - On failure, responds with [`SdkError<CreateFilterError>`](crate::operation::create_filter::CreateFilterError)
    pub fn create_filter(&self) -> crate::operation::create_filter::builders::CreateFilterFluentBuilder {
        crate::operation::create_filter::builders::CreateFilterFluentBuilder::new(self.handle.clone())
    }
}
