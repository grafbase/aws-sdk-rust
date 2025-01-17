// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSettings`](crate::operation::update_settings::builders::UpdateSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl ::std::convert::Into<String>)`](crate::operation::update_settings::builders::UpdateSettingsFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::update_settings::builders::UpdateSettingsFluentBuilder::set_directory_id): <p>The identifier of the directory for which to update settings.</p>
    ///   - [`settings(Vec<Setting>)`](crate::operation::update_settings::builders::UpdateSettingsFluentBuilder::settings) / [`set_settings(Option<Vec<Setting>>)`](crate::operation::update_settings::builders::UpdateSettingsFluentBuilder::set_settings): <p>The list of <code>Setting</code> objects.</p>
    /// - On success, responds with [`UpdateSettingsOutput`](crate::operation::update_settings::UpdateSettingsOutput) with field(s):
    ///   - [`directory_id(Option<String>)`](crate::operation::update_settings::UpdateSettingsOutput::directory_id): <p>The identifier of the directory.</p>
    /// - On failure, responds with [`SdkError<UpdateSettingsError>`](crate::operation::update_settings::UpdateSettingsError)
    pub fn update_settings(&self) -> crate::operation::update_settings::builders::UpdateSettingsFluentBuilder {
        crate::operation::update_settings::builders::UpdateSettingsFluentBuilder::new(self.handle.clone())
    }
}
