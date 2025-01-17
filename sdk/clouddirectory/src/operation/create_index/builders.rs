// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_index::_create_index_output::CreateIndexOutputBuilder;

pub use crate::operation::create_index::_create_index_input::CreateIndexInputBuilder;

impl CreateIndexInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_index::CreateIndexOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_index::CreateIndexError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_index();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateIndex`.
///
/// <p>Creates an index object. See <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/indexing_search.html">Indexing and search</a> for more information.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateIndexFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_index::builders::CreateIndexInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateIndexFluentBuilder {
    /// Creates a new `CreateIndex`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateIndex as a reference.
    pub fn as_input(&self) -> &crate::operation::create_index::builders::CreateIndexInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_index::CreateIndexOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_index::CreateIndexError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_index::CreateIndex::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_index::CreateIndex::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_index::CreateIndexOutput,
            crate::operation::create_index::CreateIndexError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_index::CreateIndexError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ARN of the directory where the index should be created.</p>
    pub fn directory_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_arn(input.into());
        self
    }
    /// <p>The ARN of the directory where the index should be created.</p>
    pub fn set_directory_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_arn(input);
        self
    }
    /// <p>The ARN of the directory where the index should be created.</p>
    pub fn get_directory_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_arn()
    }
    /// Appends an item to `OrderedIndexedAttributeList`.
    ///
    /// To override the contents of this collection use [`set_ordered_indexed_attribute_list`](Self::set_ordered_indexed_attribute_list).
    ///
    /// <p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>
    pub fn ordered_indexed_attribute_list(mut self, input: crate::types::AttributeKey) -> Self {
        self.inner = self.inner.ordered_indexed_attribute_list(input);
        self
    }
    /// <p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>
    pub fn set_ordered_indexed_attribute_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeKey>>) -> Self {
        self.inner = self.inner.set_ordered_indexed_attribute_list(input);
        self
    }
    /// <p>Specifies the attributes that should be indexed on. Currently only a single attribute is supported.</p>
    pub fn get_ordered_indexed_attribute_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AttributeKey>> {
        self.inner.get_ordered_indexed_attribute_list()
    }
    /// <p>Indicates whether the attribute that is being indexed has unique values or not.</p>
    pub fn is_unique(mut self, input: bool) -> Self {
        self.inner = self.inner.is_unique(input);
        self
    }
    /// <p>Indicates whether the attribute that is being indexed has unique values or not.</p>
    pub fn set_is_unique(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_is_unique(input);
        self
    }
    /// <p>Indicates whether the attribute that is being indexed has unique values or not.</p>
    pub fn get_is_unique(&self) -> &::std::option::Option<bool> {
        self.inner.get_is_unique()
    }
    /// <p>A reference to the parent object that contains the index object.</p>
    pub fn parent_reference(mut self, input: crate::types::ObjectReference) -> Self {
        self.inner = self.inner.parent_reference(input);
        self
    }
    /// <p>A reference to the parent object that contains the index object.</p>
    pub fn set_parent_reference(mut self, input: ::std::option::Option<crate::types::ObjectReference>) -> Self {
        self.inner = self.inner.set_parent_reference(input);
        self
    }
    /// <p>A reference to the parent object that contains the index object.</p>
    pub fn get_parent_reference(&self) -> &::std::option::Option<crate::types::ObjectReference> {
        self.inner.get_parent_reference()
    }
    /// <p>The name of the link between the parent object and the index object.</p>
    pub fn link_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.link_name(input.into());
        self
    }
    /// <p>The name of the link between the parent object and the index object.</p>
    pub fn set_link_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_link_name(input);
        self
    }
    /// <p>The name of the link between the parent object and the index object.</p>
    pub fn get_link_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_link_name()
    }
}
