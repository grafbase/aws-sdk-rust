// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::accept_reserved_instances_exchange_quote::_accept_reserved_instances_exchange_quote_output::AcceptReservedInstancesExchangeQuoteOutputBuilder;

pub use crate::operation::accept_reserved_instances_exchange_quote::_accept_reserved_instances_exchange_quote_input::AcceptReservedInstancesExchangeQuoteInputBuilder;

impl AcceptReservedInstancesExchangeQuoteInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.accept_reserved_instances_exchange_quote();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AcceptReservedInstancesExchangeQuote`.
///
/// <p>Accepts the Convertible Reserved Instance exchange quote described in the <code>GetReservedInstancesExchangeQuote</code> call.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AcceptReservedInstancesExchangeQuoteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::accept_reserved_instances_exchange_quote::builders::AcceptReservedInstancesExchangeQuoteInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AcceptReservedInstancesExchangeQuoteFluentBuilder {
    /// Creates a new `AcceptReservedInstancesExchangeQuote`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AcceptReservedInstancesExchangeQuote as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::accept_reserved_instances_exchange_quote::builders::AcceptReservedInstancesExchangeQuoteInputBuilder {
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
        crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuote::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuote::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput,
            crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteError>,
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
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// Appends an item to `ReservedInstanceIds`.
    ///
    /// To override the contents of this collection use [`set_reserved_instance_ids`](Self::set_reserved_instance_ids).
    ///
    /// <p>The IDs of the Convertible Reserved Instances to exchange for another Convertible Reserved Instance of the same or higher value.</p>
    pub fn reserved_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instance_ids(input.into());
        self
    }
    /// <p>The IDs of the Convertible Reserved Instances to exchange for another Convertible Reserved Instance of the same or higher value.</p>
    pub fn set_reserved_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_reserved_instance_ids(input);
        self
    }
    /// <p>The IDs of the Convertible Reserved Instances to exchange for another Convertible Reserved Instance of the same or higher value.</p>
    pub fn get_reserved_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_reserved_instance_ids()
    }
    /// Appends an item to `TargetConfigurations`.
    ///
    /// To override the contents of this collection use [`set_target_configurations`](Self::set_target_configurations).
    ///
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn target_configurations(mut self, input: crate::types::TargetConfigurationRequest) -> Self {
        self.inner = self.inner.target_configurations(input);
        self
    }
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn set_target_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TargetConfigurationRequest>>) -> Self {
        self.inner = self.inner.set_target_configurations(input);
        self
    }
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn get_target_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TargetConfigurationRequest>> {
        self.inner.get_target_configurations()
    }
}
