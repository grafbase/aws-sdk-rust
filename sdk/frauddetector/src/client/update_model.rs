// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateModel`](crate::operation::update_model::builders::UpdateModelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`model_id(impl ::std::convert::Into<String>)`](crate::operation::update_model::builders::UpdateModelFluentBuilder::model_id) / [`set_model_id(Option<String>)`](crate::operation::update_model::builders::UpdateModelFluentBuilder::set_model_id): <p>The model ID.</p>
    ///   - [`model_type(ModelTypeEnum)`](crate::operation::update_model::builders::UpdateModelFluentBuilder::model_type) / [`set_model_type(Option<ModelTypeEnum>)`](crate::operation::update_model::builders::UpdateModelFluentBuilder::set_model_type): <p>The model type.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_model::builders::UpdateModelFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_model::builders::UpdateModelFluentBuilder::set_description): <p>The new model description.</p>
    /// - On success, responds with [`UpdateModelOutput`](crate::operation::update_model::UpdateModelOutput)
    /// - On failure, responds with [`SdkError<UpdateModelError>`](crate::operation::update_model::UpdateModelError)
    pub fn update_model(&self) -> crate::operation::update_model::builders::UpdateModelFluentBuilder {
        crate::operation::update_model::builders::UpdateModelFluentBuilder::new(self.handle.clone())
    }
}
