// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A reservation for a specified number of data processing units (DPUs). When a reservation is initially created, it has no DPUs. Athena allocates DPUs until the allocated amount equals the requested amount.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CapacityReservation {
    /// <p>The name of the capacity reservation.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The status of the capacity reservation.</p>
    pub status: ::std::option::Option<crate::types::CapacityReservationStatus>,
    /// <p>The number of data processing units requested.</p>
    pub target_dpus: ::std::option::Option<i32>,
    /// <p>The number of data processing units currently allocated.</p>
    pub allocated_dpus: ::std::option::Option<i32>,
    /// <p>Contains the submission time of a single allocation request for a capacity reservation and the most recent status of the attempted allocation.</p>
    pub last_allocation: ::std::option::Option<crate::types::CapacityAllocation>,
    /// <p>The time of the most recent capacity allocation that succeeded.</p>
    pub last_successful_allocation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time in UTC epoch millis when the capacity reservation was created.</p>
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CapacityReservation {
    /// <p>The name of the capacity reservation.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The status of the capacity reservation.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::CapacityReservationStatus> {
        self.status.as_ref()
    }
    /// <p>The number of data processing units requested.</p>
    pub fn target_dpus(&self) -> ::std::option::Option<i32> {
        self.target_dpus
    }
    /// <p>The number of data processing units currently allocated.</p>
    pub fn allocated_dpus(&self) -> ::std::option::Option<i32> {
        self.allocated_dpus
    }
    /// <p>Contains the submission time of a single allocation request for a capacity reservation and the most recent status of the attempted allocation.</p>
    pub fn last_allocation(&self) -> ::std::option::Option<&crate::types::CapacityAllocation> {
        self.last_allocation.as_ref()
    }
    /// <p>The time of the most recent capacity allocation that succeeded.</p>
    pub fn last_successful_allocation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_successful_allocation_time.as_ref()
    }
    /// <p>The time in UTC epoch millis when the capacity reservation was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
}
impl CapacityReservation {
    /// Creates a new builder-style object to manufacture [`CapacityReservation`](crate::types::CapacityReservation).
    pub fn builder() -> crate::types::builders::CapacityReservationBuilder {
        crate::types::builders::CapacityReservationBuilder::default()
    }
}

/// A builder for [`CapacityReservation`](crate::types::CapacityReservation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CapacityReservationBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::CapacityReservationStatus>,
    pub(crate) target_dpus: ::std::option::Option<i32>,
    pub(crate) allocated_dpus: ::std::option::Option<i32>,
    pub(crate) last_allocation: ::std::option::Option<crate::types::CapacityAllocation>,
    pub(crate) last_successful_allocation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CapacityReservationBuilder {
    /// <p>The name of the capacity reservation.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the capacity reservation.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the capacity reservation.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The status of the capacity reservation.</p>
    pub fn status(mut self, input: crate::types::CapacityReservationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the capacity reservation.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::CapacityReservationStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the capacity reservation.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::CapacityReservationStatus> {
        &self.status
    }
    /// <p>The number of data processing units requested.</p>
    pub fn target_dpus(mut self, input: i32) -> Self {
        self.target_dpus = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of data processing units requested.</p>
    pub fn set_target_dpus(mut self, input: ::std::option::Option<i32>) -> Self {
        self.target_dpus = input;
        self
    }
    /// <p>The number of data processing units requested.</p>
    pub fn get_target_dpus(&self) -> &::std::option::Option<i32> {
        &self.target_dpus
    }
    /// <p>The number of data processing units currently allocated.</p>
    pub fn allocated_dpus(mut self, input: i32) -> Self {
        self.allocated_dpus = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of data processing units currently allocated.</p>
    pub fn set_allocated_dpus(mut self, input: ::std::option::Option<i32>) -> Self {
        self.allocated_dpus = input;
        self
    }
    /// <p>The number of data processing units currently allocated.</p>
    pub fn get_allocated_dpus(&self) -> &::std::option::Option<i32> {
        &self.allocated_dpus
    }
    /// <p>Contains the submission time of a single allocation request for a capacity reservation and the most recent status of the attempted allocation.</p>
    pub fn last_allocation(mut self, input: crate::types::CapacityAllocation) -> Self {
        self.last_allocation = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains the submission time of a single allocation request for a capacity reservation and the most recent status of the attempted allocation.</p>
    pub fn set_last_allocation(mut self, input: ::std::option::Option<crate::types::CapacityAllocation>) -> Self {
        self.last_allocation = input;
        self
    }
    /// <p>Contains the submission time of a single allocation request for a capacity reservation and the most recent status of the attempted allocation.</p>
    pub fn get_last_allocation(&self) -> &::std::option::Option<crate::types::CapacityAllocation> {
        &self.last_allocation
    }
    /// <p>The time of the most recent capacity allocation that succeeded.</p>
    pub fn last_successful_allocation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_successful_allocation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time of the most recent capacity allocation that succeeded.</p>
    pub fn set_last_successful_allocation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_successful_allocation_time = input;
        self
    }
    /// <p>The time of the most recent capacity allocation that succeeded.</p>
    pub fn get_last_successful_allocation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_successful_allocation_time
    }
    /// <p>The time in UTC epoch millis when the capacity reservation was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time in UTC epoch millis when the capacity reservation was created.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The time in UTC epoch millis when the capacity reservation was created.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// Consumes the builder and constructs a [`CapacityReservation`](crate::types::CapacityReservation).
    pub fn build(self) -> crate::types::CapacityReservation {
        crate::types::CapacityReservation {
            name: self.name,
            status: self.status,
            target_dpus: self.target_dpus,
            allocated_dpus: self.allocated_dpus,
            last_allocation: self.last_allocation,
            last_successful_allocation_time: self.last_successful_allocation_time,
            creation_time: self.creation_time,
        }
    }
}
