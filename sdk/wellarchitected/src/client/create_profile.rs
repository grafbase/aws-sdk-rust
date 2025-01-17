// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateProfile`](crate::operation::create_profile::builders::CreateProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`profile_name(impl ::std::convert::Into<String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::profile_name) / [`set_profile_name(Option<String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::set_profile_name): <p>Name of the profile.</p>
    ///   - [`profile_description(impl ::std::convert::Into<String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::profile_description) / [`set_profile_description(Option<String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::set_profile_description): <p>The profile description.</p>
    ///   - [`profile_questions(Vec<ProfileQuestionUpdate>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::profile_questions) / [`set_profile_questions(Option<Vec<ProfileQuestionUpdate>>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::set_profile_questions): <p>The profile questions.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::set_client_request_token): <p>A unique case-sensitive string used to ensure that this request is idempotent (executes only once).</p>  <p>You should not reuse the same token for other requests. If you retry a request with the same client request token and the same parameters after the original request has completed successfully, the result of the original request is returned.</p> <important>   <p>This token is listed as required, however, if you do not specify it, the Amazon Web Services SDKs automatically generate one for you. If you are not using the Amazon Web Services SDK or the CLI, you must provide this token or the request will fail.</p>  </important>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_profile::builders::CreateProfileFluentBuilder::set_tags): <p>The tags assigned to the profile.</p>
    /// - On success, responds with [`CreateProfileOutput`](crate::operation::create_profile::CreateProfileOutput) with field(s):
    ///   - [`profile_arn(Option<String>)`](crate::operation::create_profile::CreateProfileOutput::profile_arn): <p>The profile ARN.</p>
    ///   - [`profile_version(Option<String>)`](crate::operation::create_profile::CreateProfileOutput::profile_version): <p>Version of the profile.</p>
    /// - On failure, responds with [`SdkError<CreateProfileError>`](crate::operation::create_profile::CreateProfileError)
    pub fn create_profile(&self) -> crate::operation::create_profile::builders::CreateProfileFluentBuilder {
        crate::operation::create_profile::builders::CreateProfileFluentBuilder::new(self.handle.clone())
    }
}
