// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartStageDeploymentOutput {
    /// <p>Properties that describe the stage deployment.</p>
    pub stage_deployment: ::std::option::Option<crate::types::StageDeploymentDetails>,
    _request_id: Option<String>,
}
impl StartStageDeploymentOutput {
    /// <p>Properties that describe the stage deployment.</p>
    pub fn stage_deployment(&self) -> ::std::option::Option<&crate::types::StageDeploymentDetails> {
        self.stage_deployment.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for StartStageDeploymentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartStageDeploymentOutput {
    /// Creates a new builder-style object to manufacture [`StartStageDeploymentOutput`](crate::operation::start_stage_deployment::StartStageDeploymentOutput).
    pub fn builder() -> crate::operation::start_stage_deployment::builders::StartStageDeploymentOutputBuilder {
        crate::operation::start_stage_deployment::builders::StartStageDeploymentOutputBuilder::default()
    }
}

/// A builder for [`StartStageDeploymentOutput`](crate::operation::start_stage_deployment::StartStageDeploymentOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StartStageDeploymentOutputBuilder {
    pub(crate) stage_deployment: ::std::option::Option<crate::types::StageDeploymentDetails>,
    _request_id: Option<String>,
}
impl StartStageDeploymentOutputBuilder {
    /// <p>Properties that describe the stage deployment.</p>
    pub fn stage_deployment(mut self, input: crate::types::StageDeploymentDetails) -> Self {
        self.stage_deployment = ::std::option::Option::Some(input);
        self
    }
    /// <p>Properties that describe the stage deployment.</p>
    pub fn set_stage_deployment(mut self, input: ::std::option::Option<crate::types::StageDeploymentDetails>) -> Self {
        self.stage_deployment = input;
        self
    }
    /// <p>Properties that describe the stage deployment.</p>
    pub fn get_stage_deployment(&self) -> &::std::option::Option<crate::types::StageDeploymentDetails> {
        &self.stage_deployment
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartStageDeploymentOutput`](crate::operation::start_stage_deployment::StartStageDeploymentOutput).
    pub fn build(self) -> crate::operation::start_stage_deployment::StartStageDeploymentOutput {
        crate::operation::start_stage_deployment::StartStageDeploymentOutput {
            stage_deployment: self.stage_deployment,
            _request_id: self._request_id,
        }
    }
}
