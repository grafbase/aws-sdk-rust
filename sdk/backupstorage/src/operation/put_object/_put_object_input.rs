// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub struct PutObjectInput {
    /// Backup job Id for the in-progress backup.
    pub backup_job_id: ::std::option::Option<::std::string::String>,
    /// The name of the Object to be uploaded.
    pub object_name: ::std::option::Option<::std::string::String>,
    /// Store user defined metadata like backup checksum, disk ids, restore metadata etc.
    pub metadata_string: ::std::option::Option<::std::string::String>,
    /// Inline chunk data to be uploaded.
    pub inline_chunk: ::aws_smithy_http::byte_stream::ByteStream,
    /// Length of the inline chunk data.
    pub inline_chunk_length: i64,
    /// Inline chunk checksum
    pub inline_chunk_checksum: ::std::option::Option<::std::string::String>,
    /// Inline chunk checksum algorithm
    pub inline_chunk_checksum_algorithm: ::std::option::Option<::std::string::String>,
    /// object checksum
    pub object_checksum: ::std::option::Option<::std::string::String>,
    /// object checksum algorithm
    pub object_checksum_algorithm: ::std::option::Option<crate::types::SummaryChecksumAlgorithm>,
    /// Throw an exception if Object name is already exist.
    pub throw_on_duplicate: bool,
}
impl PutObjectInput {
    /// Backup job Id for the in-progress backup.
    pub fn backup_job_id(&self) -> ::std::option::Option<&str> {
        self.backup_job_id.as_deref()
    }
    /// The name of the Object to be uploaded.
    pub fn object_name(&self) -> ::std::option::Option<&str> {
        self.object_name.as_deref()
    }
    /// Store user defined metadata like backup checksum, disk ids, restore metadata etc.
    pub fn metadata_string(&self) -> ::std::option::Option<&str> {
        self.metadata_string.as_deref()
    }
    /// Inline chunk data to be uploaded.
    pub fn inline_chunk(&self) -> &::aws_smithy_http::byte_stream::ByteStream {
        &self.inline_chunk
    }
    /// Length of the inline chunk data.
    pub fn inline_chunk_length(&self) -> i64 {
        self.inline_chunk_length
    }
    /// Inline chunk checksum
    pub fn inline_chunk_checksum(&self) -> ::std::option::Option<&str> {
        self.inline_chunk_checksum.as_deref()
    }
    /// Inline chunk checksum algorithm
    pub fn inline_chunk_checksum_algorithm(&self) -> ::std::option::Option<&str> {
        self.inline_chunk_checksum_algorithm.as_deref()
    }
    /// object checksum
    pub fn object_checksum(&self) -> ::std::option::Option<&str> {
        self.object_checksum.as_deref()
    }
    /// object checksum algorithm
    pub fn object_checksum_algorithm(&self) -> ::std::option::Option<&crate::types::SummaryChecksumAlgorithm> {
        self.object_checksum_algorithm.as_ref()
    }
    /// Throw an exception if Object name is already exist.
    pub fn throw_on_duplicate(&self) -> bool {
        self.throw_on_duplicate
    }
}
impl PutObjectInput {
    /// Creates a new builder-style object to manufacture [`PutObjectInput`](crate::operation::put_object::PutObjectInput).
    pub fn builder() -> crate::operation::put_object::builders::PutObjectInputBuilder {
        crate::operation::put_object::builders::PutObjectInputBuilder::default()
    }
}

