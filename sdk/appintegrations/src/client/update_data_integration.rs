// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDataIntegration`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl ::std::convert::Into<String>)`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::set_identifier): <p>A unique identifier for the DataIntegration.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::set_name): <p>The name of the DataIntegration.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::set_description): <p>A description of the DataIntegration.</p>
    /// - On success, responds with [`UpdateDataIntegrationOutput`](crate::operation::update_data_integration::UpdateDataIntegrationOutput)
    /// - On failure, responds with [`SdkError<UpdateDataIntegrationError>`](crate::operation::update_data_integration::UpdateDataIntegrationError)
    pub fn update_data_integration(&self) -> crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder {
        crate::operation::update_data_integration::builders::UpdateDataIntegrationFluentBuilder::new(self.handle.clone())
    }
}
