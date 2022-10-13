// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_create_component_create_component_output_entity(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::Component>,
    crate::error::CreateComponentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_component_payload(body)
                .map_err(crate::error::CreateComponentError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_create_form_create_form_output_entity(
    body: &[u8],
) -> std::result::Result<std::option::Option<crate::model::Form>, crate::error::CreateFormError> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_form_payload(body)
                .map_err(crate::error::CreateFormError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_create_theme_create_theme_output_entity(
    body: &[u8],
) -> std::result::Result<std::option::Option<crate::model::Theme>, crate::error::CreateThemeError> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_theme_payload(body)
                .map_err(crate::error::CreateThemeError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_get_component_get_component_output_component(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::Component>,
    crate::error::GetComponentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_component_payload(body)
                .map_err(crate::error::GetComponentError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_get_form_get_form_output_form(
    body: &[u8],
) -> std::result::Result<std::option::Option<crate::model::Form>, crate::error::GetFormError> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_form_payload(body)
                .map_err(crate::error::GetFormError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_get_theme_get_theme_output_theme(
    body: &[u8],
) -> std::result::Result<std::option::Option<crate::model::Theme>, crate::error::GetThemeError> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_theme_payload(body)
                .map_err(crate::error::GetThemeError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_update_component_update_component_output_entity(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::Component>,
    crate::error::UpdateComponentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_component_payload(body)
                .map_err(crate::error::UpdateComponentError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_update_form_update_form_output_entity(
    body: &[u8],
) -> std::result::Result<std::option::Option<crate::model::Form>, crate::error::UpdateFormError> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_form_payload(body)
                .map_err(crate::error::UpdateFormError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_update_theme_update_theme_output_entity(
    body: &[u8],
) -> std::result::Result<std::option::Option<crate::model::Theme>, crate::error::UpdateThemeError> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_theme_payload(body)
                .map_err(crate::error::UpdateThemeError::unhandled)
        })
        .transpose()
}
