// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Feedback for an anomalous metric.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AnomalyGroupTimeSeriesFeedback {
    /// <p>The ID of the anomaly group.</p>
    pub anomaly_group_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the metric.</p>
    pub time_series_id: ::std::option::Option<::std::string::String>,
    /// <p>Feedback on whether the metric is a legitimate anomaly.</p>
    pub is_anomaly: ::std::option::Option<bool>,
}
impl AnomalyGroupTimeSeriesFeedback {
    /// <p>The ID of the anomaly group.</p>
    pub fn anomaly_group_id(&self) -> ::std::option::Option<&str> {
        self.anomaly_group_id.as_deref()
    }
    /// <p>The ID of the metric.</p>
    pub fn time_series_id(&self) -> ::std::option::Option<&str> {
        self.time_series_id.as_deref()
    }
    /// <p>Feedback on whether the metric is a legitimate anomaly.</p>
    pub fn is_anomaly(&self) -> ::std::option::Option<bool> {
        self.is_anomaly
    }
}
impl AnomalyGroupTimeSeriesFeedback {
    /// Creates a new builder-style object to manufacture [`AnomalyGroupTimeSeriesFeedback`](crate::types::AnomalyGroupTimeSeriesFeedback).
    pub fn builder() -> crate::types::builders::AnomalyGroupTimeSeriesFeedbackBuilder {
        crate::types::builders::AnomalyGroupTimeSeriesFeedbackBuilder::default()
    }
}

/// A builder for [`AnomalyGroupTimeSeriesFeedback`](crate::types::AnomalyGroupTimeSeriesFeedback).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AnomalyGroupTimeSeriesFeedbackBuilder {
    pub(crate) anomaly_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) time_series_id: ::std::option::Option<::std::string::String>,
    pub(crate) is_anomaly: ::std::option::Option<bool>,
}
impl AnomalyGroupTimeSeriesFeedbackBuilder {
    /// <p>The ID of the anomaly group.</p>
    pub fn anomaly_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.anomaly_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the anomaly group.</p>
    pub fn set_anomaly_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.anomaly_group_id = input;
        self
    }
    /// <p>The ID of the anomaly group.</p>
    pub fn get_anomaly_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.anomaly_group_id
    }
    /// <p>The ID of the metric.</p>
    pub fn time_series_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.time_series_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the metric.</p>
    pub fn set_time_series_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.time_series_id = input;
        self
    }
    /// <p>The ID of the metric.</p>
    pub fn get_time_series_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.time_series_id
    }
    /// <p>Feedback on whether the metric is a legitimate anomaly.</p>
    pub fn is_anomaly(mut self, input: bool) -> Self {
        self.is_anomaly = ::std::option::Option::Some(input);
        self
    }
    /// <p>Feedback on whether the metric is a legitimate anomaly.</p>
    pub fn set_is_anomaly(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_anomaly = input;
        self
    }
    /// <p>Feedback on whether the metric is a legitimate anomaly.</p>
    pub fn get_is_anomaly(&self) -> &::std::option::Option<bool> {
        &self.is_anomaly
    }
    /// Consumes the builder and constructs a [`AnomalyGroupTimeSeriesFeedback`](crate::types::AnomalyGroupTimeSeriesFeedback).
    pub fn build(self) -> crate::types::AnomalyGroupTimeSeriesFeedback {
        crate::types::AnomalyGroupTimeSeriesFeedback {
            anomaly_group_id: self.anomaly_group_id,
            time_series_id: self.time_series_id,
            is_anomaly: self.is_anomaly,
        }
    }
}