/// A builder for [`PutObjectInput`](crate::operation::put_object::PutObjectInput).
#[non_exhaustive]
#[derive(::std::default::Default, ::std::fmt::Debug)]
pub struct PutObjectInputBuilder {
    pub(crate) backup_job_id: ::std::option::Option<::std::string::String>,
    pub(crate) object_name: ::std::option::Option<::std::string::String>,
    pub(crate) metadata_string: ::std::option::Option<::std::string::String>,
    pub(crate) inline_chunk: ::std::option::Option<::aws_smithy_http::byte_stream::ByteStream>,
    pub(crate) inline_chunk_length: ::std::option::Option<i64>,
    pub(crate) inline_chunk_checksum: ::std::option::Option<::std::string::String>,
    pub(crate) inline_chunk_checksum_algorithm: ::std::option::Option<::std::string::String>,
    pub(crate) object_checksum: ::std::option::Option<::std::string::String>,
    pub(crate) object_checksum_algorithm: ::std::option::Option<crate::types::SummaryChecksumAlgorithm>,
    pub(crate) throw_on_duplicate: ::std::option::Option<bool>,
}
impl PutObjectInputBuilder {
    /// Backup job Id for the in-progress backup.
    pub fn backup_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.backup_job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Backup job Id for the in-progress backup.
    pub fn set_backup_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.backup_job_id = input;
        self
    }
    /// Backup job Id for the in-progress backup.
    pub fn get_backup_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.backup_job_id
    }
    /// The name of the Object to be uploaded.
    pub fn object_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the Object to be uploaded.
    pub fn set_object_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object_name = input;
        self
    }
    /// The name of the Object to be uploaded.
    pub fn get_object_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.object_name
    }
    /// Store user defined metadata like backup checksum, disk ids, restore metadata etc.
    pub fn metadata_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metadata_string = ::std::option::Option::Some(input.into());
        self
    }
    /// Store user defined metadata like backup checksum, disk ids, restore metadata etc.
    pub fn set_metadata_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metadata_string = input;
        self
    }
    /// Store user defined metadata like backup checksum, disk ids, restore metadata etc.
    pub fn get_metadata_string(&self) -> &::std::option::Option<::std::string::String> {
        &self.metadata_string
    }
    /// Inline chunk data to be uploaded.
    pub fn inline_chunk(mut self, input: ::aws_smithy_http::byte_stream::ByteStream) -> Self {
        self.inline_chunk = ::std::option::Option::Some(input);
        self
    }
    /// Inline chunk data to be uploaded.
    pub fn set_inline_chunk(mut self, input: ::std::option::Option<::aws_smithy_http::byte_stream::ByteStream>) -> Self {
        self.inline_chunk = input;
        self
    }
    /// Inline chunk data to be uploaded.
    pub fn get_inline_chunk(&self) -> &::std::option::Option<::aws_smithy_http::byte_stream::ByteStream> {
        &self.inline_chunk
    }
    /// Length of the inline chunk data.
    pub fn inline_chunk_length(mut self, input: i64) -> Self {
        self.inline_chunk_length = ::std::option::Option::Some(input);
        self
    }
    /// Length of the inline chunk data.
    pub fn set_inline_chunk_length(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inline_chunk_length = input;
        self
    }
    /// Length of the inline chunk data.
    pub fn get_inline_chunk_length(&self) -> &::std::option::Option<i64> {
        &self.inline_chunk_length
    }
    /// Inline chunk checksum
    pub fn inline_chunk_checksum(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inline_chunk_checksum = ::std::option::Option::Some(input.into());
        self
    }
    /// Inline chunk checksum
    pub fn set_inline_chunk_checksum(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inline_chunk_checksum = input;
        self
    }
    /// Inline chunk checksum
    pub fn get_inline_chunk_checksum(&self) -> &::std::option::Option<::std::string::String> {
        &self.inline_chunk_checksum
    }
    /// Inline chunk checksum algorithm
    pub fn inline_chunk_checksum_algorithm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inline_chunk_checksum_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    /// Inline chunk checksum algorithm
    pub fn set_inline_chunk_checksum_algorithm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inline_chunk_checksum_algorithm = input;
        self
    }
    /// Inline chunk checksum algorithm
    pub fn get_inline_chunk_checksum_algorithm(&self) -> &::std::option::Option<::std::string::String> {
        &self.inline_chunk_checksum_algorithm
    }
    /// object checksum
    pub fn object_checksum(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object_checksum = ::std::option::Option::Some(input.into());
        self
    }
    /// object checksum
    pub fn set_object_checksum(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object_checksum = input;
        self
    }
    /// object checksum
    pub fn get_object_checksum(&self) -> &::std::option::Option<::std::string::String> {
        &self.object_checksum
    }
    /// object checksum algorithm
    pub fn object_checksum_algorithm(mut self, input: crate::types::SummaryChecksumAlgorithm) -> Self {
        self.object_checksum_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// object checksum algorithm
    pub fn set_object_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::SummaryChecksumAlgorithm>) -> Self {
        self.object_checksum_algorithm = input;
        self
    }
    /// object checksum algorithm
    pub fn get_object_checksum_algorithm(&self) -> &::std::option::Option<crate::types::SummaryChecksumAlgorithm> {
        &self.object_checksum_algorithm
    }
    /// Throw an exception if Object name is already exist.
    pub fn throw_on_duplicate(mut self, input: bool) -> Self {
        self.throw_on_duplicate = ::std::option::Option::Some(input);
        self
    }
    /// Throw an exception if Object name is already exist.
    pub fn set_throw_on_duplicate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.throw_on_duplicate = input;
        self
    }
    /// Throw an exception if Object name is already exist.
    pub fn get_throw_on_duplicate(&self) -> &::std::option::Option<bool> {
        &self.throw_on_duplicate
    }
    /// Consumes the builder and constructs a [`PutObjectInput`](crate::operation::put_object::PutObjectInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_object::PutObjectInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_object::PutObjectInput {
            backup_job_id: self.backup_job_id,
            object_name: self.object_name,
            metadata_string: self.metadata_string,
            inline_chunk: self.inline_chunk.unwrap_or_default(),
            inline_chunk_length: self.inline_chunk_length.unwrap_or_default(),
            inline_chunk_checksum: self.inline_chunk_checksum,
            inline_chunk_checksum_algorithm: self.inline_chunk_checksum_algorithm,
            object_checksum: self.object_checksum,
            object_checksum_algorithm: self.object_checksum_algorithm,
            throw_on_duplicate: self.throw_on_duplicate.unwrap_or_default(),
        })
    }
}
