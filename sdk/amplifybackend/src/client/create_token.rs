// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateToken`](crate::operation::create_token::builders::CreateTokenFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::create_token::builders::CreateTokenFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::create_token::builders::CreateTokenFluentBuilder::set_app_id): <p>The app ID.</p>
    /// - On success, responds with [`CreateTokenOutput`](crate::operation::create_token::CreateTokenOutput) with field(s):
    ///   - [`app_id(Option<String>)`](crate::operation::create_token::CreateTokenOutput::app_id): <p>The app ID.</p>
    ///   - [`challenge_code(Option<String>)`](crate::operation::create_token::CreateTokenOutput::challenge_code): <p>One-time challenge code for authenticating into the Amplify Admin UI.</p>
    ///   - [`session_id(Option<String>)`](crate::operation::create_token::CreateTokenOutput::session_id): <p>A unique ID provided when creating a new challenge token.</p>
    ///   - [`ttl(Option<String>)`](crate::operation::create_token::CreateTokenOutput::ttl): <p>The expiry time for the one-time generated token code.</p>
    /// - On failure, responds with [`SdkError<CreateTokenError>`](crate::operation::create_token::CreateTokenError)
    pub fn create_token(&self) -> crate::operation::create_token::builders::CreateTokenFluentBuilder {
        crate::operation::create_token::builders::CreateTokenFluentBuilder::new(self.handle.clone())
    }
}
