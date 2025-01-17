// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Creates a new flow. The request must include one source. The request optionally can include outputs (up to 50) and entitlements (up to 50).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateFlowInput {
    /// The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// The entitlements that you want to grant on a flow.
    pub entitlements: ::std::option::Option<::std::vec::Vec<crate::types::GrantEntitlementRequest>>,
    /// The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.
    pub media_streams: ::std::option::Option<::std::vec::Vec<crate::types::AddMediaStreamRequest>>,
    /// The name of the flow.
    pub name: ::std::option::Option<::std::string::String>,
    /// The outputs that you want to add to this flow.
    pub outputs: ::std::option::Option<::std::vec::Vec<crate::types::AddOutputRequest>>,
    /// The settings for the source of the flow.
    pub source: ::std::option::Option<crate::types::SetSourceRequest>,
    /// The settings for source failover.
    pub source_failover_config: ::std::option::Option<crate::types::FailoverConfig>,
    #[allow(missing_docs)] // documentation missing in model
    pub sources: ::std::option::Option<::std::vec::Vec<crate::types::SetSourceRequest>>,
    /// The VPC interfaces you want on the flow.
    pub vpc_interfaces: ::std::option::Option<::std::vec::Vec<crate::types::VpcInterfaceRequest>>,
    /// Create maintenance setting for a flow
    pub maintenance: ::std::option::Option<crate::types::AddMaintenance>,
}
impl CreateFlowInput {
    /// The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// The entitlements that you want to grant on a flow.
    pub fn entitlements(&self) -> ::std::option::Option<&[crate::types::GrantEntitlementRequest]> {
        self.entitlements.as_deref()
    }
    /// The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.
    pub fn media_streams(&self) -> ::std::option::Option<&[crate::types::AddMediaStreamRequest]> {
        self.media_streams.as_deref()
    }
    /// The name of the flow.
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// The outputs that you want to add to this flow.
    pub fn outputs(&self) -> ::std::option::Option<&[crate::types::AddOutputRequest]> {
        self.outputs.as_deref()
    }
    /// The settings for the source of the flow.
    pub fn source(&self) -> ::std::option::Option<&crate::types::SetSourceRequest> {
        self.source.as_ref()
    }
    /// The settings for source failover.
    pub fn source_failover_config(&self) -> ::std::option::Option<&crate::types::FailoverConfig> {
        self.source_failover_config.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn sources(&self) -> ::std::option::Option<&[crate::types::SetSourceRequest]> {
        self.sources.as_deref()
    }
    /// The VPC interfaces you want on the flow.
    pub fn vpc_interfaces(&self) -> ::std::option::Option<&[crate::types::VpcInterfaceRequest]> {
        self.vpc_interfaces.as_deref()
    }
    /// Create maintenance setting for a flow
    pub fn maintenance(&self) -> ::std::option::Option<&crate::types::AddMaintenance> {
        self.maintenance.as_ref()
    }
}
impl CreateFlowInput {
    /// Creates a new builder-style object to manufacture [`CreateFlowInput`](crate::operation::create_flow::CreateFlowInput).
    pub fn builder() -> crate::operation::create_flow::builders::CreateFlowInputBuilder {
        crate::operation::create_flow::builders::CreateFlowInputBuilder::default()
    }
}

/// A builder for [`CreateFlowInput`](crate::operation::create_flow::CreateFlowInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateFlowInputBuilder {
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) entitlements: ::std::option::Option<::std::vec::Vec<crate::types::GrantEntitlementRequest>>,
    pub(crate) media_streams: ::std::option::Option<::std::vec::Vec<crate::types::AddMediaStreamRequest>>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) outputs: ::std::option::Option<::std::vec::Vec<crate::types::AddOutputRequest>>,
    pub(crate) source: ::std::option::Option<crate::types::SetSourceRequest>,
    pub(crate) source_failover_config: ::std::option::Option<crate::types::FailoverConfig>,
    pub(crate) sources: ::std::option::Option<::std::vec::Vec<crate::types::SetSourceRequest>>,
    pub(crate) vpc_interfaces: ::std::option::Option<::std::vec::Vec<crate::types::VpcInterfaceRequest>>,
    pub(crate) maintenance: ::std::option::Option<crate::types::AddMaintenance>,
}
impl CreateFlowInputBuilder {
    /// The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone = input;
        self
    }
    /// The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone
    }
    /// Appends an item to `entitlements`.
    ///
    /// To override the contents of this collection use [`set_entitlements`](Self::set_entitlements).
    ///
    /// The entitlements that you want to grant on a flow.
    pub fn entitlements(mut self, input: crate::types::GrantEntitlementRequest) -> Self {
        let mut v = self.entitlements.unwrap_or_default();
        v.push(input);
        self.entitlements = ::std::option::Option::Some(v);
        self
    }
    /// The entitlements that you want to grant on a flow.
    pub fn set_entitlements(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GrantEntitlementRequest>>) -> Self {
        self.entitlements = input;
        self
    }
    /// The entitlements that you want to grant on a flow.
    pub fn get_entitlements(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GrantEntitlementRequest>> {
        &self.entitlements
    }
    /// Appends an item to `media_streams`.
    ///
    /// To override the contents of this collection use [`set_media_streams`](Self::set_media_streams).
    ///
    /// The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.
    pub fn media_streams(mut self, input: crate::types::AddMediaStreamRequest) -> Self {
        let mut v = self.media_streams.unwrap_or_default();
        v.push(input);
        self.media_streams = ::std::option::Option::Some(v);
        self
    }
    /// The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.
    pub fn set_media_streams(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddMediaStreamRequest>>) -> Self {
        self.media_streams = input;
        self
    }
    /// The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.
    pub fn get_media_streams(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddMediaStreamRequest>> {
        &self.media_streams
    }
    /// The name of the flow.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the flow.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// The name of the flow.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Appends an item to `outputs`.
    ///
    /// To override the contents of this collection use [`set_outputs`](Self::set_outputs).
    ///
    /// The outputs that you want to add to this flow.
    pub fn outputs(mut self, input: crate::types::AddOutputRequest) -> Self {
        let mut v = self.outputs.unwrap_or_default();
        v.push(input);
        self.outputs = ::std::option::Option::Some(v);
        self
    }
    /// The outputs that you want to add to this flow.
    pub fn set_outputs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddOutputRequest>>) -> Self {
        self.outputs = input;
        self
    }
    /// The outputs that you want to add to this flow.
    pub fn get_outputs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddOutputRequest>> {
        &self.outputs
    }
    /// The settings for the source of the flow.
    pub fn source(mut self, input: crate::types::SetSourceRequest) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }
    /// The settings for the source of the flow.
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::SetSourceRequest>) -> Self {
        self.source = input;
        self
    }
    /// The settings for the source of the flow.
    pub fn get_source(&self) -> &::std::option::Option<crate::types::SetSourceRequest> {
        &self.source
    }
    /// The settings for source failover.
    pub fn source_failover_config(mut self, input: crate::types::FailoverConfig) -> Self {
        self.source_failover_config = ::std::option::Option::Some(input);
        self
    }
    /// The settings for source failover.
    pub fn set_source_failover_config(mut self, input: ::std::option::Option<crate::types::FailoverConfig>) -> Self {
        self.source_failover_config = input;
        self
    }
    /// The settings for source failover.
    pub fn get_source_failover_config(&self) -> &::std::option::Option<crate::types::FailoverConfig> {
        &self.source_failover_config
    }
    /// Appends an item to `sources`.
    ///
    /// To override the contents of this collection use [`set_sources`](Self::set_sources).
    ///
    pub fn sources(mut self, input: crate::types::SetSourceRequest) -> Self {
        let mut v = self.sources.unwrap_or_default();
        v.push(input);
        self.sources = ::std::option::Option::Some(v);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_sources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SetSourceRequest>>) -> Self {
        self.sources = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_sources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SetSourceRequest>> {
        &self.sources
    }
    /// Appends an item to `vpc_interfaces`.
    ///
    /// To override the contents of this collection use [`set_vpc_interfaces`](Self::set_vpc_interfaces).
    ///
    /// The VPC interfaces you want on the flow.
    pub fn vpc_interfaces(mut self, input: crate::types::VpcInterfaceRequest) -> Self {
        let mut v = self.vpc_interfaces.unwrap_or_default();
        v.push(input);
        self.vpc_interfaces = ::std::option::Option::Some(v);
        self
    }
    /// The VPC interfaces you want on the flow.
    pub fn set_vpc_interfaces(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VpcInterfaceRequest>>) -> Self {
        self.vpc_interfaces = input;
        self
    }
    /// The VPC interfaces you want on the flow.
    pub fn get_vpc_interfaces(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpcInterfaceRequest>> {
        &self.vpc_interfaces
    }
    /// Create maintenance setting for a flow
    pub fn maintenance(mut self, input: crate::types::AddMaintenance) -> Self {
        self.maintenance = ::std::option::Option::Some(input);
        self
    }
    /// Create maintenance setting for a flow
    pub fn set_maintenance(mut self, input: ::std::option::Option<crate::types::AddMaintenance>) -> Self {
        self.maintenance = input;
        self
    }
    /// Create maintenance setting for a flow
    pub fn get_maintenance(&self) -> &::std::option::Option<crate::types::AddMaintenance> {
        &self.maintenance
    }
    /// Consumes the builder and constructs a [`CreateFlowInput`](crate::operation::create_flow::CreateFlowInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_flow::CreateFlowInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_flow::CreateFlowInput {
            availability_zone: self.availability_zone,
            entitlements: self.entitlements,
            media_streams: self.media_streams,
            name: self.name,
            outputs: self.outputs,
            source: self.source,
            source_failover_config: self.source_failover_config,
            sources: self.sources,
            vpc_interfaces: self.vpc_interfaces,
            maintenance: self.maintenance,
        })
    }
}
