// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCapacityReservation`](crate::operation::delete_capacity_reservation::builders::DeleteCapacityReservationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_capacity_reservation::builders::DeleteCapacityReservationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_capacity_reservation::builders::DeleteCapacityReservationFluentBuilder::set_name): <p>The name of the capacity reservation to delete.</p>
    /// - On success, responds with [`DeleteCapacityReservationOutput`](crate::operation::delete_capacity_reservation::DeleteCapacityReservationOutput)
    /// - On failure, responds with [`SdkError<DeleteCapacityReservationError>`](crate::operation::delete_capacity_reservation::DeleteCapacityReservationError)
    pub fn delete_capacity_reservation(&self) -> crate::operation::delete_capacity_reservation::builders::DeleteCapacityReservationFluentBuilder {
        crate::operation::delete_capacity_reservation::builders::DeleteCapacityReservationFluentBuilder::new(self.handle.clone())
    }
}
