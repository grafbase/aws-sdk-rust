// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateLambdaFunctionInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance..</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Lambda function being disassociated.</p>
    pub function_arn: ::std::option::Option<::std::string::String>,
}
impl DisassociateLambdaFunctionInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance..</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function being disassociated.</p>
    pub fn function_arn(&self) -> ::std::option::Option<&str> {
        self.function_arn.as_deref()
    }
}
impl DisassociateLambdaFunctionInput {
    /// Creates a new builder-style object to manufacture [`DisassociateLambdaFunctionInput`](crate::operation::disassociate_lambda_function::DisassociateLambdaFunctionInput).
    pub fn builder() -> crate::operation::disassociate_lambda_function::builders::DisassociateLambdaFunctionInputBuilder {
        crate::operation::disassociate_lambda_function::builders::DisassociateLambdaFunctionInputBuilder::default()
    }
}

/// A builder for [`DisassociateLambdaFunctionInput`](crate::operation::disassociate_lambda_function::DisassociateLambdaFunctionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateLambdaFunctionInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) function_arn: ::std::option::Option<::std::string::String>,
}
impl DisassociateLambdaFunctionInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance..</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance..</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance..</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function being disassociated.</p>
    pub fn function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function being disassociated.</p>
    pub fn set_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function being disassociated.</p>
    pub fn get_function_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.function_arn
    }
    /// Consumes the builder and constructs a [`DisassociateLambdaFunctionInput`](crate::operation::disassociate_lambda_function::DisassociateLambdaFunctionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_lambda_function::DisassociateLambdaFunctionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disassociate_lambda_function::DisassociateLambdaFunctionInput {
            instance_id: self.instance_id,
            function_arn: self.function_arn,
        })
    }
}
