// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`CreateTokenInput`](crate::input::CreateTokenInput)
pub mod create_token_input {
    /// A builder for [`CreateTokenInput`](crate::input::CreateTokenInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) client_id: std::option::Option<std::string::String>,
        pub(crate) client_secret: std::option::Option<std::string::String>,
        pub(crate) grant_type: std::option::Option<std::string::String>,
        pub(crate) device_code: std::option::Option<std::string::String>,
        pub(crate) code: std::option::Option<std::string::String>,
        pub(crate) refresh_token: std::option::Option<std::string::String>,
        pub(crate) scope: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) redirect_uri: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_id = Some(input.into());
            self
        }
        /// <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_id = input;
            self
        }
        /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn client_secret(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_secret = Some(input.into());
            self
        }
        /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.client_secret = input;
            self
        }
        /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
        pub fn grant_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.grant_type = Some(input.into());
            self
        }
        /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
        pub fn set_grant_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.grant_type = input;
            self
        }
        /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
        pub fn device_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_code = Some(input.into());
            self
        }
        /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
        pub fn set_device_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_code = input;
            self
        }
        /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
        pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
            self.code = Some(input.into());
            self
        }
        /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
        pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.code = input;
            self
        }
        /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
        pub fn refresh_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.refresh_token = Some(input.into());
            self
        }
        /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
        pub fn set_refresh_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.refresh_token = input;
            self
        }
        /// Appends an item to `scope`.
        ///
        /// To override the contents of this collection use [`set_scope`](Self::set_scope).
        ///
        /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn scope(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.scope.unwrap_or_default();
            v.push(input.into());
            self.scope = Some(v);
            self
        }
        /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn set_scope(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.scope = input;
            self
        }
        /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
        pub fn redirect_uri(mut self, input: impl Into<std::string::String>) -> Self {
            self.redirect_uri = Some(input.into());
            self
        }
        /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
        pub fn set_redirect_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.redirect_uri = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateTokenInput`](crate::input::CreateTokenInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::CreateTokenInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::CreateTokenInput {
                client_id: self.client_id,
                client_secret: self.client_secret,
                grant_type: self.grant_type,
                device_code: self.device_code,
                code: self.code,
                refresh_token: self.refresh_token,
                scope: self.scope,
                redirect_uri: self.redirect_uri,
            })
        }
    }
}
#[doc(hidden)]
pub type CreateTokenInputOperationOutputAlias = crate::operation::CreateToken;
#[doc(hidden)]
pub type CreateTokenInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl CreateTokenInput {
    /// Consumes the builder and constructs an Operation<[`CreateToken`](crate::operation::CreateToken)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::CreateToken,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::CreateTokenInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/token").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::CreateTokenInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_create_token(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::CreateToken::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateToken",
            "ssooidc",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`CreateTokenInput`](crate::input::CreateTokenInput)
    pub fn builder() -> crate::input::create_token_input::Builder {
        crate::input::create_token_input::Builder::default()
    }
}

/// See [`RegisterClientInput`](crate::input::RegisterClientInput)
pub mod register_client_input {
    /// A builder for [`RegisterClientInput`](crate::input::RegisterClientInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) client_name: std::option::Option<std::string::String>,
        pub(crate) client_type: std::option::Option<std::string::String>,
        pub(crate) scopes: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>The friendly name of the client.</p>
        pub fn client_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_name = Some(input.into());
            self
        }
        /// <p>The friendly name of the client.</p>
        pub fn set_client_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_name = input;
            self
        }
        /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
        pub fn client_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_type = Some(input.into());
            self
        }
        /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
        pub fn set_client_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_type = input;
            self
        }
        /// Appends an item to `scopes`.
        ///
        /// To override the contents of this collection use [`set_scopes`](Self::set_scopes).
        ///
        /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn scopes(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.scopes.unwrap_or_default();
            v.push(input.into());
            self.scopes = Some(v);
            self
        }
        /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn set_scopes(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.scopes = input;
            self
        }
        /// Consumes the builder and constructs a [`RegisterClientInput`](crate::input::RegisterClientInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::RegisterClientInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::RegisterClientInput {
                client_name: self.client_name,
                client_type: self.client_type,
                scopes: self.scopes,
            })
        }
    }
}
#[doc(hidden)]
pub type RegisterClientInputOperationOutputAlias = crate::operation::RegisterClient;
#[doc(hidden)]
pub type RegisterClientInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl RegisterClientInput {
    /// Consumes the builder and constructs an Operation<[`RegisterClient`](crate::operation::RegisterClient)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::RegisterClient,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::RegisterClientInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/client/register").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::RegisterClientInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_register_client(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::RegisterClient::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "RegisterClient",
            "ssooidc",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`RegisterClientInput`](crate::input::RegisterClientInput)
    pub fn builder() -> crate::input::register_client_input::Builder {
        crate::input::register_client_input::Builder::default()
    }
}

/// See [`StartDeviceAuthorizationInput`](crate::input::StartDeviceAuthorizationInput)
pub mod start_device_authorization_input {
    /// A builder for [`StartDeviceAuthorizationInput`](crate::input::StartDeviceAuthorizationInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) client_id: std::option::Option<std::string::String>,
        pub(crate) client_secret: std::option::Option<std::string::String>,
        pub(crate) start_url: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_id = Some(input.into());
            self
        }
        /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_id = input;
            self
        }
        /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn client_secret(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_secret = Some(input.into());
            self
        }
        /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.client_secret = input;
            self
        }
        /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
        pub fn start_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.start_url = Some(input.into());
            self
        }
        /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
        pub fn set_start_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.start_url = input;
            self
        }
        /// Consumes the builder and constructs a [`StartDeviceAuthorizationInput`](crate::input::StartDeviceAuthorizationInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::StartDeviceAuthorizationInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::StartDeviceAuthorizationInput {
                client_id: self.client_id,
                client_secret: self.client_secret,
                start_url: self.start_url,
            })
        }
    }
}
#[doc(hidden)]
pub type StartDeviceAuthorizationInputOperationOutputAlias =
    crate::operation::StartDeviceAuthorization;
#[doc(hidden)]
pub type StartDeviceAuthorizationInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl StartDeviceAuthorizationInput {
    /// Consumes the builder and constructs an Operation<[`StartDeviceAuthorization`](crate::operation::StartDeviceAuthorization)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::StartDeviceAuthorization,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::StartDeviceAuthorizationInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/device_authorization").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::StartDeviceAuthorizationInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_start_device_authorization(
                &self,
            )?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        signing_config.signing_requirements = aws_sig_auth::signer::SigningRequirements::Disabled;
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::StartDeviceAuthorization::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "StartDeviceAuthorization",
            "ssooidc",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`StartDeviceAuthorizationInput`](crate::input::StartDeviceAuthorizationInput)
    pub fn builder() -> crate::input::start_device_authorization_input::Builder {
        crate::input::start_device_authorization_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartDeviceAuthorizationInput {
    /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
    pub client_id: std::option::Option<std::string::String>,
    /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
    pub client_secret: std::option::Option<std::string::String>,
    /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
    pub start_url: std::option::Option<std::string::String>,
}
impl StartDeviceAuthorizationInput {
    /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
    pub fn client_id(&self) -> std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
    pub fn client_secret(&self) -> std::option::Option<&str> {
        self.client_secret.as_deref()
    }
    /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
    pub fn start_url(&self) -> std::option::Option<&str> {
        self.start_url.as_deref()
    }
}
impl std::fmt::Debug for StartDeviceAuthorizationInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartDeviceAuthorizationInput");
        formatter.field("client_id", &self.client_id);
        formatter.field("client_secret", &self.client_secret);
        formatter.field("start_url", &self.start_url);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RegisterClientInput {
    /// <p>The friendly name of the client.</p>
    pub client_name: std::option::Option<std::string::String>,
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub client_type: std::option::Option<std::string::String>,
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub scopes: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl RegisterClientInput {
    /// <p>The friendly name of the client.</p>
    pub fn client_name(&self) -> std::option::Option<&str> {
        self.client_name.as_deref()
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn client_type(&self) -> std::option::Option<&str> {
        self.client_type.as_deref()
    }
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn scopes(&self) -> std::option::Option<&[std::string::String]> {
        self.scopes.as_deref()
    }
}
impl std::fmt::Debug for RegisterClientInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RegisterClientInput");
        formatter.field("client_name", &self.client_name);
        formatter.field("client_type", &self.client_type);
        formatter.field("scopes", &self.scopes);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateTokenInput {
    /// <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub client_id: std::option::Option<std::string::String>,
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub client_secret: std::option::Option<std::string::String>,
    /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
    pub grant_type: std::option::Option<std::string::String>,
    /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub device_code: std::option::Option<std::string::String>,
    /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
    pub code: std::option::Option<std::string::String>,
    /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
    pub refresh_token: std::option::Option<std::string::String>,
    /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub scope: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
    pub redirect_uri: std::option::Option<std::string::String>,
}
impl CreateTokenInput {
    /// <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn client_id(&self) -> std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn client_secret(&self) -> std::option::Option<&str> {
        self.client_secret.as_deref()
    }
    /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
    pub fn grant_type(&self) -> std::option::Option<&str> {
        self.grant_type.as_deref()
    }
    /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn device_code(&self) -> std::option::Option<&str> {
        self.device_code.as_deref()
    }
    /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
    pub fn refresh_token(&self) -> std::option::Option<&str> {
        self.refresh_token.as_deref()
    }
    /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn scope(&self) -> std::option::Option<&[std::string::String]> {
        self.scope.as_deref()
    }
    /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
    pub fn redirect_uri(&self) -> std::option::Option<&str> {
        self.redirect_uri.as_deref()
    }
}
impl std::fmt::Debug for CreateTokenInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateTokenInput");
        formatter.field("client_id", &self.client_id);
        formatter.field("client_secret", &self.client_secret);
        formatter.field("grant_type", &self.grant_type);
        formatter.field("device_code", &self.device_code);
        formatter.field("code", &self.code);
        formatter.field("refresh_token", &self.refresh_token);
        formatter.field("scope", &self.scope);
        formatter.field("redirect_uri", &self.redirect_uri);
        formatter.finish()
    }
}
