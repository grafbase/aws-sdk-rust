// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ConnectAppAuthorization`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_bundle_identifier(impl ::std::convert::Into<String>)`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::app_bundle_identifier) / [`set_app_bundle_identifier(Option<String>)`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::set_app_bundle_identifier): <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app bundle that contains the app authorization to use for the request.</p>
    ///   - [`app_authorization_identifier(impl ::std::convert::Into<String>)`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::app_authorization_identifier) / [`set_app_authorization_identifier(Option<String>)`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::set_app_authorization_identifier): <p>The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.</p>
    ///   - [`auth_request(AuthRequest)`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::auth_request) / [`set_auth_request(Option<AuthRequest>)`](crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::set_auth_request): <p>Contains OAuth2 authorization information.</p>  <p>This is required if the app authorization for the request is configured with an OAuth2 (<code>oauth2</code>) authorization type.</p>
    /// - On success, responds with [`ConnectAppAuthorizationOutput`](crate::operation::connect_app_authorization::ConnectAppAuthorizationOutput) with field(s):
    ///   - [`app_authorization_summary(Option<AppAuthorizationSummary>)`](crate::operation::connect_app_authorization::ConnectAppAuthorizationOutput::app_authorization_summary): <p>Contains a summary of the app authorization.</p>
    /// - On failure, responds with [`SdkError<ConnectAppAuthorizationError>`](crate::operation::connect_app_authorization::ConnectAppAuthorizationError)
    pub fn connect_app_authorization(&self) -> crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder {
        crate::operation::connect_app_authorization::builders::ConnectAppAuthorizationFluentBuilder::new(self.handle.clone())
    }
}
