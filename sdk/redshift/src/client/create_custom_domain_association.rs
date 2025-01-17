// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCustomDomainAssociation`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_domain_name(impl ::std::convert::Into<String>)`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::custom_domain_name) / [`set_custom_domain_name(Option<String>)`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::set_custom_domain_name): <p>The custom domain name for a custom domain association.</p>
    ///   - [`custom_domain_certificate_arn(impl ::std::convert::Into<String>)`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::custom_domain_certificate_arn) / [`set_custom_domain_certificate_arn(Option<String>)`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::set_custom_domain_certificate_arn): <p>The certificate Amazon Resource Name (ARN) for the custom domain name association.</p>
    ///   - [`cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::set_cluster_identifier): <p>The cluster identifier that the custom domain is associated with.</p>
    /// - On success, responds with [`CreateCustomDomainAssociationOutput`](crate::operation::create_custom_domain_association::CreateCustomDomainAssociationOutput) with field(s):
    ///   - [`custom_domain_name(Option<String>)`](crate::operation::create_custom_domain_association::CreateCustomDomainAssociationOutput::custom_domain_name): <p>The custom domain name for the association result.</p>
    ///   - [`custom_domain_certificate_arn(Option<String>)`](crate::operation::create_custom_domain_association::CreateCustomDomainAssociationOutput::custom_domain_certificate_arn): <p>The Amazon Resource Name (ARN) for the certificate associated with the custom domain name.</p>
    ///   - [`cluster_identifier(Option<String>)`](crate::operation::create_custom_domain_association::CreateCustomDomainAssociationOutput::cluster_identifier): <p>The identifier of the cluster that the custom domain is associated with.</p>
    ///   - [`custom_domain_cert_expiry_time(Option<String>)`](crate::operation::create_custom_domain_association::CreateCustomDomainAssociationOutput::custom_domain_cert_expiry_time): <p>The expiration time for the certificate for the custom domain.</p>
    /// - On failure, responds with [`SdkError<CreateCustomDomainAssociationError>`](crate::operation::create_custom_domain_association::CreateCustomDomainAssociationError)
    pub fn create_custom_domain_association(
        &self,
    ) -> crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder {
        crate::operation::create_custom_domain_association::builders::CreateCustomDomainAssociationFluentBuilder::new(self.handle.clone())
    }
}
