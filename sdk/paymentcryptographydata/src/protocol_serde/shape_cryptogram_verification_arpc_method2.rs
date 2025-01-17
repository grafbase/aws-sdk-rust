// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cryptogram_verification_arpc_method2(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CryptogramVerificationArpcMethod2,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.card_status_update {
        object.key("CardStatusUpdate").string(var_1.as_str());
    }
    if let Some(var_2) = &input.proprietary_authentication_data {
        object.key("ProprietaryAuthenticationData").string(var_2.as_str());
    }
    Ok(())
}
