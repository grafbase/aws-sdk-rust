// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_invoke_endpoint(
    input: &crate::input::InvokeEndpointInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.content_type {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "content_type",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Content-Type", header_value);
        }
    }
    if let Some(inner_3) = &input.accept {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "accept",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Accept", header_value);
        }
    }
    if let Some(inner_5) = &input.custom_attributes {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "custom_attributes",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Custom-Attributes", header_value);
        }
    }
    if let Some(inner_7) = &input.target_model {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "target_model",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Target-Model", header_value);
        }
    }
    if let Some(inner_9) = &input.target_variant {
        let formatted_10 = AsRef::<str>::as_ref(inner_9);
        if !formatted_10.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "target_variant",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Target-Variant", header_value);
        }
    }
    if let Some(inner_11) = &input.target_container_hostname {
        let formatted_12 = AsRef::<str>::as_ref(inner_11);
        if !formatted_12.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_12;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "target_container_hostname",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Target-Container-Hostname", header_value);
        }
    }
    if let Some(inner_13) = &input.inference_id {
        let formatted_14 = AsRef::<str>::as_ref(inner_13);
        if !formatted_14.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_14;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "inference_id",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Inference-Id", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_invoke_endpoint_async(
    input: &crate::input::InvokeEndpointAsyncInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_15) = &input.content_type {
        let formatted_16 = AsRef::<str>::as_ref(inner_15);
        if !formatted_16.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_16;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "content_type",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Content-Type", header_value);
        }
    }
    if let Some(inner_17) = &input.accept {
        let formatted_18 = AsRef::<str>::as_ref(inner_17);
        if !formatted_18.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_18;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "accept",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Accept", header_value);
        }
    }
    if let Some(inner_19) = &input.custom_attributes {
        let formatted_20 = AsRef::<str>::as_ref(inner_19);
        if !formatted_20.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_20;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "custom_attributes",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Custom-Attributes", header_value);
        }
    }
    if let Some(inner_21) = &input.inference_id {
        let formatted_22 = AsRef::<str>::as_ref(inner_21);
        if !formatted_22.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_22;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "inference_id",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-Inference-Id", header_value);
        }
    }
    if let Some(inner_23) = &input.input_location {
        let formatted_24 = AsRef::<str>::as_ref(inner_23);
        if !formatted_24.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_24;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "input_location",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-InputLocation", header_value);
        }
    }
    if let Some(inner_25) = &input.request_ttl_seconds {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_25);
        let formatted_26 = encoder.encode();
        if !formatted_26.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_26;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "request_ttl_seconds",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("X-Amzn-SageMaker-RequestTTLSeconds", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_payload_invoke_endpoint_invoke_endpoint_output_body(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::InvokeEndpointError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_header_invoke_endpoint_invoke_endpoint_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_endpoint_invoke_endpoint_output_custom_attributes(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("X-Amzn-SageMaker-Custom-Attributes")
        .iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_endpoint_invoke_endpoint_output_invoked_production_variant(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-Amzn-Invoked-Production-Variant")
        .iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_invoke_endpoint_async_invoke_endpoint_async_output_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("X-Amzn-SageMaker-OutputLocation").iter();
    aws_smithy_http::header::one_or_none(headers)
}
