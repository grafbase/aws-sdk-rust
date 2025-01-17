// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of names of sending authorization policies that apply to an identity.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListIdentityPoliciesOutput {
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub policy_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl ListIdentityPoliciesOutput {
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub fn policy_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.policy_names.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListIdentityPoliciesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListIdentityPoliciesOutput {
    /// Creates a new builder-style object to manufacture [`ListIdentityPoliciesOutput`](crate::operation::list_identity_policies::ListIdentityPoliciesOutput).
    pub fn builder() -> crate::operation::list_identity_policies::builders::ListIdentityPoliciesOutputBuilder {
        crate::operation::list_identity_policies::builders::ListIdentityPoliciesOutputBuilder::default()
    }
}

/// A builder for [`ListIdentityPoliciesOutput`](crate::operation::list_identity_policies::ListIdentityPoliciesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListIdentityPoliciesOutputBuilder {
    pub(crate) policy_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl ListIdentityPoliciesOutputBuilder {
    /// Appends an item to `policy_names`.
    ///
    /// To override the contents of this collection use [`set_policy_names`](Self::set_policy_names).
    ///
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub fn policy_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.policy_names.unwrap_or_default();
        v.push(input.into());
        self.policy_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub fn set_policy_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.policy_names = input;
        self
    }
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub fn get_policy_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.policy_names
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListIdentityPoliciesOutput`](crate::operation::list_identity_policies::ListIdentityPoliciesOutput).
    pub fn build(self) -> crate::operation::list_identity_policies::ListIdentityPoliciesOutput {
        crate::operation::list_identity_policies::ListIdentityPoliciesOutput {
            policy_names: self.policy_names,
            _request_id: self._request_id,
        }
    }
}
