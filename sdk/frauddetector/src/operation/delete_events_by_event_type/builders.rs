// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_events_by_event_type::_delete_events_by_event_type_output::DeleteEventsByEventTypeOutputBuilder;

pub use crate::operation::delete_events_by_event_type::_delete_events_by_event_type_input::DeleteEventsByEventTypeInputBuilder;

impl DeleteEventsByEventTypeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_events_by_event_type();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteEventsByEventType`.
///
/// <p>Deletes all events of a particular event type.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteEventsByEventTypeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_events_by_event_type::builders::DeleteEventsByEventTypeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteEventsByEventTypeFluentBuilder {
    /// Creates a new `DeleteEventsByEventType`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteEventsByEventType as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_events_by_event_type::builders::DeleteEventsByEventTypeInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_events_by_event_type::DeleteEventsByEventType::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_events_by_event_type::DeleteEventsByEventType::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeOutput,
            crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_events_by_event_type::DeleteEventsByEventTypeError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the event type.</p>
    pub fn event_type_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_type_name(input.into());
        self
    }
    /// <p>The name of the event type.</p>
    pub fn set_event_type_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_type_name(input);
        self
    }
    /// <p>The name of the event type.</p>
    pub fn get_event_type_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_type_name()
    }
}
