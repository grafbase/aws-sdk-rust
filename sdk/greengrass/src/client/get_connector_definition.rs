// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConnectorDefinition`](crate::operation::get_connector_definition::builders::GetConnectorDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connector_definition_id(impl ::std::convert::Into<String>)`](crate::operation::get_connector_definition::builders::GetConnectorDefinitionFluentBuilder::connector_definition_id) / [`set_connector_definition_id(Option<String>)`](crate::operation::get_connector_definition::builders::GetConnectorDefinitionFluentBuilder::set_connector_definition_id): The ID of the connector definition.
    /// - On success, responds with [`GetConnectorDefinitionOutput`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::arn): The ARN of the definition.
    ///   - [`creation_timestamp(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the definition was created.
    ///   - [`id(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::id): The ID of the definition.
    ///   - [`last_updated_timestamp(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::last_updated_timestamp): The time, in milliseconds since the epoch, when the definition was last updated.
    ///   - [`latest_version(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::latest_version): The ID of the latest version associated with the definition.
    ///   - [`latest_version_arn(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::latest_version_arn): The ARN of the latest version associated with the definition.
    ///   - [`name(Option<String>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::name): The name of the definition.
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::get_connector_definition::GetConnectorDefinitionOutput::tags): Tag(s) attached to the resource arn.
    /// - On failure, responds with [`SdkError<GetConnectorDefinitionError>`](crate::operation::get_connector_definition::GetConnectorDefinitionError)
    pub fn get_connector_definition(&self) -> crate::operation::get_connector_definition::builders::GetConnectorDefinitionFluentBuilder {
        crate::operation::get_connector_definition::builders::GetConnectorDefinitionFluentBuilder::new(self.handle.clone())
    }
}
