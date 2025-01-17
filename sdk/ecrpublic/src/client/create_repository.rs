// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRepository`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_repository_name): <p>The name to use for the repository. This appears publicly in the Amazon ECR Public Gallery. The repository name can be specified on its own (for example <code>nginx-web-app</code>) or prepended with a namespace to group the repository into a category (for example <code>project-a/nginx-web-app</code>).</p>
    ///   - [`catalog_data(RepositoryCatalogDataInput)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::catalog_data) / [`set_catalog_data(Option<RepositoryCatalogDataInput>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_catalog_data): <p>The details about the repository that are publicly visible in the Amazon ECR Public Gallery.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_tags): <p>The metadata that you apply to each repository to help categorize and organize your repositories. Each tag consists of a key and an optional value. You define both of them. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    /// - On success, responds with [`CreateRepositoryOutput`](crate::operation::create_repository::CreateRepositoryOutput) with field(s):
    ///   - [`repository(Option<Repository>)`](crate::operation::create_repository::CreateRepositoryOutput::repository): <p>The repository that was created.</p>
    ///   - [`catalog_data(Option<RepositoryCatalogData>)`](crate::operation::create_repository::CreateRepositoryOutput::catalog_data): <p>The catalog data for a repository. This data is publicly visible in the Amazon ECR Public Gallery.</p>
    /// - On failure, responds with [`SdkError<CreateRepositoryError>`](crate::operation::create_repository::CreateRepositoryError)
    pub fn create_repository(&self) -> crate::operation::create_repository::builders::CreateRepositoryFluentBuilder {
        crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::new(self.handle.clone())
    }
}
