// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteContactFlowInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the flow.</p>
    pub contact_flow_id: ::std::option::Option<::std::string::String>,
}
impl DeleteContactFlowInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The identifier of the flow.</p>
    pub fn contact_flow_id(&self) -> ::std::option::Option<&str> {
        self.contact_flow_id.as_deref()
    }
}
impl DeleteContactFlowInput {
    /// Creates a new builder-style object to manufacture [`DeleteContactFlowInput`](crate::operation::delete_contact_flow::DeleteContactFlowInput).
    pub fn builder() -> crate::operation::delete_contact_flow::builders::DeleteContactFlowInputBuilder {
        crate::operation::delete_contact_flow::builders::DeleteContactFlowInputBuilder::default()
    }
}

/// A builder for [`DeleteContactFlowInput`](crate::operation::delete_contact_flow::DeleteContactFlowInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteContactFlowInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) contact_flow_id: ::std::option::Option<::std::string::String>,
}
impl DeleteContactFlowInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// <p>The identifier of the flow.</p>
    pub fn contact_flow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.contact_flow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the flow.</p>
    pub fn set_contact_flow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.contact_flow_id = input;
        self
    }
    /// <p>The identifier of the flow.</p>
    pub fn get_contact_flow_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.contact_flow_id
    }
    /// Consumes the builder and constructs a [`DeleteContactFlowInput`](crate::operation::delete_contact_flow::DeleteContactFlowInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_contact_flow::DeleteContactFlowInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_contact_flow::DeleteContactFlowInput {
            instance_id: self.instance_id,
            contact_flow_id: self.contact_flow_id,
        })
    }
}
