// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_db_instance_input_input(
    input: &crate::operation::modify_db_instance::ModifyDbInstanceInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyDBInstance", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBInstanceIdentifier");
    if let Some(var_2) = &input.db_instance_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AllocatedStorage");
    if let Some(var_4) = &input.allocated_storage {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DBInstanceClass");
    if let Some(var_6) = &input.db_instance_class {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DBSubnetGroupName");
    if let Some(var_8) = &input.db_subnet_group_name {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DBSecurityGroups");
    if let Some(var_10) = &input.db_security_groups {
        let mut list_12 = scope_9.start_list(false, Some("DBSecurityGroupName"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            entry_13.string(item_11);
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("VpcSecurityGroupIds");
    if let Some(var_15) = &input.vpc_security_group_ids {
        let mut list_17 = scope_14.start_list(false, Some("VpcSecurityGroupId"));
        for item_16 in var_15 {
            #[allow(unused_mut)]
            let mut entry_18 = list_17.entry();
            entry_18.string(item_16);
        }
        list_17.finish();
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("ApplyImmediately");
    if input.apply_immediately {
        scope_19.boolean(input.apply_immediately);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("MasterUserPassword");
    if let Some(var_21) = &input.master_user_password {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("DBParameterGroupName");
    if let Some(var_23) = &input.db_parameter_group_name {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("BackupRetentionPeriod");
    if let Some(var_25) = &input.backup_retention_period {
        scope_24.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_25).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("PreferredBackupWindow");
    if let Some(var_27) = &input.preferred_backup_window {
        scope_26.string(var_27);
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_29) = &input.preferred_maintenance_window {
        scope_28.string(var_29);
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("MultiAZ");
    if let Some(var_31) = &input.multi_az {
        scope_30.boolean(*var_31);
    }
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("EngineVersion");
    if let Some(var_33) = &input.engine_version {
        scope_32.string(var_33);
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("AllowMajorVersionUpgrade");
    if input.allow_major_version_upgrade {
        scope_34.boolean(input.allow_major_version_upgrade);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_36) = &input.auto_minor_version_upgrade {
        scope_35.boolean(*var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("LicenseModel");
    if let Some(var_38) = &input.license_model {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("Iops");
    if let Some(var_40) = &input.iops {
        scope_39.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_40).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("OptionGroupName");
    if let Some(var_42) = &input.option_group_name {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("NewDBInstanceIdentifier");
    if let Some(var_44) = &input.new_db_instance_identifier {
        scope_43.string(var_44);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("StorageType");
    if let Some(var_46) = &input.storage_type {
        scope_45.string(var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("TdeCredentialArn");
    if let Some(var_48) = &input.tde_credential_arn {
        scope_47.string(var_48);
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("TdeCredentialPassword");
    if let Some(var_50) = &input.tde_credential_password {
        scope_49.string(var_50);
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("CACertificateIdentifier");
    if let Some(var_52) = &input.ca_certificate_identifier {
        scope_51.string(var_52);
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("Domain");
    if let Some(var_54) = &input.domain {
        scope_53.string(var_54);
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("DomainFqdn");
    if let Some(var_56) = &input.domain_fqdn {
        scope_55.string(var_56);
    }
    #[allow(unused_mut)]
    let mut scope_57 = writer.prefix("DomainOu");
    if let Some(var_58) = &input.domain_ou {
        scope_57.string(var_58);
    }
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("DomainAuthSecretArn");
    if let Some(var_60) = &input.domain_auth_secret_arn {
        scope_59.string(var_60);
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("DomainDnsIps");
    if let Some(var_62) = &input.domain_dns_ips {
        let mut list_64 = scope_61.start_list(false, None);
        for item_63 in var_62 {
            #[allow(unused_mut)]
            let mut entry_65 = list_64.entry();
            entry_65.string(item_63);
        }
        list_64.finish();
    }
    #[allow(unused_mut)]
    let mut scope_66 = writer.prefix("CopyTagsToSnapshot");
    if let Some(var_67) = &input.copy_tags_to_snapshot {
        scope_66.boolean(*var_67);
    }
    #[allow(unused_mut)]
    let mut scope_68 = writer.prefix("MonitoringInterval");
    if let Some(var_69) = &input.monitoring_interval {
        scope_68.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_69).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_70 = writer.prefix("DBPortNumber");
    if let Some(var_71) = &input.db_port_number {
        scope_70.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_71).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_72 = writer.prefix("PubliclyAccessible");
    if let Some(var_73) = &input.publicly_accessible {
        scope_72.boolean(*var_73);
    }
    #[allow(unused_mut)]
    let mut scope_74 = writer.prefix("MonitoringRoleArn");
    if let Some(var_75) = &input.monitoring_role_arn {
        scope_74.string(var_75);
    }
    #[allow(unused_mut)]
    let mut scope_76 = writer.prefix("DomainIAMRoleName");
    if let Some(var_77) = &input.domain_iam_role_name {
        scope_76.string(var_77);
    }
    #[allow(unused_mut)]
    let mut scope_78 = writer.prefix("DisableDomain");
    if let Some(var_79) = &input.disable_domain {
        scope_78.boolean(*var_79);
    }
    #[allow(unused_mut)]
    let mut scope_80 = writer.prefix("PromotionTier");
    if let Some(var_81) = &input.promotion_tier {
        scope_80.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_81).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_82 = writer.prefix("EnableIAMDatabaseAuthentication");
    if let Some(var_83) = &input.enable_iam_database_authentication {
        scope_82.boolean(*var_83);
    }
    #[allow(unused_mut)]
    let mut scope_84 = writer.prefix("EnablePerformanceInsights");
    if let Some(var_85) = &input.enable_performance_insights {
        scope_84.boolean(*var_85);
    }
    #[allow(unused_mut)]
    let mut scope_86 = writer.prefix("PerformanceInsightsKMSKeyId");
    if let Some(var_87) = &input.performance_insights_kms_key_id {
        scope_86.string(var_87);
    }
    #[allow(unused_mut)]
    let mut scope_88 = writer.prefix("PerformanceInsightsRetentionPeriod");
    if let Some(var_89) = &input.performance_insights_retention_period {
        scope_88.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_89).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_90 = writer.prefix("CloudwatchLogsExportConfiguration");
    if let Some(var_91) = &input.cloudwatch_logs_export_configuration {
        crate::protocol_serde::shape_cloudwatch_logs_export_configuration::ser_cloudwatch_logs_export_configuration(scope_90, var_91)?;
    }
    #[allow(unused_mut)]
    let mut scope_92 = writer.prefix("ProcessorFeatures");
    if let Some(var_93) = &input.processor_features {
        let mut list_95 = scope_92.start_list(false, Some("ProcessorFeature"));
        for item_94 in var_93 {
            #[allow(unused_mut)]
            let mut entry_96 = list_95.entry();
            crate::protocol_serde::shape_processor_feature::ser_processor_feature(entry_96, item_94)?;
        }
        list_95.finish();
    }
    #[allow(unused_mut)]
    let mut scope_97 = writer.prefix("UseDefaultProcessorFeatures");
    if let Some(var_98) = &input.use_default_processor_features {
        scope_97.boolean(*var_98);
    }
    #[allow(unused_mut)]
    let mut scope_99 = writer.prefix("DeletionProtection");
    if let Some(var_100) = &input.deletion_protection {
        scope_99.boolean(*var_100);
    }
    #[allow(unused_mut)]
    let mut scope_101 = writer.prefix("MaxAllocatedStorage");
    if let Some(var_102) = &input.max_allocated_storage {
        scope_101.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_102).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_103 = writer.prefix("CertificateRotationRestart");
    if let Some(var_104) = &input.certificate_rotation_restart {
        scope_103.boolean(*var_104);
    }
    #[allow(unused_mut)]
    let mut scope_105 = writer.prefix("ReplicaMode");
    if let Some(var_106) = &input.replica_mode {
        scope_105.string(var_106.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_107 = writer.prefix("EnableCustomerOwnedIp");
    if let Some(var_108) = &input.enable_customer_owned_ip {
        scope_107.boolean(*var_108);
    }
    #[allow(unused_mut)]
    let mut scope_109 = writer.prefix("AwsBackupRecoveryPointArn");
    if let Some(var_110) = &input.aws_backup_recovery_point_arn {
        scope_109.string(var_110);
    }
    #[allow(unused_mut)]
    let mut scope_111 = writer.prefix("AutomationMode");
    if let Some(var_112) = &input.automation_mode {
        scope_111.string(var_112.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_113 = writer.prefix("ResumeFullAutomationModeMinutes");
    if let Some(var_114) = &input.resume_full_automation_mode_minutes {
        scope_113.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_114).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_115 = writer.prefix("NetworkType");
    if let Some(var_116) = &input.network_type {
        scope_115.string(var_116);
    }
    #[allow(unused_mut)]
    let mut scope_117 = writer.prefix("StorageThroughput");
    if let Some(var_118) = &input.storage_throughput {
        scope_117.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_118).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_119 = writer.prefix("ManageMasterUserPassword");
    if let Some(var_120) = &input.manage_master_user_password {
        scope_119.boolean(*var_120);
    }
    #[allow(unused_mut)]
    let mut scope_121 = writer.prefix("RotateMasterUserPassword");
    if let Some(var_122) = &input.rotate_master_user_password {
        scope_121.boolean(*var_122);
    }
    #[allow(unused_mut)]
    let mut scope_123 = writer.prefix("MasterUserSecretKmsKeyId");
    if let Some(var_124) = &input.master_user_secret_kms_key_id {
        scope_123.string(var_124);
    }
    #[allow(unused_mut)]
    let mut scope_125 = writer.prefix("Engine");
    if let Some(var_126) = &input.engine {
        scope_125.string(var_126);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
