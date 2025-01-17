// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the details of the <code>RequestCancelActivityTask</code> decision.</p>
/// <p> <b>Access Control</b> </p>
/// <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p>
/// <ul>
/// <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li>
/// <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li>
/// <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li>
/// </ul>
/// <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestCancelActivityTaskDecisionAttributes {
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    pub activity_id: ::std::option::Option<::std::string::String>,
}
impl RequestCancelActivityTaskDecisionAttributes {
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    pub fn activity_id(&self) -> ::std::option::Option<&str> {
        self.activity_id.as_deref()
    }
}
impl RequestCancelActivityTaskDecisionAttributes {
    /// Creates a new builder-style object to manufacture [`RequestCancelActivityTaskDecisionAttributes`](crate::types::RequestCancelActivityTaskDecisionAttributes).
    pub fn builder() -> crate::types::builders::RequestCancelActivityTaskDecisionAttributesBuilder {
        crate::types::builders::RequestCancelActivityTaskDecisionAttributesBuilder::default()
    }
}

/// A builder for [`RequestCancelActivityTaskDecisionAttributes`](crate::types::RequestCancelActivityTaskDecisionAttributes).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestCancelActivityTaskDecisionAttributesBuilder {
    pub(crate) activity_id: ::std::option::Option<::std::string::String>,
}
impl RequestCancelActivityTaskDecisionAttributesBuilder {
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    pub fn activity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.activity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    pub fn set_activity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.activity_id = input;
        self
    }
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    pub fn get_activity_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.activity_id
    }
    /// Consumes the builder and constructs a [`RequestCancelActivityTaskDecisionAttributes`](crate::types::RequestCancelActivityTaskDecisionAttributes).
    pub fn build(self) -> crate::types::RequestCancelActivityTaskDecisionAttributes {
        crate::types::RequestCancelActivityTaskDecisionAttributes {
            activity_id: self.activity_id,
        }
    }
}
