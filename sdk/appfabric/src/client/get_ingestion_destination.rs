// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetIngestionDestination`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_bundle_identifier(impl ::std::convert::Into<String>)`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::app_bundle_identifier) / [`set_app_bundle_identifier(Option<String>)`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::set_app_bundle_identifier): <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle to use for the request.</p>
    ///   - [`ingestion_identifier(impl ::std::convert::Into<String>)`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::ingestion_identifier) / [`set_ingestion_identifier(Option<String>)`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::set_ingestion_identifier): <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the ingestion to use for the request.</p>
    ///   - [`ingestion_destination_identifier(impl ::std::convert::Into<String>)`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::ingestion_destination_identifier) / [`set_ingestion_destination_identifier(Option<String>)`](crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::set_ingestion_destination_identifier): <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the ingestion destination to use for the request.</p>
    /// - On success, responds with [`GetIngestionDestinationOutput`](crate::operation::get_ingestion_destination::GetIngestionDestinationOutput) with field(s):
    ///   - [`ingestion_destination(Option<IngestionDestination>)`](crate::operation::get_ingestion_destination::GetIngestionDestinationOutput::ingestion_destination): <p>Contains information about an ingestion destination.</p>
    /// - On failure, responds with [`SdkError<GetIngestionDestinationError>`](crate::operation::get_ingestion_destination::GetIngestionDestinationError)
    pub fn get_ingestion_destination(&self) -> crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder {
        crate::operation::get_ingestion_destination::builders::GetIngestionDestinationFluentBuilder::new(self.handle.clone())
    }
}
