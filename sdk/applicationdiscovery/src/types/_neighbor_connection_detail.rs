// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about neighboring servers.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NeighborConnectionDetail {
    /// <p>The ID of the server that opened the network connection.</p>
    pub source_server_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the server that accepted the network connection.</p>
    pub destination_server_id: ::std::option::Option<::std::string::String>,
    /// <p>The destination network port for the connection.</p>
    pub destination_port: ::std::option::Option<i32>,
    /// <p>The network protocol used for the connection.</p>
    pub transport_protocol: ::std::option::Option<::std::string::String>,
    /// <p>The number of open network connections with the neighboring server.</p>
    pub connections_count: i64,
}
impl NeighborConnectionDetail {
    /// <p>The ID of the server that opened the network connection.</p>
    pub fn source_server_id(&self) -> ::std::option::Option<&str> {
        self.source_server_id.as_deref()
    }
    /// <p>The ID of the server that accepted the network connection.</p>
    pub fn destination_server_id(&self) -> ::std::option::Option<&str> {
        self.destination_server_id.as_deref()
    }
    /// <p>The destination network port for the connection.</p>
    pub fn destination_port(&self) -> ::std::option::Option<i32> {
        self.destination_port
    }
    /// <p>The network protocol used for the connection.</p>
    pub fn transport_protocol(&self) -> ::std::option::Option<&str> {
        self.transport_protocol.as_deref()
    }
    /// <p>The number of open network connections with the neighboring server.</p>
    pub fn connections_count(&self) -> i64 {
        self.connections_count
    }
}
impl NeighborConnectionDetail {
    /// Creates a new builder-style object to manufacture [`NeighborConnectionDetail`](crate::types::NeighborConnectionDetail).
    pub fn builder() -> crate::types::builders::NeighborConnectionDetailBuilder {
        crate::types::builders::NeighborConnectionDetailBuilder::default()
    }
}

/// A builder for [`NeighborConnectionDetail`](crate::types::NeighborConnectionDetail).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct NeighborConnectionDetailBuilder {
    pub(crate) source_server_id: ::std::option::Option<::std::string::String>,
    pub(crate) destination_server_id: ::std::option::Option<::std::string::String>,
    pub(crate) destination_port: ::std::option::Option<i32>,
    pub(crate) transport_protocol: ::std::option::Option<::std::string::String>,
    pub(crate) connections_count: ::std::option::Option<i64>,
}
impl NeighborConnectionDetailBuilder {
    /// <p>The ID of the server that opened the network connection.</p>
    pub fn source_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_server_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the server that opened the network connection.</p>
    pub fn set_source_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_server_id = input;
        self
    }
    /// <p>The ID of the server that opened the network connection.</p>
    pub fn get_source_server_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_server_id
    }
    /// <p>The ID of the server that accepted the network connection.</p>
    pub fn destination_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_server_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the server that accepted the network connection.</p>
    pub fn set_destination_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_server_id = input;
        self
    }
    /// <p>The ID of the server that accepted the network connection.</p>
    pub fn get_destination_server_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_server_id
    }
    /// <p>The destination network port for the connection.</p>
    pub fn destination_port(mut self, input: i32) -> Self {
        self.destination_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination network port for the connection.</p>
    pub fn set_destination_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.destination_port = input;
        self
    }
    /// <p>The destination network port for the connection.</p>
    pub fn get_destination_port(&self) -> &::std::option::Option<i32> {
        &self.destination_port
    }
    /// <p>The network protocol used for the connection.</p>
    pub fn transport_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transport_protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The network protocol used for the connection.</p>
    pub fn set_transport_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transport_protocol = input;
        self
    }
    /// <p>The network protocol used for the connection.</p>
    pub fn get_transport_protocol(&self) -> &::std::option::Option<::std::string::String> {
        &self.transport_protocol
    }
    /// <p>The number of open network connections with the neighboring server.</p>
    pub fn connections_count(mut self, input: i64) -> Self {
        self.connections_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of open network connections with the neighboring server.</p>
    pub fn set_connections_count(mut self, input: ::std::option::Option<i64>) -> Self {
        self.connections_count = input;
        self
    }
    /// <p>The number of open network connections with the neighboring server.</p>
    pub fn get_connections_count(&self) -> &::std::option::Option<i64> {
        &self.connections_count
    }
    /// Consumes the builder and constructs a [`NeighborConnectionDetail`](crate::types::NeighborConnectionDetail).
    pub fn build(self) -> crate::types::NeighborConnectionDetail {
        crate::types::NeighborConnectionDetail {
            source_server_id: self.source_server_id,
            destination_server_id: self.destination_server_id,
            destination_port: self.destination_port,
            transport_protocol: self.transport_protocol,
            connections_count: self.connections_count.unwrap_or_default(),
        }
    }
}
