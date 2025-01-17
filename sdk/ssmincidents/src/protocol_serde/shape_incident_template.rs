// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_incident_template(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IncidentTemplate,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.title {
        object.key("title").string(var_1.as_str());
    }
    if let Some(var_2) = &input.impact {
        object.key("impact").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.summary {
        object.key("summary").string(var_3.as_str());
    }
    if let Some(var_4) = &input.dedupe_string {
        object.key("dedupeString").string(var_4.as_str());
    }
    if let Some(var_5) = &input.notification_targets {
        let mut array_6 = object.key("notificationTargets").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_notification_target_item::ser_notification_target_item(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.incident_tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("incidentTags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}

pub(crate) fn de_incident_template<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::IncidentTemplate>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::IncidentTemplateBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "title" => {
                            builder = builder.set_title(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "impact" => {
                            builder = builder.set_impact(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "summary" => {
                            builder = builder.set_summary(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "dedupeString" => {
                            builder = builder.set_dedupe_string(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "notificationTargets" => {
                            builder = builder
                                .set_notification_targets(crate::protocol_serde::shape_notification_target_set::de_notification_target_set(tokens)?);
                        }
                        "incidentTags" => {
                            builder = builder.set_incident_tags(crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?);
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
