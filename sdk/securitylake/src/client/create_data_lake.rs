// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDataLake`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configurations(Vec<DataLakeConfiguration>)`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::configurations) / [`set_configurations(Option<Vec<DataLakeConfiguration>>)`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::set_configurations): <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    ///   - [`meta_store_manager_role_arn(impl ::std::convert::Into<String>)`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::meta_store_manager_role_arn) / [`set_meta_store_manager_role_arn(Option<String>)`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::set_meta_store_manager_role_arn): <p>The Amazon Resource Name (ARN) used to create and update the Glue table. This table contains partitions generated by the ingestion and normalization of Amazon Web Services log sources and custom sources.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::set_tags): <p>An array of objects, one for each tag to associate with the data lake configuration. For each tag, you must specify both a tag key and a tag value. A tag value cannot be null, but it can be an empty string.</p>
    /// - On success, responds with [`CreateDataLakeOutput`](crate::operation::create_data_lake::CreateDataLakeOutput) with field(s):
    ///   - [`data_lakes(Option<Vec<DataLakeResource>>)`](crate::operation::create_data_lake::CreateDataLakeOutput::data_lakes): <p>The created Security Lake configuration object.</p>
    /// - On failure, responds with [`SdkError<CreateDataLakeError>`](crate::operation::create_data_lake::CreateDataLakeError)
    pub fn create_data_lake(&self) -> crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder {
        crate::operation::create_data_lake::builders::CreateDataLakeFluentBuilder::new(self.handle.clone())
    }
}
