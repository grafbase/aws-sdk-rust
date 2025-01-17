// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a model in a generic data schema.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CodegenGenericDataModel {
    /// <p>The fields in the generic data model.</p>
    pub fields: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CodegenGenericDataField>>,
    /// <p>Specifies whether the generic data model is a join table.</p>
    pub is_join_table: ::std::option::Option<bool>,
    /// <p>The primary keys of the generic data model.</p>
    pub primary_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl CodegenGenericDataModel {
    /// <p>The fields in the generic data model.</p>
    pub fn fields(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, crate::types::CodegenGenericDataField>> {
        self.fields.as_ref()
    }
    /// <p>Specifies whether the generic data model is a join table.</p>
    pub fn is_join_table(&self) -> ::std::option::Option<bool> {
        self.is_join_table
    }
    /// <p>The primary keys of the generic data model.</p>
    pub fn primary_keys(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.primary_keys.as_deref()
    }
}
impl CodegenGenericDataModel {
    /// Creates a new builder-style object to manufacture [`CodegenGenericDataModel`](crate::types::CodegenGenericDataModel).
    pub fn builder() -> crate::types::builders::CodegenGenericDataModelBuilder {
        crate::types::builders::CodegenGenericDataModelBuilder::default()
    }
}

/// A builder for [`CodegenGenericDataModel`](crate::types::CodegenGenericDataModel).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CodegenGenericDataModelBuilder {
    pub(crate) fields: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CodegenGenericDataField>>,
    pub(crate) is_join_table: ::std::option::Option<bool>,
    pub(crate) primary_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl CodegenGenericDataModelBuilder {
    /// Adds a key-value pair to `fields`.
    ///
    /// To override the contents of this collection use [`set_fields`](Self::set_fields).
    ///
    /// <p>The fields in the generic data model.</p>
    pub fn fields(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::CodegenGenericDataField) -> Self {
        let mut hash_map = self.fields.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.fields = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The fields in the generic data model.</p>
    pub fn set_fields(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CodegenGenericDataField>>,
    ) -> Self {
        self.fields = input;
        self
    }
    /// <p>The fields in the generic data model.</p>
    pub fn get_fields(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CodegenGenericDataField>> {
        &self.fields
    }
    /// <p>Specifies whether the generic data model is a join table.</p>
    pub fn is_join_table(mut self, input: bool) -> Self {
        self.is_join_table = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the generic data model is a join table.</p>
    pub fn set_is_join_table(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_join_table = input;
        self
    }
    /// <p>Specifies whether the generic data model is a join table.</p>
    pub fn get_is_join_table(&self) -> &::std::option::Option<bool> {
        &self.is_join_table
    }
    /// Appends an item to `primary_keys`.
    ///
    /// To override the contents of this collection use [`set_primary_keys`](Self::set_primary_keys).
    ///
    /// <p>The primary keys of the generic data model.</p>
    pub fn primary_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.primary_keys.unwrap_or_default();
        v.push(input.into());
        self.primary_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p>The primary keys of the generic data model.</p>
    pub fn set_primary_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.primary_keys = input;
        self
    }
    /// <p>The primary keys of the generic data model.</p>
    pub fn get_primary_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.primary_keys
    }
    /// Consumes the builder and constructs a [`CodegenGenericDataModel`](crate::types::CodegenGenericDataModel).
    pub fn build(self) -> crate::types::CodegenGenericDataModel {
        crate::types::CodegenGenericDataModel {
            fields: self.fields,
            is_join_table: self.is_join_table,
            primary_keys: self.primary_keys,
        }
    }
}
