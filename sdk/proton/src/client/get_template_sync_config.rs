// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTemplateSyncConfig`](crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_name(impl ::std::convert::Into<String>)`](crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder::template_name) / [`set_template_name(Option<String>)`](crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder::set_template_name): <p>The template name.</p>
    ///   - [`template_type(TemplateType)`](crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder::template_type) / [`set_template_type(Option<TemplateType>)`](crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder::set_template_type): <p>The template type.</p>
    /// - On success, responds with [`GetTemplateSyncConfigOutput`](crate::operation::get_template_sync_config::GetTemplateSyncConfigOutput) with field(s):
    ///   - [`template_sync_config(Option<TemplateSyncConfig>)`](crate::operation::get_template_sync_config::GetTemplateSyncConfigOutput::template_sync_config): <p>The template sync configuration detail data that's returned by Proton.</p>
    /// - On failure, responds with [`SdkError<GetTemplateSyncConfigError>`](crate::operation::get_template_sync_config::GetTemplateSyncConfigError)
    pub fn get_template_sync_config(&self) -> crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder {
        crate::operation::get_template_sync_config::builders::GetTemplateSyncConfigFluentBuilder::new(self.handle.clone())
    }
}
