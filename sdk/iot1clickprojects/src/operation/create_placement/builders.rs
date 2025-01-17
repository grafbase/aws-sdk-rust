// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_placement::_create_placement_output::CreatePlacementOutputBuilder;

pub use crate::operation::create_placement::_create_placement_input::CreatePlacementInputBuilder;

impl CreatePlacementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_placement::CreatePlacementOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_placement::CreatePlacementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_placement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePlacement`.
///
/// <p>Creates an empty placement.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePlacementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_placement::builders::CreatePlacementInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreatePlacementFluentBuilder {
    /// Creates a new `CreatePlacement`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePlacement as a reference.
    pub fn as_input(&self) -> &crate::operation::create_placement::builders::CreatePlacementInputBuilder {
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
        crate::operation::create_placement::CreatePlacementOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_placement::CreatePlacementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_placement::CreatePlacement::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_placement::CreatePlacement::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_placement::CreatePlacementOutput,
            crate::operation::create_placement::CreatePlacementError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_placement::CreatePlacementError>,
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
    /// <p>The name of the placement to be created.</p>
    pub fn placement_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.placement_name(input.into());
        self
    }
    /// <p>The name of the placement to be created.</p>
    pub fn set_placement_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_placement_name(input);
        self
    }
    /// <p>The name of the placement to be created.</p>
    pub fn get_placement_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_placement_name()
    }
    /// <p>The name of the project in which to create the placement.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project in which to create the placement.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the project in which to create the placement.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_name()
    }
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>Optional user-defined key/value pairs providing contextual data (such as location or function) for the placement.</p>
    pub fn attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attributes(k.into(), v.into());
        self
    }
    /// <p>Optional user-defined key/value pairs providing contextual data (such as location or function) for the placement.</p>
    pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_attributes(input);
        self
    }
    /// <p>Optional user-defined key/value pairs providing contextual data (such as location or function) for the placement.</p>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_attributes()
    }
}
