// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An instance of EventStream in a list of EventStreams.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EventStreamSummary {
    /// <p>The unique name of the domain.</p>
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the event stream.</p>
    pub event_stream_name: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the event stream.</p>
    pub event_stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The operational state of destination stream for export.</p>
    pub state: ::std::option::Option<crate::types::EventStreamState>,
    /// <p>The timestamp when the <code>State</code> changed to <code>STOPPED</code>.</p>
    pub stopped_since: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Summary information about the Kinesis data stream.</p>
    pub destination_summary: ::std::option::Option<crate::types::DestinationSummary>,
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl EventStreamSummary {
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>The name of the event stream.</p>
    pub fn event_stream_name(&self) -> ::std::option::Option<&str> {
        self.event_stream_name.as_deref()
    }
    /// <p>A unique identifier for the event stream.</p>
    pub fn event_stream_arn(&self) -> ::std::option::Option<&str> {
        self.event_stream_arn.as_deref()
    }
    /// <p>The operational state of destination stream for export.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::EventStreamState> {
        self.state.as_ref()
    }
    /// <p>The timestamp when the <code>State</code> changed to <code>STOPPED</code>.</p>
    pub fn stopped_since(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.stopped_since.as_ref()
    }
    /// <p>Summary information about the Kinesis data stream.</p>
    pub fn destination_summary(&self) -> ::std::option::Option<&crate::types::DestinationSummary> {
        self.destination_summary.as_ref()
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl EventStreamSummary {
    /// Creates a new builder-style object to manufacture [`EventStreamSummary`](crate::types::EventStreamSummary).
    pub fn builder() -> crate::types::builders::EventStreamSummaryBuilder {
        crate::types::builders::EventStreamSummaryBuilder::default()
    }
}

/// A builder for [`EventStreamSummary`](crate::types::EventStreamSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EventStreamSummaryBuilder {
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) event_stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) event_stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::EventStreamState>,
    pub(crate) stopped_since: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) destination_summary: ::std::option::Option<crate::types::DestinationSummary>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl EventStreamSummaryBuilder {
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_name
    }
    /// <p>The name of the event stream.</p>
    pub fn event_stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the event stream.</p>
    pub fn set_event_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_stream_name = input;
        self
    }
    /// <p>The name of the event stream.</p>
    pub fn get_event_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_stream_name
    }
    /// <p>A unique identifier for the event stream.</p>
    pub fn event_stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the event stream.</p>
    pub fn set_event_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_stream_arn = input;
        self
    }
    /// <p>A unique identifier for the event stream.</p>
    pub fn get_event_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_stream_arn
    }
    /// <p>The operational state of destination stream for export.</p>
    pub fn state(mut self, input: crate::types::EventStreamState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The operational state of destination stream for export.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::EventStreamState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The operational state of destination stream for export.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::EventStreamState> {
        &self.state
    }
    /// <p>The timestamp when the <code>State</code> changed to <code>STOPPED</code>.</p>
    pub fn stopped_since(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.stopped_since = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the <code>State</code> changed to <code>STOPPED</code>.</p>
    pub fn set_stopped_since(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.stopped_since = input;
        self
    }
    /// <p>The timestamp when the <code>State</code> changed to <code>STOPPED</code>.</p>
    pub fn get_stopped_since(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.stopped_since
    }
    /// <p>Summary information about the Kinesis data stream.</p>
    pub fn destination_summary(mut self, input: crate::types::DestinationSummary) -> Self {
        self.destination_summary = ::std::option::Option::Some(input);
        self
    }
    /// <p>Summary information about the Kinesis data stream.</p>
    pub fn set_destination_summary(mut self, input: ::std::option::Option<crate::types::DestinationSummary>) -> Self {
        self.destination_summary = input;
        self
    }
    /// <p>Summary information about the Kinesis data stream.</p>
    pub fn get_destination_summary(&self) -> &::std::option::Option<crate::types::DestinationSummary> {
        &self.destination_summary
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`EventStreamSummary`](crate::types::EventStreamSummary).
    pub fn build(self) -> crate::types::EventStreamSummary {
        crate::types::EventStreamSummary {
            domain_name: self.domain_name,
            event_stream_name: self.event_stream_name,
            event_stream_arn: self.event_stream_arn,
            state: self.state,
            stopped_since: self.stopped_since,
            destination_summary: self.destination_summary,
            tags: self.tags,
        }
    }
}
