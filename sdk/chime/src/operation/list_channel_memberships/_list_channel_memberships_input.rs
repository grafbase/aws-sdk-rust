// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ListChannelMembershipsInput {
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned.</p>
    pub r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub chime_bearer: ::std::option::Option<::std::string::String>,
}
impl ListChannelMembershipsInput {
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ChannelMembershipType> {
        self.r#type.as_ref()
    }
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn chime_bearer(&self) -> ::std::option::Option<&str> {
        self.chime_bearer.as_deref()
    }
}
impl ::std::fmt::Debug for ListChannelMembershipsInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListChannelMembershipsInput");
        formatter.field("channel_arn", &self.channel_arn);
        formatter.field("r#type", &self.r#type);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("chime_bearer", &self.chime_bearer);
        formatter.finish()
    }
}
impl ListChannelMembershipsInput {
    /// Creates a new builder-style object to manufacture [`ListChannelMembershipsInput`](crate::operation::list_channel_memberships::ListChannelMembershipsInput).
    pub fn builder() -> crate::operation::list_channel_memberships::builders::ListChannelMembershipsInputBuilder {
        crate::operation::list_channel_memberships::builders::ListChannelMembershipsInputBuilder::default()
    }
}

/// A builder for [`ListChannelMembershipsInput`](crate::operation::list_channel_memberships::ListChannelMembershipsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ListChannelMembershipsInputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) chime_bearer: ::std::option::Option<::std::string::String>,
}
impl ListChannelMembershipsInputBuilder {
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_arn
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned.</p>
    pub fn r#type(mut self, input: crate::types::ChannelMembershipType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ChannelMembershipType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ChannelMembershipType> {
        &self.r#type
    }
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of channel memberships that you want returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.chime_bearer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.chime_bearer = input;
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        &self.chime_bearer
    }
    /// Consumes the builder and constructs a [`ListChannelMembershipsInput`](crate::operation::list_channel_memberships::ListChannelMembershipsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_channel_memberships::ListChannelMembershipsInput, ::aws_smithy_http::operation::error::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::list_channel_memberships::ListChannelMembershipsInput {
            channel_arn: self.channel_arn,
            r#type: self.r#type,
            max_results: self.max_results,
            next_token: self.next_token,
            chime_bearer: self.chime_bearer,
        })
    }
}
impl ::std::fmt::Debug for ListChannelMembershipsInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListChannelMembershipsInputBuilder");
        formatter.field("channel_arn", &self.channel_arn);
        formatter.field("r#type", &self.r#type);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("chime_bearer", &self.chime_bearer);
        formatter.finish()
    }
}
