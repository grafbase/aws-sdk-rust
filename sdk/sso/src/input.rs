// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput).
pub mod get_role_credentials_input {

    /// A builder for [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) role_name: std::option::Option<std::string::String>,
        pub(crate) account_id: std::option::Option<std::string::String>,
        pub(crate) access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.role_name = Some(input.into());
            self
        }
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.role_name = input;
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput).
        pub fn build(
            self,
        ) -> Result<crate::input::GetRoleCredentialsInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::GetRoleCredentialsInput {
                role_name: self.role_name,
                account_id: self.account_id,
                access_token: self.access_token,
            })
        }
    }
}
impl GetRoleCredentialsInput {
    /// Consumes the builder and constructs an Operation<[`GetRoleCredentials`](crate::operation::GetRoleCredentials)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetRoleCredentials,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetRoleCredentialsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/federation/credentials").expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::GetRoleCredentialsInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_1) = &_input.role_name {
                    query.push_kv("role_name", &aws_smithy_http::query::fmt_string(&inner_1));
                }
                if let Some(inner_2) = &_input.account_id {
                    query.push_kv("account_id", &aws_smithy_http::query::fmt_string(&inner_2));
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetRoleCredentialsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder = crate::http_serde::add_headers_get_role_credentials(input, builder)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
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
            crate::operation::GetRoleCredentials::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetRoleCredentials",
            "sso",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput).
    pub fn builder() -> crate::input::get_role_credentials_input::Builder {
        crate::input::get_role_credentials_input::Builder::default()
    }
}

/// See [`ListAccountRolesInput`](crate::input::ListAccountRolesInput).
pub mod list_account_roles_input {

    /// A builder for [`ListAccountRolesInput`](crate::input::ListAccountRolesInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) access_token: std::option::Option<std::string::String>,
        pub(crate) account_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAccountRolesInput`](crate::input::ListAccountRolesInput).
        pub fn build(
            self,
        ) -> Result<crate::input::ListAccountRolesInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListAccountRolesInput {
                next_token: self.next_token,
                max_results: self.max_results,
                access_token: self.access_token,
                account_id: self.account_id,
            })
        }
    }
}
impl ListAccountRolesInput {
    /// Consumes the builder and constructs an Operation<[`ListAccountRoles`](crate::operation::ListAccountRoles)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::ListAccountRoles,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::ListAccountRolesInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/assignment/roles").expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::ListAccountRolesInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_3) = &_input.next_token {
                    query.push_kv("next_token", &aws_smithy_http::query::fmt_string(&inner_3));
                }
                if let Some(inner_4) = &_input.max_results {
                    query.push_kv(
                        "max_result",
                        aws_smithy_types::primitive::Encoder::from(*inner_4).encode(),
                    );
                }
                if let Some(inner_5) = &_input.account_id {
                    query.push_kv("account_id", &aws_smithy_http::query::fmt_string(&inner_5));
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::ListAccountRolesInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder = crate::http_serde::add_headers_list_account_roles(input, builder)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
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
            crate::operation::ListAccountRoles::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListAccountRoles",
            "sso",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`ListAccountRolesInput`](crate::input::ListAccountRolesInput).
    pub fn builder() -> crate::input::list_account_roles_input::Builder {
        crate::input::list_account_roles_input::Builder::default()
    }
}

/// See [`ListAccountsInput`](crate::input::ListAccountsInput).
pub mod list_accounts_input {

    /// A builder for [`ListAccountsInput`](crate::input::ListAccountsInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAccountsInput`](crate::input::ListAccountsInput).
        pub fn build(
            self,
        ) -> Result<crate::input::ListAccountsInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListAccountsInput {
                next_token: self.next_token,
                max_results: self.max_results,
                access_token: self.access_token,
            })
        }
    }
}
impl ListAccountsInput {
    /// Consumes the builder and constructs an Operation<[`ListAccounts`](crate::operation::ListAccounts)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::ListAccounts,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::ListAccountsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/assignment/accounts").expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::ListAccountsInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_6) = &_input.next_token {
                    query.push_kv("next_token", &aws_smithy_http::query::fmt_string(&inner_6));
                }
                if let Some(inner_7) = &_input.max_results {
                    query.push_kv(
                        "max_result",
                        aws_smithy_types::primitive::Encoder::from(*inner_7).encode(),
                    );
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::ListAccountsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder = crate::http_serde::add_headers_list_accounts(input, builder)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
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
            crate::operation::ListAccounts::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListAccounts",
            "sso",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`ListAccountsInput`](crate::input::ListAccountsInput).
    pub fn builder() -> crate::input::list_accounts_input::Builder {
        crate::input::list_accounts_input::Builder::default()
    }
}

/// See [`LogoutInput`](crate::input::LogoutInput).
pub mod logout_input {

    /// A builder for [`LogoutInput`](crate::input::LogoutInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`LogoutInput`](crate::input::LogoutInput).
        pub fn build(
            self,
        ) -> Result<crate::input::LogoutInput, aws_smithy_http::operation::BuildError> {
            Ok(crate::input::LogoutInput {
                access_token: self.access_token,
            })
        }
    }
}
impl LogoutInput {
    /// Consumes the builder and constructs an Operation<[`Logout`](crate::operation::Logout)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::Logout,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::LogoutInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/logout").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::LogoutInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                let builder = crate::http_serde::add_headers_logout(input, builder)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::Logout::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new("Logout", "sso"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`LogoutInput`](crate::input::LogoutInput).
    pub fn builder() -> crate::input::logout_input::Builder {
        crate::input::logout_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct LogoutInput {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    #[doc(hidden)]
    pub access_token: std::option::Option<std::string::String>,
}
impl LogoutInput {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn access_token(&self) -> std::option::Option<&str> {
        self.access_token.as_deref()
    }
}
impl std::fmt::Debug for LogoutInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LogoutInput");
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAccountsInput {
    /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>This is the number of items clients can request per page.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    #[doc(hidden)]
    pub access_token: std::option::Option<std::string::String>,
}
impl ListAccountsInput {
    /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>This is the number of items clients can request per page.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn access_token(&self) -> std::option::Option<&str> {
        self.access_token.as_deref()
    }
}
impl std::fmt::Debug for ListAccountsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAccountsInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAccountRolesInput {
    /// <p>The page token from the previous response output when you request subsequent pages.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The number of items that clients can request per page.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    #[doc(hidden)]
    pub access_token: std::option::Option<std::string::String>,
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
}
impl ListAccountRolesInput {
    /// <p>The page token from the previous response output when you request subsequent pages.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The number of items that clients can request per page.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn access_token(&self) -> std::option::Option<&str> {
        self.access_token.as_deref()
    }
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
}
impl std::fmt::Debug for ListAccountRolesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAccountRolesInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.field("account_id", &self.account_id);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRoleCredentialsInput {
    /// <p>The friendly name of the role that is assigned to the user.</p>
    #[doc(hidden)]
    pub role_name: std::option::Option<std::string::String>,
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    #[doc(hidden)]
    pub access_token: std::option::Option<std::string::String>,
}
impl GetRoleCredentialsInput {
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub fn role_name(&self) -> std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn access_token(&self) -> std::option::Option<&str> {
        self.access_token.as_deref()
    }
}
impl std::fmt::Debug for GetRoleCredentialsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoleCredentialsInput");
        formatter.field("role_name", &self.role_name);
        formatter.field("account_id", &self.account_id);
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
