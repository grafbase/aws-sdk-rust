// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDataset`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_name): <p>The name of the dataset to be created. Valid characters are alphanumeric (A-Z, a-z, 0-9), hyphen (-), period (.), and space.</p>
    ///   - [`format(InputFormat)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::format) / [`set_format(Option<InputFormat>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_format): <p>The file format of a dataset that is created from an Amazon S3 file or folder.</p>
    ///   - [`format_options(FormatOptions)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::format_options) / [`set_format_options(Option<FormatOptions>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_format_options): <p>Represents a set of options that define the structure of either comma-separated value (CSV), Excel, or JSON input.</p>
    ///   - [`input(Input)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::input) / [`set_input(Option<Input>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_input): <p>Represents information on how DataBrew can find data, in either the Glue Data Catalog or Amazon S3.</p>
    ///   - [`path_options(PathOptions)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::path_options) / [`set_path_options(Option<PathOptions>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_path_options): <p>A set of options that defines how DataBrew interprets an Amazon S3 path of the dataset.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_tags): <p>Metadata tags to apply to this dataset.</p>
    /// - On success, responds with [`CreateDatasetOutput`](crate::operation::create_dataset::CreateDatasetOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::create_dataset::CreateDatasetOutput::name): <p>The name of the dataset that you created.</p>
    /// - On failure, responds with [`SdkError<CreateDatasetError>`](crate::operation::create_dataset::CreateDatasetError)
    pub fn create_dataset(&self) -> crate::operation::create_dataset::builders::CreateDatasetFluentBuilder {
        crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::new(self.handle.clone())
    }
}
