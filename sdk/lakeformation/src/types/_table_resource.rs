// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure for the table object. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TableResource {
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    pub catalog_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    pub database_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the table.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A wildcard object representing every table under a database.</p>
    /// <p>At least one of <code>TableResource$Name</code> or <code>TableResource$TableWildcard</code> is required.</p>
    pub table_wildcard: ::std::option::Option<crate::types::TableWildcard>,
}
impl TableResource {
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    pub fn catalog_id(&self) -> ::std::option::Option<&str> {
        self.catalog_id.as_deref()
    }
    /// <p>The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
    /// <p>The name of the table.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A wildcard object representing every table under a database.</p>
    /// <p>At least one of <code>TableResource$Name</code> or <code>TableResource$TableWildcard</code> is required.</p>
    pub fn table_wildcard(&self) -> ::std::option::Option<&crate::types::TableWildcard> {
        self.table_wildcard.as_ref()
    }
}
impl TableResource {
    /// Creates a new builder-style object to manufacture [`TableResource`](crate::types::TableResource).
    pub fn builder() -> crate::types::builders::TableResourceBuilder {
        crate::types::builders::TableResourceBuilder::default()
    }
}

/// A builder for [`TableResource`](crate::types::TableResource).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TableResourceBuilder {
    pub(crate) catalog_id: ::std::option::Option<::std::string::String>,
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) table_wildcard: ::std::option::Option<crate::types::TableWildcard>,
}
impl TableResourceBuilder {
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.catalog_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.catalog_id = input;
        self
    }
    /// <p>The identifier for the Data Catalog. By default, it is the account ID of the caller.</p>
    pub fn get_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.catalog_id
    }
    /// <p>The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    pub fn database_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    pub fn set_database_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.database_name = input;
        self
    }
    /// <p>The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal. </p>
    pub fn get_database_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.database_name
    }
    /// <p>The name of the table.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the table.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>A wildcard object representing every table under a database.</p>
    /// <p>At least one of <code>TableResource$Name</code> or <code>TableResource$TableWildcard</code> is required.</p>
    pub fn table_wildcard(mut self, input: crate::types::TableWildcard) -> Self {
        self.table_wildcard = ::std::option::Option::Some(input);
        self
    }
    /// <p>A wildcard object representing every table under a database.</p>
    /// <p>At least one of <code>TableResource$Name</code> or <code>TableResource$TableWildcard</code> is required.</p>
    pub fn set_table_wildcard(mut self, input: ::std::option::Option<crate::types::TableWildcard>) -> Self {
        self.table_wildcard = input;
        self
    }
    /// <p>A wildcard object representing every table under a database.</p>
    /// <p>At least one of <code>TableResource$Name</code> or <code>TableResource$TableWildcard</code> is required.</p>
    pub fn get_table_wildcard(&self) -> &::std::option::Option<crate::types::TableWildcard> {
        &self.table_wildcard
    }
    /// Consumes the builder and constructs a [`TableResource`](crate::types::TableResource).
    pub fn build(self) -> crate::types::TableResource {
        crate::types::TableResource {
            catalog_id: self.catalog_id,
            database_name: self.database_name,
            name: self.name,
            table_wildcard: self.table_wildcard,
        }
    }
}
