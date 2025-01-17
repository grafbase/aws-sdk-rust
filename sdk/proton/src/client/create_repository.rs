// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRepository`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`provider(RepositoryProvider)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::provider) / [`set_provider(Option<RepositoryProvider>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_provider): <p>The repository provider.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_name): <p>The repository name (for example, <code>myrepos/myrepo</code>).</p>
    ///   - [`connection_arn(impl ::std::convert::Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::connection_arn) / [`set_connection_arn(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_connection_arn): <p>The Amazon Resource Name (ARN) of your AWS CodeStar connection that connects Proton to your repository provider account. For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/setting-up-for-service.html">Setting up for Proton</a> in the <i>Proton User Guide</i>.</p>
    ///   - [`encryption_key(impl ::std::convert::Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::encryption_key) / [`set_encryption_key(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_encryption_key): <p>The ARN of your customer Amazon Web Services Key Management Service (Amazon Web Services KMS) key.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_tags): <p>An optional list of metadata items that you can associate with the Proton repository. A tag is a key-value pair.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/resources.html">Proton resources and tagging</a> in the <i>Proton User Guide</i>.</p>
    /// - On success, responds with [`CreateRepositoryOutput`](crate::operation::create_repository::CreateRepositoryOutput) with field(s):
    ///   - [`repository(Option<Repository>)`](crate::operation::create_repository::CreateRepositoryOutput::repository): <p>The repository link's detail data that's returned by Proton.</p>
    /// - On failure, responds with [`SdkError<CreateRepositoryError>`](crate::operation::create_repository::CreateRepositoryError)
    pub fn create_repository(&self) -> crate::operation::create_repository::builders::CreateRepositoryFluentBuilder {
        crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::new(self.handle.clone())
    }
}
