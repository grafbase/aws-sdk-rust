// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The time range during which an Amazon Web Services event occurred. Amazon Web Services resource events and metrics are analyzed by DevOps Guru to find anomalous behavior and provide recommendations to improve your operational solutions. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EventTimeRange {
    /// <p> The time when the event started. </p>
    pub from_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The time when the event ended. </p>
    pub to_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl EventTimeRange {
    /// <p> The time when the event started. </p>
    pub fn from_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.from_time.as_ref()
    }
    /// <p> The time when the event ended. </p>
    pub fn to_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.to_time.as_ref()
    }
}
impl EventTimeRange {
    /// Creates a new builder-style object to manufacture [`EventTimeRange`](crate::types::EventTimeRange).
    pub fn builder() -> crate::types::builders::EventTimeRangeBuilder {
        crate::types::builders::EventTimeRangeBuilder::default()
    }
}

/// A builder for [`EventTimeRange`](crate::types::EventTimeRange).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EventTimeRangeBuilder {
    pub(crate) from_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) to_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl EventTimeRangeBuilder {
    /// <p> The time when the event started. </p>
    pub fn from_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.from_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time when the event started. </p>
    pub fn set_from_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.from_time = input;
        self
    }
    /// <p> The time when the event started. </p>
    pub fn get_from_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.from_time
    }
    /// <p> The time when the event ended. </p>
    pub fn to_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.to_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time when the event ended. </p>
    pub fn set_to_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.to_time = input;
        self
    }
    /// <p> The time when the event ended. </p>
    pub fn get_to_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.to_time
    }
    /// Consumes the builder and constructs a [`EventTimeRange`](crate::types::EventTimeRange).
    pub fn build(self) -> crate::types::EventTimeRange {
        crate::types::EventTimeRange {
            from_time: self.from_time,
            to_time: self.to_time,
        }
    }
}
