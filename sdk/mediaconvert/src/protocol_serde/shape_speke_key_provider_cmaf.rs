// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_speke_key_provider_cmaf(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SpekeKeyProviderCmaf,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_arn {
        object.key("certificateArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dash_signaled_system_ids {
        let mut array_3 = object.key("dashSignaledSystemIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.hls_signaled_system_ids {
        let mut array_6 = object.key("hlsSignaledSystemIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.resource_id {
        object.key("resourceId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.url {
        object.key("url").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_speke_key_provider_cmaf<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::SpekeKeyProviderCmaf>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SpekeKeyProviderCmafBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "certificateArn" => {
                            builder = builder.set_certificate_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "dashSignaledSystemIds" => {
                            builder = builder.set_dash_signaled_system_ids(
                                    crate::protocol_serde::shape___list_of__string_min36_max36_pattern09a_faf809a_faf409a_faf409a_faf409a_faf12::de___list_of__string_min36_max36_pattern09a_faf809a_faf409a_faf409a_faf409a_faf12(tokens)?
                                );
                        }
                        "hlsSignaledSystemIds" => {
                            builder = builder.set_hls_signaled_system_ids(
                                    crate::protocol_serde::shape___list_of__string_min36_max36_pattern09a_faf809a_faf409a_faf409a_faf409a_faf12::de___list_of__string_min36_max36_pattern09a_faf809a_faf409a_faf409a_faf409a_faf12(tokens)?
                                );
                        }
                        "resourceId" => {
                            builder = builder.set_resource_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "url" => {
                            builder = builder.set_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
