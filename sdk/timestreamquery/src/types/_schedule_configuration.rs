// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration of the schedule of the query.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScheduleConfiguration {
    /// <p>An expression that denotes when to trigger the scheduled query run. This can be a cron expression or a rate expression. </p>
    pub schedule_expression: ::std::option::Option<::std::string::String>,
}
impl ScheduleConfiguration {
    /// <p>An expression that denotes when to trigger the scheduled query run. This can be a cron expression or a rate expression. </p>
    pub fn schedule_expression(&self) -> ::std::option::Option<&str> {
        self.schedule_expression.as_deref()
    }
}
impl ScheduleConfiguration {
    /// Creates a new builder-style object to manufacture [`ScheduleConfiguration`](crate::types::ScheduleConfiguration).
    pub fn builder() -> crate::types::builders::ScheduleConfigurationBuilder {
        crate::types::builders::ScheduleConfigurationBuilder::default()
    }
}

/// A builder for [`ScheduleConfiguration`](crate::types::ScheduleConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ScheduleConfigurationBuilder {
    pub(crate) schedule_expression: ::std::option::Option<::std::string::String>,
}
impl ScheduleConfigurationBuilder {
    /// <p>An expression that denotes when to trigger the scheduled query run. This can be a cron expression or a rate expression. </p>
    pub fn schedule_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schedule_expression = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An expression that denotes when to trigger the scheduled query run. This can be a cron expression or a rate expression. </p>
    pub fn set_schedule_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schedule_expression = input;
        self
    }
    /// <p>An expression that denotes when to trigger the scheduled query run. This can be a cron expression or a rate expression. </p>
    pub fn get_schedule_expression(&self) -> &::std::option::Option<::std::string::String> {
        &self.schedule_expression
    }
    /// Consumes the builder and constructs a [`ScheduleConfiguration`](crate::types::ScheduleConfiguration).
    pub fn build(self) -> crate::types::ScheduleConfiguration {
        crate::types::ScheduleConfiguration {
            schedule_expression: self.schedule_expression,
        }
    }
}
