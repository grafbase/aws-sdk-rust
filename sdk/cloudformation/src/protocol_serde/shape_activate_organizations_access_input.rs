// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_activate_organizations_access_input_input(
    input: &crate::operation::activate_organizations_access::ActivateOrganizationsAccessInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError> {
    let _ = input;
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ActivateOrganizationsAccess", "2010-05-15");
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
