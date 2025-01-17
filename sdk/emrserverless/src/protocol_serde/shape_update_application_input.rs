// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_application_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_application::UpdateApplicationInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.architecture {
        object.key("architecture").string(var_1.as_str());
    }
    if let Some(var_2) = &input.auto_start_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("autoStartConfiguration").start_object();
        crate::protocol_serde::shape_auto_start_config::ser_auto_start_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.auto_stop_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("autoStopConfiguration").start_object();
        crate::protocol_serde::shape_auto_stop_config::ser_auto_stop_config(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.client_token {
        object.key("clientToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.image_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("imageConfiguration").start_object();
        crate::protocol_serde::shape_image_configuration_input::ser_image_configuration_input(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.initial_capacity {
        #[allow(unused_mut)]
        let mut object_10 = object.key("initialCapacity").start_object();
        for (key_11, value_12) in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_13 = object_10.key(key_11.as_str()).start_object();
                crate::protocol_serde::shape_initial_capacity_config::ser_initial_capacity_config(&mut object_13, value_12)?;
                object_13.finish();
            }
        }
        object_10.finish();
    }
    if let Some(var_14) = &input.maximum_capacity {
        #[allow(unused_mut)]
        let mut object_15 = object.key("maximumCapacity").start_object();
        crate::protocol_serde::shape_maximum_allowed_resources::ser_maximum_allowed_resources(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_17 = object.key("networkConfiguration").start_object();
        crate::protocol_serde::shape_network_configuration::ser_network_configuration(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.release_label {
        object.key("releaseLabel").string(var_18.as_str());
    }
    if let Some(var_19) = &input.worker_type_specifications {
        #[allow(unused_mut)]
        let mut object_20 = object.key("workerTypeSpecifications").start_object();
        for (key_21, value_22) in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_23 = object_20.key(key_21.as_str()).start_object();
                crate::protocol_serde::shape_worker_type_specification_input::ser_worker_type_specification_input(&mut object_23, value_22)?;
                object_23.finish();
            }
        }
        object_20.finish();
    }
    Ok(())
}
