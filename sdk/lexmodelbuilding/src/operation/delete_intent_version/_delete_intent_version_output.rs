// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteIntentVersionOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DeleteIntentVersionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteIntentVersionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteIntentVersionOutput`](crate::operation::delete_intent_version::DeleteIntentVersionOutput).
    pub fn builder() -> crate::operation::delete_intent_version::builders::DeleteIntentVersionOutputBuilder {
        crate::operation::delete_intent_version::builders::DeleteIntentVersionOutputBuilder::default()
    }
}

/// A builder for [`DeleteIntentVersionOutput`](crate::operation::delete_intent_version::DeleteIntentVersionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteIntentVersionOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteIntentVersionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteIntentVersionOutput`](crate::operation::delete_intent_version::DeleteIntentVersionOutput).
    pub fn build(self) -> crate::operation::delete_intent_version::DeleteIntentVersionOutput {
        crate::operation::delete_intent_version::DeleteIntentVersionOutput {
            _request_id: self._request_id,
        }
    }
}
