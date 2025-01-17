// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_role_from_instance_profile_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileOutput,
    crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "LimitExceeded" => crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchEntity" => crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailure" => crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnmodifiableEntity" => {
            crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::UnmodifiableEntityException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnmodifiableEntityExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_unmodifiable_entity_exception::de_unmodifiable_entity_exception_xml_err(_response_body, output)
                            .map_err(crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_role_from_instance_profile_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileOutput,
    crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
