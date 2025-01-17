// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResolveAppVersionResourcesOutput {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub app_arn: ::std::option::Option<::std::string::String>,
    /// <p>The version of the application.</p>
    pub app_version: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for a specific resolution.</p>
    pub resolution_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the action.</p>
    pub status: ::std::option::Option<crate::types::ResourceResolutionStatusType>,
    _request_id: Option<String>,
}
impl ResolveAppVersionResourcesOutput {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(&self) -> ::std::option::Option<&str> {
        self.app_arn.as_deref()
    }
    /// <p>The version of the application.</p>
    pub fn app_version(&self) -> ::std::option::Option<&str> {
        self.app_version.as_deref()
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn resolution_id(&self) -> ::std::option::Option<&str> {
        self.resolution_id.as_deref()
    }
    /// <p>The status of the action.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ResourceResolutionStatusType> {
        self.status.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for ResolveAppVersionResourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ResolveAppVersionResourcesOutput {
    /// Creates a new builder-style object to manufacture [`ResolveAppVersionResourcesOutput`](crate::operation::resolve_app_version_resources::ResolveAppVersionResourcesOutput).
    pub fn builder() -> crate::operation::resolve_app_version_resources::builders::ResolveAppVersionResourcesOutputBuilder {
        crate::operation::resolve_app_version_resources::builders::ResolveAppVersionResourcesOutputBuilder::default()
    }
}

/// A builder for [`ResolveAppVersionResourcesOutput`](crate::operation::resolve_app_version_resources::ResolveAppVersionResourcesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ResolveAppVersionResourcesOutputBuilder {
    pub(crate) app_arn: ::std::option::Option<::std::string::String>,
    pub(crate) app_version: ::std::option::Option<::std::string::String>,
    pub(crate) resolution_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ResourceResolutionStatusType>,
    _request_id: Option<String>,
}
impl ResolveAppVersionResourcesOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn set_app_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn get_app_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.app_arn
    }
    /// <p>The version of the application.</p>
    pub fn app_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the application.</p>
    pub fn set_app_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_version = input;
        self
    }
    /// <p>The version of the application.</p>
    pub fn get_app_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.app_version
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn resolution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resolution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn set_resolution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resolution_id = input;
        self
    }
    /// <p>The identifier for a specific resolution.</p>
    pub fn get_resolution_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.resolution_id
    }
    /// <p>The status of the action.</p>
    pub fn status(mut self, input: crate::types::ResourceResolutionStatusType) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the action.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ResourceResolutionStatusType>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the action.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ResourceResolutionStatusType> {
        &self.status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ResolveAppVersionResourcesOutput`](crate::operation::resolve_app_version_resources::ResolveAppVersionResourcesOutput).
    pub fn build(self) -> crate::operation::resolve_app_version_resources::ResolveAppVersionResourcesOutput {
        crate::operation::resolve_app_version_resources::ResolveAppVersionResourcesOutput {
            app_arn: self.app_arn,
            app_version: self.app_version,
            resolution_id: self.resolution_id,
            status: self.status,
            _request_id: self._request_id,
        }
    }
}
