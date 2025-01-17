// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the name of the schema and table to be reloaded.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TableToReload {
    /// <p>The schema name of the table to be reloaded.</p>
    pub schema_name: ::std::option::Option<::std::string::String>,
    /// <p>The table name of the table to be reloaded.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
}
impl TableToReload {
    /// <p>The schema name of the table to be reloaded.</p>
    pub fn schema_name(&self) -> ::std::option::Option<&str> {
        self.schema_name.as_deref()
    }
    /// <p>The table name of the table to be reloaded.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
}
impl TableToReload {
    /// Creates a new builder-style object to manufacture [`TableToReload`](crate::types::TableToReload).
    pub fn builder() -> crate::types::builders::TableToReloadBuilder {
        crate::types::builders::TableToReloadBuilder::default()
    }
}

/// A builder for [`TableToReload`](crate::types::TableToReload).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TableToReloadBuilder {
    pub(crate) schema_name: ::std::option::Option<::std::string::String>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl TableToReloadBuilder {
    /// <p>The schema name of the table to be reloaded.</p>
    pub fn schema_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schema_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The schema name of the table to be reloaded.</p>
    pub fn set_schema_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schema_name = input;
        self
    }
    /// <p>The schema name of the table to be reloaded.</p>
    pub fn get_schema_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.schema_name
    }
    /// <p>The table name of the table to be reloaded.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The table name of the table to be reloaded.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The table name of the table to be reloaded.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// Consumes the builder and constructs a [`TableToReload`](crate::types::TableToReload).
    pub fn build(self) -> crate::types::TableToReload {
        crate::types::TableToReload {
            schema_name: self.schema_name,
            table_name: self.table_name,
        }
    }
}
