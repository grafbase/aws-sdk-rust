// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_image_attribute::_modify_image_attribute_output::ModifyImageAttributeOutputBuilder;

pub use crate::operation::modify_image_attribute::_modify_image_attribute_input::ModifyImageAttributeInputBuilder;

impl ModifyImageAttributeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_image_attribute::ModifyImageAttributeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_image_attribute::ModifyImageAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_image_attribute();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyImageAttribute`.
///
/// <p>Modifies the specified attribute of the specified AMI. You can specify only one attribute at a time.</p>
/// <p>To specify the attribute, you can use the <code>Attribute</code> parameter, or one of the following parameters: <code>Description</code>, <code>ImdsSupport</code>, or <code>LaunchPermission</code>.</p>
/// <p>Images with an Amazon Web Services Marketplace product code cannot be made public.</p>
/// <p>To enable the SriovNetSupport enhanced networking attribute of an image, enable SriovNetSupport on an instance and create an AMI from the instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyImageAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_image_attribute::builders::ModifyImageAttributeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ModifyImageAttributeFluentBuilder {
    /// Creates a new `ModifyImageAttribute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyImageAttribute as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_image_attribute::builders::ModifyImageAttributeInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_image_attribute::ModifyImageAttributeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_image_attribute::ModifyImageAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_image_attribute::ModifyImageAttribute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_image_attribute::ModifyImageAttribute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::modify_image_attribute::ModifyImageAttributeOutput,
            crate::operation::modify_image_attribute::ModifyImageAttributeError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_image_attribute::ModifyImageAttributeError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the attribute to modify.</p>
    /// <p>Valid values: <code>description</code> | <code>imdsSupport</code> | <code>launchPermission</code> </p>
    pub fn attribute(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attribute(input.into());
        self
    }
    /// <p>The name of the attribute to modify.</p>
    /// <p>Valid values: <code>description</code> | <code>imdsSupport</code> | <code>launchPermission</code> </p>
    pub fn set_attribute(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_attribute(input);
        self
    }
    /// <p>The name of the attribute to modify.</p>
    /// <p>Valid values: <code>description</code> | <code>imdsSupport</code> | <code>launchPermission</code> </p>
    pub fn get_attribute(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_attribute()
    }
    /// <p>A new description for the AMI.</p>
    pub fn description(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.description(input);
        self
    }
    /// <p>A new description for the AMI.</p>
    pub fn set_description(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A new description for the AMI.</p>
    pub fn get_description(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_description()
    }
    /// <p>The ID of the AMI.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_image_id()
    }
    /// <p>A new launch permission for the AMI.</p>
    pub fn launch_permission(mut self, input: crate::types::LaunchPermissionModifications) -> Self {
        self.inner = self.inner.launch_permission(input);
        self
    }
    /// <p>A new launch permission for the AMI.</p>
    pub fn set_launch_permission(mut self, input: ::std::option::Option<crate::types::LaunchPermissionModifications>) -> Self {
        self.inner = self.inner.set_launch_permission(input);
        self
    }
    /// <p>A new launch permission for the AMI.</p>
    pub fn get_launch_permission(&self) -> &::std::option::Option<crate::types::LaunchPermissionModifications> {
        self.inner.get_launch_permission()
    }
    /// <p>The operation type. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn operation_type(mut self, input: crate::types::OperationType) -> Self {
        self.inner = self.inner.operation_type(input);
        self
    }
    /// <p>The operation type. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn set_operation_type(mut self, input: ::std::option::Option<crate::types::OperationType>) -> Self {
        self.inner = self.inner.set_operation_type(input);
        self
    }
    /// <p>The operation type. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn get_operation_type(&self) -> &::std::option::Option<crate::types::OperationType> {
        self.inner.get_operation_type()
    }
    /// Appends an item to `ProductCodes`.
    ///
    /// To override the contents of this collection use [`set_product_codes`](Self::set_product_codes).
    ///
    /// <p>Not supported.</p>
    pub fn product_codes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.product_codes(input.into());
        self
    }
    /// <p>Not supported.</p>
    pub fn set_product_codes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_product_codes(input);
        self
    }
    /// <p>Not supported.</p>
    pub fn get_product_codes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_product_codes()
    }
    /// Appends an item to `UserGroups`.
    ///
    /// To override the contents of this collection use [`set_user_groups`](Self::set_user_groups).
    ///
    /// <p>The user groups. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn user_groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_groups(input.into());
        self
    }
    /// <p>The user groups. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn set_user_groups(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_user_groups(input);
        self
    }
    /// <p>The user groups. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn get_user_groups(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_user_groups()
    }
    /// Appends an item to `UserIds`.
    ///
    /// To override the contents of this collection use [`set_user_ids`](Self::set_user_ids).
    ///
    /// <p>The Amazon Web Services account IDs. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn user_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_ids(input.into());
        self
    }
    /// <p>The Amazon Web Services account IDs. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn set_user_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_user_ids(input);
        self
    }
    /// <p>The Amazon Web Services account IDs. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn get_user_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_user_ids()
    }
    /// <p>The value of the attribute being modified. This parameter can be used only when the <code>Attribute</code> parameter is <code>description</code> or <code>imdsSupport</code>.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.value(input.into());
        self
    }
    /// <p>The value of the attribute being modified. This parameter can be used only when the <code>Attribute</code> parameter is <code>description</code> or <code>imdsSupport</code>.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_value(input);
        self
    }
    /// <p>The value of the attribute being modified. This parameter can be used only when the <code>Attribute</code> parameter is <code>description</code> or <code>imdsSupport</code>.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_value()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// Appends an item to `OrganizationArns`.
    ///
    /// To override the contents of this collection use [`set_organization_arns`](Self::set_organization_arns).
    ///
    /// <p>The Amazon Resource Name (ARN) of an organization. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn organization_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_arns(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organization. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn set_organization_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_organization_arns(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organization. This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn get_organization_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_organization_arns()
    }
    /// Appends an item to `OrganizationalUnitArns`.
    ///
    /// To override the contents of this collection use [`set_organizational_unit_arns`](Self::set_organizational_unit_arns).
    ///
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU). This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn organizational_unit_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organizational_unit_arns(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU). This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn set_organizational_unit_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_organizational_unit_arns(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU). This parameter can be used only when the <code>Attribute</code> parameter is <code>launchPermission</code>.</p>
    pub fn get_organizational_unit_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_organizational_unit_arns()
    }
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <important>
    /// <p>Do not use this parameter unless your AMI software supports IMDSv2. After you set the value to <code>v2.0</code>, you can't undo it. The only way to “reset” your AMI is to create a new AMI from the underlying snapshot.</p>
    /// </important>
    pub fn imds_support(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.imds_support(input);
        self
    }
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <important>
    /// <p>Do not use this parameter unless your AMI software supports IMDSv2. After you set the value to <code>v2.0</code>, you can't undo it. The only way to “reset” your AMI is to create a new AMI from the underlying snapshot.</p>
    /// </important>
    pub fn set_imds_support(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_imds_support(input);
        self
    }
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <important>
    /// <p>Do not use this parameter unless your AMI software supports IMDSv2. After you set the value to <code>v2.0</code>, you can't undo it. The only way to “reset” your AMI is to create a new AMI from the underlying snapshot.</p>
    /// </important>
    pub fn get_imds_support(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_imds_support()
    }
}
