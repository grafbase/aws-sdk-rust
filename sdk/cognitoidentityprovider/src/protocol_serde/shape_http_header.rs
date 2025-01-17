// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_http_header(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::HttpHeader,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.header_name {
        object.key("headerName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.header_value {
        object.key("headerValue").string(var_2.as_str());
    }
    Ok(())
}
