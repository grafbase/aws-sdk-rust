// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_configure_logs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ConfigureLogsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.egress_access_logs {
        let mut object_2 = object.key("egressAccessLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_egress_access_logs(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.ingress_access_logs {
        let mut object_4 = object.key("ingressAccessLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_ingress_access_logs(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.id {
        object.key("id").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        let mut object_8 = object.key("tags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_harvest_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHarvestJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.end_time {
        object.key("endTime").string(var_11.as_str());
    }
    if let Some(var_12) = &input.id {
        object.key("id").string(var_12.as_str());
    }
    if let Some(var_13) = &input.origin_endpoint_id {
        object.key("originEndpointId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.s3_destination {
        let mut object_15 = object.key("s3Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_destination(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.start_time {
        object.key("startTime").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_origin_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateOriginEndpointInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.authorization {
        let mut object_18 = object.key("authorization").start_object();
        crate::json_ser::serialize_structure_crate_model_authorization(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.channel_id {
        object.key("channelId").string(var_19.as_str());
    }
    if let Some(var_20) = &input.cmaf_package {
        let mut object_21 = object.key("cmafPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_cmaf_package_create_or_update_parameters(
            &mut object_21,
            var_20,
        )?;
        object_21.finish();
    }
    if let Some(var_22) = &input.dash_package {
        let mut object_23 = object.key("dashPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_package(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.description {
        object.key("description").string(var_24.as_str());
    }
    if let Some(var_25) = &input.hls_package {
        let mut object_26 = object.key("hlsPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_package(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.id {
        object.key("id").string(var_27.as_str());
    }
    if let Some(var_28) = &input.manifest_name {
        object.key("manifestName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.mss_package {
        let mut object_30 = object.key("mssPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_mss_package(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.origination {
        object.key("origination").string(var_31.as_str());
    }
    if input.startover_window_seconds != 0 {
        object.key("startoverWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.startover_window_seconds).into()),
        );
    }
    if let Some(var_32) = &input.tags {
        let mut object_33 = object.key("tags").start_object();
        for (key_34, value_35) in var_32 {
            {
                object_33.key(key_34).string(value_35.as_str());
            }
        }
        object_33.finish();
    }
    if input.time_delay_seconds != 0 {
        object.key("timeDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.time_delay_seconds).into()),
        );
    }
    if let Some(var_36) = &input.whitelist {
        let mut array_37 = object.key("whitelist").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.tags {
        let mut object_40 = object.key("tags").start_object();
        for (key_41, value_42) in var_39 {
            {
                object_40.key(key_41).string(value_42.as_str());
            }
        }
        object_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.description {
        object.key("description").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_origin_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateOriginEndpointInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.authorization {
        let mut object_45 = object.key("authorization").start_object();
        crate::json_ser::serialize_structure_crate_model_authorization(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.cmaf_package {
        let mut object_47 = object.key("cmafPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_cmaf_package_create_or_update_parameters(
            &mut object_47,
            var_46,
        )?;
        object_47.finish();
    }
    if let Some(var_48) = &input.dash_package {
        let mut object_49 = object.key("dashPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_package(&mut object_49, var_48)?;
        object_49.finish();
    }
    if let Some(var_50) = &input.description {
        object.key("description").string(var_50.as_str());
    }
    if let Some(var_51) = &input.hls_package {
        let mut object_52 = object.key("hlsPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_package(&mut object_52, var_51)?;
        object_52.finish();
    }
    if let Some(var_53) = &input.manifest_name {
        object.key("manifestName").string(var_53.as_str());
    }
    if let Some(var_54) = &input.mss_package {
        let mut object_55 = object.key("mssPackage").start_object();
        crate::json_ser::serialize_structure_crate_model_mss_package(&mut object_55, var_54)?;
        object_55.finish();
    }
    if let Some(var_56) = &input.origination {
        object.key("origination").string(var_56.as_str());
    }
    if input.startover_window_seconds != 0 {
        object.key("startoverWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.startover_window_seconds).into()),
        );
    }
    if input.time_delay_seconds != 0 {
        object.key("timeDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.time_delay_seconds).into()),
        );
    }
    if let Some(var_57) = &input.whitelist {
        let mut array_58 = object.key("whitelist").start_array();
        for item_59 in var_57 {
            {
                array_58.value().string(item_59.as_str());
            }
        }
        array_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_egress_access_logs(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EgressAccessLogs,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.log_group_name {
        object.key("logGroupName").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ingress_access_logs(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IngressAccessLogs,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.log_group_name {
        object.key("logGroupName").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_destination(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Destination,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.bucket_name {
        object.key("bucketName").string(var_62.as_str());
    }
    if let Some(var_63) = &input.manifest_key {
        object.key("manifestKey").string(var_63.as_str());
    }
    if let Some(var_64) = &input.role_arn {
        object.key("roleArn").string(var_64.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_authorization(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Authorization,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.cdn_identifier_secret {
        object.key("cdnIdentifierSecret").string(var_65.as_str());
    }
    if let Some(var_66) = &input.secrets_role_arn {
        object.key("secretsRoleArn").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cmaf_package_create_or_update_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CmafPackageCreateOrUpdateParameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.encryption {
        let mut object_68 = object.key("encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_cmaf_encryption(&mut object_68, var_67)?;
        object_68.finish();
    }
    if let Some(var_69) = &input.hls_manifests {
        let mut array_70 = object.key("hlsManifests").start_array();
        for item_71 in var_69 {
            {
                let mut object_72 = array_70.value().start_object();
                crate::json_ser::serialize_structure_crate_model_hls_manifest_create_or_update_parameters(&mut object_72, item_71)?;
                object_72.finish();
            }
        }
        array_70.finish();
    }
    if input.segment_duration_seconds != 0 {
        object.key("segmentDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.segment_duration_seconds).into()),
        );
    }
    if let Some(var_73) = &input.segment_prefix {
        object.key("segmentPrefix").string(var_73.as_str());
    }
    if let Some(var_74) = &input.stream_selection {
        let mut object_75 = object.key("streamSelection").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_selection(&mut object_75, var_74)?;
        object_75.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_package(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashPackage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.ad_triggers {
        let mut array_77 = object.key("adTriggers").start_array();
        for item_78 in var_76 {
            {
                array_77.value().string(item_78.as_str());
            }
        }
        array_77.finish();
    }
    if let Some(var_79) = &input.ads_on_delivery_restrictions {
        object
            .key("adsOnDeliveryRestrictions")
            .string(var_79.as_str());
    }
    if let Some(var_80) = &input.encryption {
        let mut object_81 = object.key("encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_encryption(&mut object_81, var_80)?;
        object_81.finish();
    }
    if input.include_iframe_only_stream {
        object
            .key("includeIframeOnlyStream")
            .boolean(input.include_iframe_only_stream);
    }
    if let Some(var_82) = &input.manifest_layout {
        object.key("manifestLayout").string(var_82.as_str());
    }
    if input.manifest_window_seconds != 0 {
        object.key("manifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    if input.min_buffer_time_seconds != 0 {
        object.key("minBufferTimeSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_buffer_time_seconds).into()),
        );
    }
    if input.min_update_period_seconds != 0 {
        object.key("minUpdatePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_update_period_seconds).into()),
        );
    }
    if let Some(var_83) = &input.period_triggers {
        let mut array_84 = object.key("periodTriggers").start_array();
        for item_85 in var_83 {
            {
                array_84.value().string(item_85.as_str());
            }
        }
        array_84.finish();
    }
    if let Some(var_86) = &input.profile {
        object.key("profile").string(var_86.as_str());
    }
    if input.segment_duration_seconds != 0 {
        object.key("segmentDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.segment_duration_seconds).into()),
        );
    }
    if let Some(var_87) = &input.segment_template_format {
        object.key("segmentTemplateFormat").string(var_87.as_str());
    }
    if let Some(var_88) = &input.stream_selection {
        let mut object_89 = object.key("streamSelection").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_selection(&mut object_89, var_88)?;
        object_89.finish();
    }
    if input.suggested_presentation_delay_seconds != 0 {
        object.key("suggestedPresentationDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.suggested_presentation_delay_seconds).into()),
        );
    }
    if let Some(var_90) = &input.utc_timing {
        object.key("utcTiming").string(var_90.as_str());
    }
    if let Some(var_91) = &input.utc_timing_uri {
        object.key("utcTimingUri").string(var_91.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_package(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsPackage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.ad_markers {
        object.key("adMarkers").string(var_92.as_str());
    }
    if let Some(var_93) = &input.ad_triggers {
        let mut array_94 = object.key("adTriggers").start_array();
        for item_95 in var_93 {
            {
                array_94.value().string(item_95.as_str());
            }
        }
        array_94.finish();
    }
    if let Some(var_96) = &input.ads_on_delivery_restrictions {
        object
            .key("adsOnDeliveryRestrictions")
            .string(var_96.as_str());
    }
    if let Some(var_97) = &input.encryption {
        let mut object_98 = object.key("encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_encryption(&mut object_98, var_97)?;
        object_98.finish();
    }
    if input.include_dvb_subtitles {
        object
            .key("includeDvbSubtitles")
            .boolean(input.include_dvb_subtitles);
    }
    if input.include_iframe_only_stream {
        object
            .key("includeIframeOnlyStream")
            .boolean(input.include_iframe_only_stream);
    }
    if let Some(var_99) = &input.playlist_type {
        object.key("playlistType").string(var_99.as_str());
    }
    if input.playlist_window_seconds != 0 {
        object.key("playlistWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.playlist_window_seconds).into()),
        );
    }
    if input.program_date_time_interval_seconds != 0 {
        object.key("programDateTimeIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.program_date_time_interval_seconds).into()),
        );
    }
    if input.segment_duration_seconds != 0 {
        object.key("segmentDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.segment_duration_seconds).into()),
        );
    }
    if let Some(var_100) = &input.stream_selection {
        let mut object_101 = object.key("streamSelection").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_selection(
            &mut object_101,
            var_100,
        )?;
        object_101.finish();
    }
    if input.use_audio_rendition_group {
        object
            .key("useAudioRenditionGroup")
            .boolean(input.use_audio_rendition_group);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_mss_package(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MssPackage,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.encryption {
        let mut object_103 = object.key("encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_mss_encryption(&mut object_103, var_102)?;
        object_103.finish();
    }
    if input.manifest_window_seconds != 0 {
        object.key("manifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    if input.segment_duration_seconds != 0 {
        object.key("segmentDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.segment_duration_seconds).into()),
        );
    }
    if let Some(var_104) = &input.stream_selection {
        let mut object_105 = object.key("streamSelection").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_selection(
            &mut object_105,
            var_104,
        )?;
        object_105.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cmaf_encryption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CmafEncryption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.constant_initialization_vector {
        object
            .key("constantInitializationVector")
            .string(var_106.as_str());
    }
    if let Some(var_107) = &input.encryption_method {
        object.key("encryptionMethod").string(var_107.as_str());
    }
    if input.key_rotation_interval_seconds != 0 {
        object.key("keyRotationIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.key_rotation_interval_seconds).into()),
        );
    }
    if let Some(var_108) = &input.speke_key_provider {
        let mut object_109 = object.key("spekeKeyProvider").start_object();
        crate::json_ser::serialize_structure_crate_model_speke_key_provider(
            &mut object_109,
            var_108,
        )?;
        object_109.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_manifest_create_or_update_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsManifestCreateOrUpdateParameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.ad_markers {
        object.key("adMarkers").string(var_110.as_str());
    }
    if let Some(var_111) = &input.ad_triggers {
        let mut array_112 = object.key("adTriggers").start_array();
        for item_113 in var_111 {
            {
                array_112.value().string(item_113.as_str());
            }
        }
        array_112.finish();
    }
    if let Some(var_114) = &input.ads_on_delivery_restrictions {
        object
            .key("adsOnDeliveryRestrictions")
            .string(var_114.as_str());
    }
    if let Some(var_115) = &input.id {
        object.key("id").string(var_115.as_str());
    }
    if input.include_iframe_only_stream {
        object
            .key("includeIframeOnlyStream")
            .boolean(input.include_iframe_only_stream);
    }
    if let Some(var_116) = &input.manifest_name {
        object.key("manifestName").string(var_116.as_str());
    }
    if let Some(var_117) = &input.playlist_type {
        object.key("playlistType").string(var_117.as_str());
    }
    if input.playlist_window_seconds != 0 {
        object.key("playlistWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.playlist_window_seconds).into()),
        );
    }
    if input.program_date_time_interval_seconds != 0 {
        object.key("programDateTimeIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.program_date_time_interval_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_selection(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamSelection,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_video_bits_per_second != 0 {
        object.key("maxVideoBitsPerSecond").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_video_bits_per_second).into()),
        );
    }
    if input.min_video_bits_per_second != 0 {
        object.key("minVideoBitsPerSecond").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_video_bits_per_second).into()),
        );
    }
    if let Some(var_118) = &input.stream_order {
        object.key("streamOrder").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_encryption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashEncryption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.key_rotation_interval_seconds != 0 {
        object.key("keyRotationIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.key_rotation_interval_seconds).into()),
        );
    }
    if let Some(var_119) = &input.speke_key_provider {
        let mut object_120 = object.key("spekeKeyProvider").start_object();
        crate::json_ser::serialize_structure_crate_model_speke_key_provider(
            &mut object_120,
            var_119,
        )?;
        object_120.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_encryption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsEncryption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.constant_initialization_vector {
        object
            .key("constantInitializationVector")
            .string(var_121.as_str());
    }
    if let Some(var_122) = &input.encryption_method {
        object.key("encryptionMethod").string(var_122.as_str());
    }
    if input.key_rotation_interval_seconds != 0 {
        object.key("keyRotationIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.key_rotation_interval_seconds).into()),
        );
    }
    if input.repeat_ext_x_key {
        object.key("repeatExtXKey").boolean(input.repeat_ext_x_key);
    }
    if let Some(var_123) = &input.speke_key_provider {
        let mut object_124 = object.key("spekeKeyProvider").start_object();
        crate::json_ser::serialize_structure_crate_model_speke_key_provider(
            &mut object_124,
            var_123,
        )?;
        object_124.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_mss_encryption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MssEncryption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.speke_key_provider {
        let mut object_126 = object.key("spekeKeyProvider").start_object();
        crate::json_ser::serialize_structure_crate_model_speke_key_provider(
            &mut object_126,
            var_125,
        )?;
        object_126.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_speke_key_provider(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SpekeKeyProvider,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.certificate_arn {
        object.key("certificateArn").string(var_127.as_str());
    }
    if let Some(var_128) = &input.encryption_contract_configuration {
        let mut object_129 = object.key("encryptionContractConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_contract_configuration(
            &mut object_129,
            var_128,
        )?;
        object_129.finish();
    }
    if let Some(var_130) = &input.resource_id {
        object.key("resourceId").string(var_130.as_str());
    }
    if let Some(var_131) = &input.role_arn {
        object.key("roleArn").string(var_131.as_str());
    }
    if let Some(var_132) = &input.system_ids {
        let mut array_133 = object.key("systemIds").start_array();
        for item_134 in var_132 {
            {
                array_133.value().string(item_134.as_str());
            }
        }
        array_133.finish();
    }
    if let Some(var_135) = &input.url {
        object.key("url").string(var_135.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_contract_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionContractConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_136) = &input.preset_speke20_audio {
        object.key("presetSpeke20Audio").string(var_136.as_str());
    }
    if let Some(var_137) = &input.preset_speke20_video {
        object.key("presetSpeke20Video").string(var_137.as_str());
    }
    Ok(())
}
