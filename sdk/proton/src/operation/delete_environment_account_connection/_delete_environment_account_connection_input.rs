// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteEnvironmentAccountConnectionInput {
    /// <p>The ID of the environment account connection to delete.</p>
    pub id: ::std::option::Option<::std::string::String>,
}
impl DeleteEnvironmentAccountConnectionInput {
    /// <p>The ID of the environment account connection to delete.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl DeleteEnvironmentAccountConnectionInput {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentAccountConnectionInput`](crate::operation::delete_environment_account_connection::DeleteEnvironmentAccountConnectionInput).
    pub fn builder() -> crate::operation::delete_environment_account_connection::builders::DeleteEnvironmentAccountConnectionInputBuilder {
        crate::operation::delete_environment_account_connection::builders::DeleteEnvironmentAccountConnectionInputBuilder::default()
    }
}

/// A builder for [`DeleteEnvironmentAccountConnectionInput`](crate::operation::delete_environment_account_connection::DeleteEnvironmentAccountConnectionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteEnvironmentAccountConnectionInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl DeleteEnvironmentAccountConnectionInputBuilder {
    /// <p>The ID of the environment account connection to delete.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the environment account connection to delete.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the environment account connection to delete.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Consumes the builder and constructs a [`DeleteEnvironmentAccountConnectionInput`](crate::operation::delete_environment_account_connection::DeleteEnvironmentAccountConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_environment_account_connection::DeleteEnvironmentAccountConnectionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_environment_account_connection::DeleteEnvironmentAccountConnectionInput { id: self.id })
    }
}
